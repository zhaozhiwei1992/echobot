  # Maintainer: Your Name <your@email.com>
  pkgname=echobot
  pkgver=0.1.0
  pkgrel=1
  pkgdesc="A simple echo bot written in Rust"
  arch=('x86_64')
  license=('MIT')
  makedepends=('rust')
  source=("echobot-$pkgver.tar.gz")

  sha512sums=('SKIP')

  build() {
  cd "$srcdir/$pkgname-$pkgver"
  cargo build --release
  }

  package() {
  cd "$srcdir/$pkgname-$pkgver"
  install -Dm755 "target/release/echobot" "$pkgdir/usr/bin/echobot"
  }

