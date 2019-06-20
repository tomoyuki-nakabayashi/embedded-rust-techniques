## Yocto

Yoctoは、組込みLinuxディストリビューションを作成するためのプロジェクトです。
製品固有のLinuxディストリビューションを作成、管理できるため、組込みLinux開発で広く用いられています。
ここで言うLinuxディストリビューションは、Linux kernel、ライブラリ、アプリケーションを全て含みます。

日本語のまとまった書籍は、2019年現在ありませんが、雑誌インタフェースなどで、Raspberry Piの独自環境構築や、FPGAボードZynqの環境構築方法が紹介されています。
[みつきんのメモ]は、Yocto関連のノウハウが多く掲載されており、普段からお世話になっています。

[みつきんのメモ]: http://mickey-happygolucky.hatenablog.com/

ここでは、RustプロジェクトをYoctoビルド環境にインテグレーションする方法を紹介します。
想定する利用方法は、ホスト上のRustツールチェインで一通り開発、デバッグを行った上で、distributionに取り込んで配布する、というものです。

Yoctoの基礎から説明するスキルが著者にないため、Yoctoを触ったことある方向けの情報になります。ご了承下さい。

ターゲット環境はraspberry pi3です。

### [meta-rust]

[meta-rust]: https://github.com/meta-rust/meta-rust

まず、raspberry pi3環境をビルドするためのレイヤを取得します。

```
mkdir -p rpi-thud/layers
cd rpi-thud/layers
git clone git://git.yoctoproject.org/poky.git -b thud
git clone git://git.yoctoproject.org/meta-raspberrypi -b thud
git clone git://git.openembedded.org/meta-openembedded -b thud
```

次に、Rustのパッケージを含んでいる`meta-rust`をcloneします。

```
git clone https://github.com/meta-rust/meta-rust.git
```

環境変数を読み込みます。

```
source layers/poky/oe-init-build-env build
```

ビルド対象のレイヤを追加します。

```
bitbake-layers add-layer ../layers/meta-openembedded/meta-oe
bitbake-layers add-layer ../layers/meta-openembedded/meta-python
bitbake-layers add-layer ../layers/meta-openembedded/meta-networking
bitbake-layers add-layer ../layers/meta-raspberrypi
bitbake-layers add-layer ../layers/meta-rust
```

`local.conf`を修正します。

ターゲットをraspberry pi3にします。

```
MACHINE = "raspberrypi3"
```

Rustのサンプルパッケージをrootfsにインストールするようにします。

```
IMAGE_INSTALL_append = " rust-hello-world"
```

ビルドします。

```
bitbake core-image-base
```

`dd`コマンドでマイクロSDカードにイメージを書き込みます。

```
sudo dd if=tmp/deploy/images/raspberrypi3/core-image-base-raspberrypi3.rpi-sdimg of=/dev/sdX bs=100M
```

`/sdX`は使用している環境に合わせて適宜変更して下さい。

raspberry pi3を起動して、`rust-hello-world`を実行します。

```
# rust-hello-world
Hello, world!
```

無事、実行できます。

### [meta-rust-bin]

[meta-rust-bin]: https://github.com/rust-embedded/meta-rust-bin

`meta-rust`では、LLVM、Rustコンパイラ、CargoをビルドしてRustツールチェインを構築するため、ビルド時間が大幅に増加します。
それにも関わらず、Yoctoで作成したクロス開発環境には、このツールチェインが含まれません。
純粋に、Rustのプロジェクトをビルドするだけであれば、既存のRustツールチェインバイナリを取得する方がよほどお手軽です。

そこで、Rustのツールチェインバイナリを取得して、Rustプロジェクトをビルドする`meta-rust-bin`があります。

> TODO: 使い方

### [cargo-bitbake]

[cargo-bitbake]: https://github.com/cardoe/cargo-bitbake

既存のCargoプロジェクトから`meta-rust`のYoctoレシピを作成してくれるCargoの拡張機能です。

`cargo-bitbake`は`libssl-dev`を使用するため、インストールします。

```
sudo apt install libssl-dev
```

`cargo-bitbake`をインストールします。

```
cargo install cargo-bitbake
```

Rust製grepツールの[ripgrep]を取り込んでみます。

[ripgrep]: https://github.com/BurntSushi/ripgrep

```
git clone https://github.com/BurntSushi/ripgrep.git
cd ripgrep
cargo bitbake
```

これで、レシピが生成されます。

> TODO: `meta-rust-bin`でレシピが利用できるかどうか試す

## 参考

- [yoctoでもRust]

[yoctoでもRust]: http://mickey-happygolucky.hatenablog.com/entry/2019/05/01/125555?_ga=2.6048497.350764793.1560857949-1518570932.1554416614