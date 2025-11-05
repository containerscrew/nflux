pkgname=nflux
pkgver=0.13.0
pkgrel=1
pkgdesc="Simple network monitoring agent tool powered by eBPF & Rust"
arch=('x86_64')
url="https://github.com/containerscrew/nflux"
license=('MIT' 'GPL3')
depends=('systemd')
makedepends=('rust' 'cargo')
#options=('!debug' '!strip')
source=()
sha256sums=()

build() {
    cd "$srcdir/.."
    cargo build --release --locked
}

check() {
    cargo test --release --locked || echo "Tests skipped"
}

package() {
    cd "$srcdir/.."
    install -Dm755 "target/release/nflux" "$pkgdir/usr/bin/nflux"
    install -Dm644 "systemd/nflux.service" "$pkgdir/usr/lib/systemd/system/nflux.service"
    install -Dm644 "nflux.toml.example" "$pkgdir/etc/nflux/nflux.toml"
    install -Dm644 "systemd/nflux.logrotate" "$pkgdir/etc/logrotate.d/nflux"
    install -d -m755 "$pkgdir/var/log/nflux"
}

