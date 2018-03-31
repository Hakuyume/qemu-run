pkgname=qemu-run
pkgver=0.2.3
pkgrel=1
pkgdesc='A simple wrapper for QEMU'
arch=('x86_64')
depends=('qemu-headless' 'ovmf' 'socat')
makedepends=('cargo' 'rust')

source=('qemu@.service')
sha256sums=('bd6ac77d8b53356b0afb37815d62b3844c41c97a23487c8e00a1c22d4fb4b7c0')

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
