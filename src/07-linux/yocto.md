## Yocto

Yoctoは、組込みLinuxディストリビューションを作成するためのプロジェクトです。
製品固有のLinuxディストリビューションを作成、管理できるため、組込みLinux開発で広く用いられています。
ここで言うLinuxディストリビューションは、Linux kernel、ライブラリ、アプリケーションを全て含みます。

日本語のまとまった書籍は、2019年現在ありませんが、雑誌インタフェースなどで、Raspberry Piの独自環境構築や、FPGAボードZynqの環境構築方法が紹介されています。
[みつきんのメモ]は、Yocto関連のノウハウが多く掲載されており、普段からお世話になっています。

[みつきんのメモ]: http://mickey-happygolucky.hatenablog.com/

ここでは、RustプロジェクトをYoctoビルド環境にインテグレーションする方法を紹介します。
Yoctoの基礎から説明するスキルが著者にないため、Yoctoを触ったことある方向けの情報になります。ご了承下さい。

ターゲット環境はraspberry pi3です。

### meta-rust

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

### meta-rust-bin

### cargo-bitbake