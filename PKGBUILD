pkgname=qemu-run
pkgver=0.2.4
pkgrel=1
pkgdesc='A simple wrapper for QEMU'
arch=('x86_64')
depends=('qemu-headless' 'ovmf' 'socat')
makedepends=('cargo' 'rust')

source=('qemu@.service')
sha256sums=('768ee7a351bbacb170d25266c5c2310eb9f92536f29d9d7db7120fcc90f88fed')

prepare() {
    cp -au "$startdir/$pkgname" "$srcdir/"
}

build() {
    cd "$srcdir/$pkgname/"
    cargo build --release
}

check() {
    cd "$srcdir/$pkgname/"
    cargo test --release
}

package() {
    install -m 755 -d "$pkgdir/usr/bin"
    install -m 755 "$srcdir/$pkgname/target/release/$pkgname" "$pkgdir/usr/bin/"

    install -m 755 -d "$pkgdir/usr/lib/systemd/system"
    install -m 644 "$srcdir/qemu@.service" "$pkgdir/usr/lib/systemd/system/"
}
