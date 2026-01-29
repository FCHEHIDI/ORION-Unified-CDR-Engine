#!/bin/bash
# Ceph demo script - Essential commands for ORION integration
# Run after deploy-ceph-docker.sh

set -e

CONTAINER="ceph-mon1"
BUCKET="orion-cdr-archive"
USER="orion-storage-cold"

echo "🎯 Ceph Demo - Essential Commands"
echo "=================================="

# Function to run ceph commands
ceph_exec() {
    docker exec $CONTAINER "$@"
}

echo ""
echo "1️⃣  Cluster Status"
echo "-------------------"
ceph_exec ceph -s
echo ""

echo "2️⃣  OSD Tree (Disk Topology)"
echo "-----------------------------"
ceph_exec ceph osd tree
echo ""

echo "3️⃣  Cluster Capacity"
echo "---------------------"
ceph_exec ceph df
echo ""

echo "4️⃣  Create ORION Pool"
echo "----------------------"
if ceph_exec ceph osd lspools | grep -q "$BUCKET"; then
    echo "Pool $BUCKET already exists"
else
    ceph_exec ceph osd pool create $BUCKET 128
    ceph_exec ceph osd pool set $BUCKET size 3
    ceph_exec ceph osd pool set $BUCKET min_size 2
    echo "✅ Pool $BUCKET created (128 PGs, 3 replicas)"
fi
echo ""

echo "5️⃣  Create RGW User for ORION"
echo "-------------------------------"
if ceph_exec radosgw-admin user info --uid=$USER 2>/dev/null; then
    echo "User $USER already exists"
else
    ceph_exec radosgw-admin user create \
        --uid=$USER \
        --display-name="ORION Cold Storage Service" \
        --email=orion@example.com
    echo "✅ User created"
fi

echo ""
echo "📋 User Credentials:"
ceph_exec radosgw-admin user info --uid=$USER | grep -E '"access_key"|"secret_key"'
echo ""

echo "6️⃣  Create S3 Bucket"
echo "---------------------"
# Extract credentials
ACCESS_KEY=$(ceph_exec radosgw-admin user info --uid=$USER | grep -oP '"access_key": "\K[^"]+' | head -1)
SECRET_KEY=$(ceph_exec radosgw-admin user info --uid=$USER | grep -oP '"secret_key": "\K[^"]+' | head -1)

echo "Using access_key: $ACCESS_KEY"

# Test S3 endpoint with AWS CLI (if installed)
if command -v aws &> /dev/null; then
    export AWS_ACCESS_KEY_ID=$ACCESS_KEY
    export AWS_SECRET_ACCESS_KEY=$SECRET_KEY
    
    aws s3 mb s3://$BUCKET \
        --endpoint-url http://localhost:7480 \
        --region default 2>/dev/null || echo "Bucket may already exist"
    
    echo "✅ Bucket created/verified"
    
    echo ""
    echo "7️⃣  Test S3 Operations"
    echo "-----------------------"
    
    # Upload test file
    echo "ORION CDR Archive Test" > /tmp/test-orion.txt
    aws s3 cp /tmp/test-orion.txt s3://$BUCKET/test/ \
        --endpoint-url http://localhost:7480 \
        --region default
    
    # List objects
    aws s3 ls s3://$BUCKET/test/ \
        --endpoint-url http://localhost:7480 \
        --region default
    
    echo "✅ S3 operations working"
    rm /tmp/test-orion.txt
else
    echo "⚠️  AWS CLI not installed, skipping S3 tests"
    echo "   Install: pip install awscli"
fi
echo ""

echo "8️⃣  Monitor Performance"
echo "------------------------"
ceph_exec ceph osd perf
echo ""

echo "9️⃣  Health Details"
echo "-------------------"
ceph_exec ceph health detail
echo ""

echo "🔟  Placement Groups (PGs)"
echo "--------------------------"
ceph_exec ceph pg stat
echo ""

echo "✅ Demo Complete!"
echo ""
echo "📝 Configuration for orion-storage-cold:"
echo "========================================="
cat <<EOF
S3_ENDPOINT=http://localhost:7480
S3_REGION=default
S3_BUCKET=$BUCKET
S3_ACCESS_KEY=$ACCESS_KEY
S3_SECRET_KEY=$SECRET_KEY
S3_PATH_STYLE=true
EOF
echo ""

echo "📊 Useful commands:"
echo "  - Cluster status: docker exec $CONTAINER ceph -s"
echo "  - Watch health: docker exec $CONTAINER ceph -w"
echo "  - Pool stats: docker exec $CONTAINER rados df"
echo "  - RGW logs: docker logs ceph-rgw -f"
echo ""

echo "🌐 Access points:"
echo "  - Dashboard: http://localhost:7000 (enable with: ceph mgr module enable dashboard)"
echo "  - Prometheus: http://localhost:9283/metrics"
echo "  - S3 API: http://localhost:7480"
