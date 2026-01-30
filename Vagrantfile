# -*- mode: ruby -*-
# vi: set ft=ruby :

# ORION Unified CDR Engine - Vagrantfile
# Automated RHEL/AlmaLinux VM provisioning for VirtualBox
# Author: ORION Team
# Date: January 2026

Vagrant.configure("2") do |config|
  # Base box - AlmaLinux 9 (RHEL-compatible, no subscription needed)
  # Alternative: Use "generic/rhel9" if you have RHEL subscription
  config.vm.box = "almalinux/9"
  config.vm.box_version = ">= 9.0"
  
  # VM Configuration
  config.vm.hostname = "orion-rhel-prod"
  
  # Network Configuration
  # Port forwarding for ORION services
  config.vm.network "forwarded_port", guest: 8080, host: 8080, protocol: "tcp"  # API
  config.vm.network "forwarded_port", guest: 8081, host: 8081, protocol: "tcp"  # Ingestion
  config.vm.network "forwarded_port", guest: 8085, host: 8085, protocol: "tcp"  # Storage Hot
  config.vm.network "forwarded_port", guest: 9200, host: 9200, protocol: "tcp"  # Traffic Gen
  config.vm.network "forwarded_port", guest: 3000, host: 3000, protocol: "tcp"  # Grafana
  config.vm.network "forwarded_port", guest: 9090, host: 9090, protocol: "tcp"  # Prometheus
  config.vm.network "forwarded_port", guest: 9042, host: 9042, protocol: "tcp"  # ScyllaDB
  
  # Private network for host-only access
  config.vm.network "private_network", type: "dhcp"
  
  # Shared folder (optional - for development)
  # config.vm.synced_folder ".", "/home/vagrant/ORION-Unified-CDR-Engine", type: "virtualbox"
  
  # VirtualBox Provider Configuration
  config.vm.provider "virtualbox" do |vb|
    # Display name in VirtualBox GUI
    vb.name = "ORION-RHEL-Production"
    
    # Performance settings
    vb.memory = "16384"  # 16 GB RAM (adjust based on your system)
    vb.cpus = 4          # 4 CPU cores (adjust based on your system)
    
    # Enable hardware virtualization
    vb.customize ["modifyvm", :id, "--nested-hw-virt", "on"]
    vb.customize ["modifyvm", :id, "--ioapic", "on"]
    vb.customize ["modifyvm", :id, "--pae", "on"]
    
    # Video settings
    vb.customize ["modifyvm", :id, "--vram", "128"]
    vb.customize ["modifyvm", :id, "--graphicscontroller", "vmsvga"]
    vb.customize ["modifyvm", :id, "--accelerate3d", "on"]
    
    # Network performance
    vb.customize ["modifyvm", :id, "--natdnshostresolver1", "on"]
    vb.customize ["modifyvm", :id, "--natdnsproxy1", "on"]
    
    # Disk size (requires vagrant-disksize plugin)
    # Install: vagrant plugin install vagrant-disksize
    # vb.customize ["modifyhd", "disk_id", "--resize", 150 * 1024]
  end
  
  # Provisioning Script - Executed on first 'vagrant up'
  config.vm.provision "shell", inline: <<-SHELL
    set -e
    
    echo "========================================="
    echo "🚀 ORION Unified CDR Engine - Setup"
    echo "========================================="
    
    # System update
    echo "📦 Updating system packages..."
    dnf update -y
    
    # Install essential tools
    echo "🔧 Installing essential tools..."
    dnf install -y \
      git \
      curl \
      wget \
      vim \
      htop \
      tmux \
      net-tools \
      firewalld \
      policycoreutils-python-utils \
      rsync \
      tar \
      gzip
    
    # Install Docker
    echo "🐋 Installing Docker..."
    dnf config-manager --add-repo=https://download.docker.com/linux/rhel/docker-ce.repo
    dnf install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin
    
    # Start Docker
    systemctl enable --now docker
    
    # Add vagrant user to docker group
    usermod -aG docker vagrant
    
    # Configure firewall
    echo "🔥 Configuring firewall..."
    systemctl enable --now firewalld
    
    # ORION services ports
    firewall-cmd --permanent --add-port=8080/tcp   # API
    firewall-cmd --permanent --add-port=8081/tcp   # Ingestion
    firewall-cmd --permanent --add-port=8082/tcp   # Validation
    firewall-cmd --permanent --add-port=8083/tcp   # Normalization
    firewall-cmd --permanent --add-port=8084/tcp   # Enrichment
    firewall-cmd --permanent --add-port=8085/tcp   # Storage Hot
    firewall-cmd --permanent --add-port=8086/tcp   # ML Fraud Agent
    firewall-cmd --permanent --add-port=9200/tcp   # Traffic Generator
    firewall-cmd --permanent --add-port=9400/tcp   # Storage Cold
    
    # Monitoring ports
    firewall-cmd --permanent --add-port=3000/tcp   # Grafana
    firewall-cmd --permanent --add-port=9090/tcp   # Prometheus
    
    # Database ports
    firewall-cmd --permanent --add-port=9042/tcp   # ScyllaDB
    firewall-cmd --permanent --add-port=9000/tcp   # MinIO API
    firewall-cmd --permanent --add-port=9001/tcp   # MinIO Console
    
    # Kafka ports
    firewall-cmd --permanent --add-port=9092/tcp   # Kafka
    firewall-cmd --permanent --add-port=2181/tcp   # ZooKeeper
    
    firewall-cmd --reload
    
    # SELinux configuration
    echo "🔒 Configuring SELinux..."
    # Keep enforcing mode but allow Docker
    setenforce 1
    setsebool -P container_manage_cgroup on
    
    # Kernel tuning for ORION workloads
    echo "⚙️ Applying kernel tuning..."
    cat >> /etc/sysctl.conf <<EOF

