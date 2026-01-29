#!/bin/bash
# Deploy Ceph cluster with Docker Compose for ORION development/testing
# Production: Use cephadm on RHEL/CentOS

set -e

CEPH_VERSION="v18.2.0"  # Reef (latest stable)
CLUSTER_NETWORK="172.20.0.0/16"

echo "🚀 Deploying Ceph cluster (Docker)"
echo "Version: $CEPH_VERSION"
echo "Network: $CLUSTER_NETWORK"

# Create directories for OSD data
mkdir -p ceph-data/{osd1,osd2,osd3}
mkdir -p ceph-data/mon{1,2,3}
mkdir -p ceph-data/mgr
mkdir -p ceph-data/rgw

# Generate Ceph configuration
cat > ceph-data/ceph.conf <<EOF
[global]
fsid = $(uuidgen)
mon_initial_members = mon1,mon2,mon3
mon_host = 172.20.0.11,172.20.0.12,172.20.0.13
public_network = $CLUSTER_NETWORK
cluster_network = $CLUSTER_NETWORK
auth_cluster_required = cephx
auth_service_required = cephx
auth_client_required = cephx
osd_pool_default_size = 3
osd_pool_default_min_size = 2
osd_pool_default_pg_num = 128
osd_pool_default_pgp_num = 128

[mon]
mon_allow_pool_delete = true

[osd]
osd_journal_size = 1024
osd_max_object_name_len = 256

[client.rgw]
rgw_frontends = civetweb port=7480
rgw_dns_name = rgw.ceph.local
EOF

# Create docker-compose.ceph.yml
cat > docker-compose.ceph.yml <<EOF
version: '3.8'

networks:
  ceph-network:
    driver: bridge
    ipam:
      config:
        - subnet: $CLUSTER_NETWORK

services:
  mon1:
    image: quay.io/ceph/ceph:$CEPH_VERSION
    container_name: ceph-mon1
    networks:
      ceph-network:
        ipv4_address: 172.20.0.11
    volumes:
      - ./ceph-data/ceph.conf:/etc/ceph/ceph.conf
      - ./ceph-data/mon1:/var/lib/ceph/mon/ceph-mon1
    command: mon
    environment:
      - MON_IP=172.20.0.11
      - CEPH_PUBLIC_NETWORK=$CLUSTER_NETWORK
    hostname: mon1

  mon2:
    image: quay.io/ceph/ceph:$CEPH_VERSION
    container_name: ceph-mon2
    networks:
      ceph-network:
        ipv4_address: 172.20.0.12
    volumes:
      - ./ceph-data/ceph.conf:/etc/ceph/ceph.conf
      - ./ceph-data/mon2:/var/lib/ceph/mon/ceph-mon2
    command: mon
    environment:
      - MON_IP=172.20.0.12
      - CEPH_PUBLIC_NETWORK=$CLUSTER_NETWORK
    hostname: mon2
    depends_on:
      - mon1

  mon3:
    image: quay.io/ceph/ceph:$CEPH_VERSION
    container_name: ceph-mon3
    networks:
      ceph-network:
        ipv4_address: 172.20.0.13
    volumes:
      - ./ceph-data/ceph.conf:/etc/ceph/ceph.conf
      - ./ceph-data/mon3:/var/lib/ceph/mon/ceph-mon3
    command: mon
    environment:
      - MON_IP=172.20.0.13
      - CEPH_PUBLIC_NETWORK=$CLUSTER_NETWORK
    hostname: mon3
    depends_on:
      - mon2

  osd1:
    image: quay.io/ceph/ceph:$CEPH_VERSION
    container_name: ceph-osd1
    networks:
      - ceph-network
    volumes:
      - ./ceph-data/ceph.conf:/etc/ceph/ceph.conf
      - ./ceph-data/osd1:/var/lib/ceph/osd/ceph-0
    command: osd
    environment:
      - OSD_TYPE=directory
    depends_on:
      - mon3
    privileged: true

  osd2:
    image: quay.io/ceph/ceph:$CEPH_VERSION
    container_name: ceph-osd2
    networks:
      - ceph-network
    volumes:
      - ./ceph-data/ceph.conf:/etc/ceph/ceph.conf
      - ./ceph-data/osd2:/var/lib/ceph/osd/ceph-1
    command: osd
    environment:
      - OSD_TYPE=directory
    depends_on:
      - osd1
    privileged: true

  osd3:
    image: quay.io/ceph/ceph:$CEPH_VERSION
    container_name: ceph-osd3
    networks:
      - ceph-network
    volumes:
      - ./ceph-data/ceph.conf:/etc/ceph/ceph.conf
      - ./ceph-data/osd3:/var/lib/ceph/osd/ceph-2
    command: osd
    environment:
      - OSD_TYPE=directory
    depends_on:
      - osd2
    privileged: true

  mgr:
    image: quay.io/ceph/ceph:$CEPH_VERSION
    container_name: ceph-mgr
    networks:
      - ceph-network
    volumes:
      - ./ceph-data/ceph.conf:/etc/ceph/ceph.conf
      - ./ceph-data/mgr:/var/lib/ceph/mgr
    command: mgr
    ports:
      - "7000:7000"  # Dashboard
      - "9283:9283"  # Prometheus metrics
    depends_on:
      - mon3

  rgw:
    image: quay.io/ceph/ceph:$CEPH_VERSION
    container_name: ceph-rgw
    networks:
      - ceph-network
    volumes:
      - ./ceph-data/ceph.conf:/etc/ceph/ceph.conf
      - ./ceph-data/rgw:/var/lib/ceph/radosgw
    command: rgw
    ports:
      - "7480:7480"  # S3 endpoint
    environment:
      - RGW_NAME=rgw.ceph.local
    depends_on:
      - mgr
EOF

echo "✅ Configuration files created"

# Start Ceph cluster
echo "🐳 Starting Ceph cluster..."
docker-compose -f docker-compose.ceph.yml up -d

echo "⏳ Waiting for cluster to stabilize (60s)..."
sleep 60

# Check cluster status
echo "📊 Checking cluster status..."
docker exec ceph-mon1 ceph -s

echo ""
echo "✅ Ceph cluster deployed!"
echo ""
echo "📋 Next steps:"
echo "  1. Create ORION user: ./scripts/ceph-demo.sh"
echo "  2. Configure orion-storage-cold:"
echo "     S3_ENDPOINT=http://localhost:7480"
echo "     S3_ACCESS_KEY=<from ceph-demo.sh>"
echo "     S3_SECRET_KEY=<from ceph-demo.sh>"
echo ""
echo "🌐 Dashboard: http://localhost:7000"
echo "📊 Prometheus: http://localhost:9283/metrics"
echo "🗄️  S3 Endpoint: http://localhost:7480"
