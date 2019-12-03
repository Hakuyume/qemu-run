pkgname=qemu-run
pkgver=0.2.8
pkgrel=1
pkgdesc='A simple wrapper for QEMU'
arch=('x86_64')
depends=('libusb' 'ovmf' 'qemu-headless' 'socat')
makedepends=('cargo' 'rust')

source=('qemu@.service')
sha256sums=('315f2a3659d77dd2b149219daa29e9193de28d82d145b96b918b67ae02815423')

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
