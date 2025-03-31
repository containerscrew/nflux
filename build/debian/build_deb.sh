#!/bin/bash

# Create the package directory structure
mkdir -p deb-build/nflux/DEBIAN
mkdir -p deb-build/nflux/usr/share/doc/nflux
mkdir -p deb-build/nflux/usr/local/bin


# Copy control and service files
cp build/debian/control deb-build/nflux/DEBIAN/
cp docs/usage.md deb-build/nflux/usr/share/doc/nflux/

# Build the Rust binary and place it in the package structure
cargo build --release
cp target/release/nflux deb-build/nflux/usr/local/bin/

# Set permissions for package files
chmod 755 deb-build/nflux/DEBIAN
chmod 755 deb-build/nflux/usr/local/bin/nflux

# Build the .deb package
dpkg-deb --build deb-build/nflux

# Move .deb package to root dir
# mv deb-build/nflux.deb nflux.deb
