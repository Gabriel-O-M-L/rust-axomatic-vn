#!/bin/bash

# Install Rust
curl -1 --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Install necessary packages
sudo apt-get install -y libssl-dev
sudo apt-get install -y postgresql

# Create PostgreSQL user with superuser privileges and password
sudo -u postgres psql -c "CREATE USER RustPi WITH SUPERUSER PASSWORD 'rust';"

# Switch to the postgres user and create a new database
sudo -u postgres psql -c "CREATE DATABASE RustPi WITH ENCODING 'UTF8';"

docker-compose up -d --build