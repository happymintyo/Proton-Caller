# Maintainer: Avery <averylapine@gmail.com>
pkgname=Proton-Caller
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
sha256sums=('517f5b4c1c3ffc769da9cb507744626e02efe8c85e04683f6d79a2decd3d550b')

build() {
  cd "$srcdir/$pkgname-$pkgver"
    make build
}

package() {
  cd "$srcdir/$pkgname-$pkgver"
  make install
}
