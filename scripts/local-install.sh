#!/usr/bin/env bash

set -e

# Compile the program
cargo build --release

BIN_NAME="target/release/nflux"
SERVICE_NAME="systemd/nflux.service"

# Copy the binary
sudo install -m 0755 "$BIN_NAME" /usr/bin/

# Copy systemd service
sudo install -m 0644 "$SERVICE_NAME" /etc/systemd/system/

# Copy the config file
sudo mkdir -p /etc/nflux
sudo cp /etc/nflux/nflux.toml /etc/nflux/nflux.toml.bak 2>/dev/null || true
sudo cp nflux.toml.example /etc/nflux/nflux.toml
sudo chown -R root:root /etc/nflux
sudo chmod 644 /etc/nflux/nflux.toml

# Create log directory and file
sudo mkdir -p /var/log/nflux
sudo rm -f /var/log/nflux/lastlog
sudo touch /var/log/nflux/lastlog
sudo chown root:root /var/log/nflux/lastlog
sudo chmod 600 /var/log/nflux/lastlog

# Reload systemd and enable service
sudo systemctl daemon-reload
sudo systemctl enable --now nflux.service

printf "\nInstallation complete. Service 'nflux' is installed and running.\n"