# Maintainer: Daniels <info@containerscrew.com>
pkgname=nflux
pkgver=0.1.0
pkgrel=1
pkgdesc="Network monitoring and firewall using EBPF, XDP, and TC. Powered by Aya-rs"
arch=('x86_64')
url="https://github.com/containerscrew/nflux"
license=('AGPL-3.0')
depends=('bpf-linker')  # Add runtime dependencies if needed
makedepends=('rust' 'cargo')  # Rust tools required for building

source=(
    "$pkgname-$pkgver.tar.gz::https://github.com/containerscrew/nflux/archive/refs/tags/v$pkgver.tar.gz"
    "nflux.service"  # Include the systemd service file
)
sha256sums=('SKIP' 'SKIP')  # Replace with actual sha256sums for both files

build() {
    # Change to the root directory of the extracted source
    cd "$srcdir/$pkgname-$pkgver"
    # Run `cargo xtask build --release` from the root
    cargo xtask build --release
}

package() {
    cd "$srcdir/$pkgname-$pkgver"
    # Install the binary to /usr/local/bin/
    install -Dm755 "target/release/nflux" "$pkgdir/usr/local/bin/nflux"

    # Install the systemd service file to /etc/systemd/system/
    install -Dm644 "$srcdir/nflux.service" "$pkgdir/etc/systemd/system/nflux.service"
}
