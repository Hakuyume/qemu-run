pkgname=qemu-run
pkgver=0.2.4
pkgrel=2
pkgdesc='A simple wrapper for QEMU'
arch=('x86_64')
depends=('qemu-headless' 'ovmf' 'socat')
makedepends=('cargo' 'rust')

source=('qemu@.service')
sha256sums=('c4db1ed596f2cff388baf7a9474c989363429d4df779b1b19f15b14f047cad93')

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
