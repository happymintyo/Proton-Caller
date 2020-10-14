# Maintainer: Avery Murray <averylapine@gmail.com>

pkgname=Proton-Caller
pkgver=1.0.0a4
pkgrel=5
pkgdesc="Run any Windows program through Proton"
arch=('any')
url="https://github.com/caverym/Proton-Caller/"
license=('GPL3')
depends=(
  'steam'
  'dxvk'
)
source=("https://github.com/caverym/Proton-Caller/archive/${pkgver}.tar.gz")
sha256sums=('e17d72cd99f6f50565f0539798aa674713e9cc80c768cdbd6d39992108db4b42')

build() {
  cd "$srcdir/$pkgname-$pkgver"
    g++ -o proton-call main.cpp
}

package() {
  mkdir -p "$pkgdir"/usr/bin/
  mkdir -p "$pkgdir"/usr/share/licenses/proton-call/
  cp -f "$srcdir/$pkgname-$pkgver"/LICENSE "$pkgdir"/usr/share/licenses/proton-call/
  install -g 0 -o 0 -m 0644 "$srcdir/$pkgname-$pkgver"/proton-call.6 "$pkgdir"/usr/share/man/man6/
  gzip -f "$pkgdir"/usr/share/man/man6/proton-call.6
  cp "$srcdir/$pkgname-$pkgver"/proton-call "$pkgdir"/usr/bin
}