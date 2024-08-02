#!/bin/bash

# Install Rust
curl -1 --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Install necessary packages
sudo apt-get install -y libssl-dev
sudo apt-get install -y postgresql
sudo apt-get install -y docker-compose
pip install docker -y requests

# Add Docker's official GPG key:
sudo apt-get update
sudo apt-get install ca-certificates curl
sudo install -m 0755 -d /etc/apt/keyrings
sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
sudo chmod a+r /etc/apt/keyrings/docker.asc

# Add the repository to Apt sources:
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu \
  $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

sudo apt-get update
sudo apt-get install docker-ce docker-engine docker.io containerd runc


# Create PostgreSQL user with superuser privileges and password
sudo -u postgres psql -c "CREATE USER RustPi WITH SUPERUSER PASSWORD 'rust';"
# Switch to the postgres user and create a new database
sudo -u postgres psql -c "CREATE DATABASE RustPi WITH ENCODING 'UTF8';"

docker-compose up -d --build