pkgname=qemu-run
pkgver=0.2.2
pkgrel=1
pkgdesc='A simple wrapper for QEMU'
arch=('x86_64')
depends=('qemu-headless' 'ovmf' 'socat')
makedepends=('cargo' 'rust')

source=('qemu@.service')
sha256sums=('6c248cabec0d373624240c771c7418dce3ec85d7c6f026408d46ba3cd9ae6ab2')

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
