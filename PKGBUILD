pkgname=qemu-run
pkgver=0.2.0
pkgrel=1
pkgdesc='A simple wrapper for QEMU'
arch=('x86_64')
depends=('qemu-headless' 'ovmf' 'socat')
makedepends=('cargo' 'rust')

source=('qemu@.service')
sha256sums=('d2bed686b89660023725d4bccdd948733d235bdc4b2c83e932ffe6d7e9a0f8a9')

prepare() {
    cp -au "$startdir/$pkgname" "$srcdir/"
}

build() {
    cd "$srcdir/$pkgname"
    cargo build --release
}

package() {
    install -m 755 -d "$pkgdir/usr/bin"
    install -m 755 "$srcdir/$pkgname/target/release/$pkgname" "$pkgdir/usr/bin/"

    install -m 755 -d "$pkgdir/usr/lib/systemd/system"
    install -m 644 "$srcdir/qemu@.service" "$pkgdir/usr/lib/systemd/system/"
}
