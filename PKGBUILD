pkgname=qemu-run
pkgver=0.1
pkgrel=1
pkgdesc='A simple wrapper for QEMU'
arch=('x86_64')
depends=('python' 'python-yaml' 'qemu-headless' 'ovmf' 'socat')
options=(!strip)
source=('qemu-run'
        'qemu@.service')
sha256sums=('48891459d03c8912f17f7f0e0ffd1f1e546074e3698d92a9287aa0870ae2f299'
            'd2bed686b89660023725d4bccdd948733d235bdc4b2c83e932ffe6d7e9a0f8a9')

package() {
    install -m 755 -d "$pkgdir/usr/bin"
    install -m 755 "$srcdir/qemu-run" "$pkgdir/usr/bin/"

    install -m 755 -d "$pkgdir/usr/lib/systemd/system"
    install -m 644 "$srcdir/qemu@.service" "$pkgdir/usr/lib/systemd/system"
}
