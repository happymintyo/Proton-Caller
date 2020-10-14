# Maintainer: Avery <averylapine@gmail.com>
pkgname=proton-call
pkgver=1.0.0a
pkgrel=1
pkgdesc="Run any window program through proton"
arch=('any')
url="https://github.com/caverym/Proton-Caller/"
license=('GPL3')
depends=(
  'steam'
  'dxvk'
)
source=("https://github.com/caverym/Proton-Caller/archive/${pkgver}.tar.gz")
sha256sums=('663838390e3123a4a237cc8a0260426641bf32c7239c7e3e7475746056e4d1b5')

build() {
  cd "$srcdir/$pkgname-$pkgver"
    make build
}

package() {
  cd "$srcdir/$pkgname-$pkgver"
  make install
}