# ORION CDR Engine optimizations
# Network
net.core.somaxconn = 4096
net.core.netdev_max_backlog = 5000
net.ipv4.tcp_max_syn_backlog = 8192
net.ipv4.tcp_fin_timeout = 30
net.ipv4.tcp_keepalive_time = 300
net.ipv4.tcp_keepalive_intvl = 30
net.ipv4.tcp_keepalive_probes = 3

# Memory
vm.swappiness = 1
vm.max_map_count = 262144

# File descriptors
fs.file-max = 100000
EOF
    sysctl -p
    
    # Increase file limits
    echo "📂 Configuring file limits..."
    cat >> /etc/security/limits.conf <<EOF

# ORION file limits
*               soft    nofile          100000
*               hard    nofile          100000
vagrant         soft    nofile          100000
vagrant         hard    nofile          100000
EOF
    
    # Clone ORION repository
    echo "📥 Cloning ORION repository..."
    cd /home/vagrant
    if [ ! -d "ORION-Unified-CDR-Engine" ]; then
      sudo -u vagrant git clone https://github.com/FCHEHIDI/ORION-Unified-CDR-Engine.git
    else
      echo "Repository already exists, pulling latest changes..."
      cd ORION-Unified-CDR-Engine
      sudo -u vagrant git pull
    fi
    
    # Create ORION directories
    echo "📁 Creating ORION directories..."
    mkdir -p /opt/orion/{bin,config,logs,data,certs}
    chown -R vagrant:vagrant /opt/orion
    
    # Setup systemd directory
    mkdir -p /opt/orion/systemd
    
    # Build Docker images (optional - uncomment to auto-build)
    # echo "🔨 Building ORION Docker images..."
    # cd /home/vagrant/ORION-Unified-CDR-Engine
    # sudo -u vagrant docker compose build
    
    # Display completion message
    echo ""
    echo "========================================="
    echo "✅ ORION Setup Complete!"
    echo "========================================="
    echo ""
    echo "📍 Next Steps:"
    echo "1. SSH into VM: vagrant ssh"
    echo "2. Build images: cd ORION-Unified-CDR-Engine && docker compose build"
    echo "3. Start ORION: docker compose up -d"
    echo "4. Check status: docker compose ps"
    echo ""
    echo "🌐 Access from host:"
    echo "- Grafana:    http://localhost:3000"
    echo "- Prometheus: http://localhost:9090"
    echo "- API:        http://localhost:8080"
    echo ""
    echo "📊 System Info:"
    echo "- RAM: $(free -h | grep Mem | awk '{print $2}')"
    echo "- CPU: $(nproc) cores"
    echo "- Disk: $(df -h / | tail -1 | awk '{print $4}') available"
    echo ""
    echo "========================================="
  SHELL
  
  # Post-provisioning message
  config.vm.post_up_message = <<-MESSAGE
  ╔═══════════════════════════════════════════════════════╗
  ║  🚀 ORION Unified CDR Engine - VM Ready!             ║
  ╚═══════════════════════════════════════════════════════╝
  
  Connect to VM:
    vagrant ssh
  
  Build & Start ORION:
    cd ORION-Unified-CDR-Engine
    docker compose build
    docker compose up -d
  
  Monitor logs:
    docker compose logs -f
  
  Access Grafana:
    http://localhost:3000 (admin/admin)
  
  VM Management:
    vagrant halt       # Stop VM
    vagrant up         # Start VM
    vagrant reload     # Restart VM
    vagrant destroy    # Delete VM
  MESSAGE
end
