#!/bin/bash

# Create the package directory structure
mkdir -p deb-build/nflux/DEBIAN
mkdir -p deb-build/nflux/usr/local/bin
mkdir -p deb-build/nflux/etc/systemd/system

# Copy control and service files
cp debian/control deb-build/nflux/DEBIAN/
cp debian/nflux.service deb-build/nflux/etc/systemd/system/
cp debian/postrm deb-build/nflux/DEBIAN/

# Build the Rust binary and place it in the package structure
cargo xtask build --release
cp target/release/nflux deb-build/nflux/usr/local/bin/

# Set permissions for package files
chmod 755 deb-build/nflux/DEBIAN
chmod 644 deb-build/nflux/etc/systemd/system/nflux.service
chmod 755 deb-build/nflux/usr/local/bin/nflux

# Build the .deb package
dpkg-deb --build deb-build/nflux

# Move .deb package to root dir
mv deb-build/nflux.deb nflux.deb
