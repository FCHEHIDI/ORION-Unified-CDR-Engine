use anyhow::{Context, Result};
use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::config::{Credentials, Region};
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tracing::info;

use crate::config::S3Config;

pub struct S3Uploader {
    client: S3Client,
    bucket: String,
}

impl S3Uploader {
    pub async fn new(config: &S3Config) -> Result<Self> {
        let creds = Credentials::new(
            &config.access_key,
            &config.secret_key,
            None,
            None,
            "static",
        );

        let s3_config = aws_sdk_s3::Config::builder()
            .region(Region::new(config.region.clone()))
            .endpoint_url(&config.endpoint)
            .credentials_provider(creds)
            .force_path_style(config.path_style) // Critical for MinIO/Ceph RGW
            .build();

        let client = S3Client::from_conf(s3_config);

        // Ensure bucket exists
        match client.head_bucket()
            .bucket(&config.bucket)
            .send()
            .await
        {
            Ok(_) => info!("Bucket '{}' exists", config.bucket),
            Err(_) => {
                info!("Creating bucket '{}'", config.bucket);
                client.create_bucket()
                    .bucket(&config.bucket)
                    .send()
                    .await
                    .context("Failed to create bucket")?;
            }
        }

        Ok(Self {
            client,
            bucket: config.bucket.clone(),
        })
    }

    /// Upload Parquet file to S3-compatible storage (MinIO/Ceph)
    pub async fn upload_file(&self, local_path: &Path) -> Result<String> {
        let mut file = File::open(local_path)
            .await
            .context("Failed to open file")?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .await
            .context("Failed to read file")?;

        // Preserve partition structure in S3 key
        let key = local_path
            .file_name()
            .and_then(|n| n.to_str())
            .context("Invalid filename")?;

        let partition_prefix = local_path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("");

        let s3_key = format!("{}/{}", partition_prefix, key);

        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&s3_key)
            .body(buffer.into())
            .content_type("application/octet-stream")
            .send()
            .await
            .context("Failed to upload to S3")?;

        info!("Uploaded {} to s3://{}/{}", local_path.display(), self.bucket, s3_key);
        Ok(s3_key)
    }

    /// Delete local file after successful upload
    pub async fn cleanup_local(&self, path: &Path) -> Result<()> {
        tokio::fs::remove_file(path)
            .await
            .context("Failed to delete local file")?;
        info!("Cleaned up local file: {}", path.display());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s3_key_generation() {
        let path = Path::new("/tmp/year=2024/month=01/day=15/country=US/cdr_123.parquet");
        let key = path.file_name().unwrap().to_str().unwrap();
        assert_eq!(key, "cdr_123.parquet");

        let partition = path.parent().unwrap().file_name().unwrap().to_str().unwrap();
        assert_eq!(partition, "country=US");
    }
}
