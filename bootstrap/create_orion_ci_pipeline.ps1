# create_orion_ci_pipeline.ps1
# Génère un pipeline CI/CD GitHub Actions pour ORION

$dir = "./.github/workflows"
New-Item -ItemType Directory -Force -Path $dir | Out-Null

$file = "$dir/orion-ci.yml"

$content = @"
name: ORION CI Pipeline

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]

jobs:

  build:
    name: Build ORION Workspace
    runs-on: ubuntu-latest

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Cache Cargo
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: \${{ runner.os }}-cargo-\${{ hashFiles('**/Cargo.lock') }}

      - name: Build workspace
        run: cargo build --workspace --release

      - name: Run tests
        run: cargo test --workspace

      - name: Check formatting
        run: cargo fmt --all -- --check

      - name: Clippy (lint)
        run: cargo clippy --workspace -- -D warnings

      - name: Security audit
        run: cargo install cargo-audit && cargo audit

      - name: Upload build artifacts
        uses: actions/upload-artifact@v4
        with:
          name: orion-binaries
          path: target/release/
"@

Set-Content -Path $file -Value $content

Write-Host "Pipeline CI/CD ORION généré avec succès."
