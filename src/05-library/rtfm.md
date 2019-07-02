## 5-4. RTFM (Real Time For the Masses)

[RTFM for ARM Cortex-M]は、リアルタイムシステムを構築するための並行処理フレームワークです。[Real-time for the masses, step 1: Programming API and static priority SRP kernel primitives.]というリアルタイムシステム構築の論文を、Rustで実装しています。RTOSほどの機能はありませんが、小規模なリアルタイムシステム構築に向いています。

[RTFM for ARM Cortex-M]: https://github.com/japaric/cortex-m-rtfm
[Real-time for the masses, step 1: Programming API and static priority SRP kernel primitives.]: https://www.diva-portal.org/smash/get/diva2:1005680/FULLTEXT01.pdf

### 機能一覧

- 並行処理の単位tとしてタスクが定義されています。タスクはイベントトリガ、もしくは、アプリケーションからspawnすることができます。
- タスク間でメッセージ送受信が可能です。
- ソフトウェアタスクをスケジュールするタイマキューがあります。周期タスクを実装するために利用できます。
- 優先度付きのタスク、および、プリエンプティブマルチタスキングを提供します。
- 優先度に基づいたクリティカルセクション制御により、効率的でデータ競合のないメモリ共有が可能です。
- コンパイル時にデッドロックが発生しないことが保証されます。
- スケジューラは最小限のソフトウェアで実装されており、スケジューリングオーバーヘッドは最小です。
- 全てのタスクが1つのコールスタックを共有しており、極めて効率的にメモリを利用します。
- 全Cortex-Mデバイスをサポートしています。

### アプリケーション実装方法

**cortex-m-rt**クレートと**Peripheral Access Crate (PAC)**に、初期化、タスク、優先度、共有リソースの概念が追加されます。

```rust,ignore
#[app(device = lm3s6965)]
const APP: () = {
    #[init]
    fn init() {
        // Cortex-M peripherals
        let _core: rtfm::Peripherals = core;

        // Device specific peripherals
        let _device: lm3s6965::Peripherals = device;

        // Pends the UART0 interrupt but its handler won't run until *after*
        // `init` returns because interrupts are disabled
        rtfm::pend(Interrupt::UART0);

        hprintln!("init").unwrap();
    }

    #[idle]
    fn idle() -> ! {
        // interrupts are enabled again; the `UART0` handler runs at this point

        hprintln!("idle").unwrap();

        rtfm::pend(Interrupt::UART0);

        debug::exit(debug::EXIT_SUCCESS);

        loop {}
    }

    #[interrupt]
    fn UART0() {
        static mut TIMES: u32 = 0;

        // Safe access to local `static mut` variable
        *TIMES += 1;

        hprintln!(
            "UART0 called {} time{}",
            *TIMES,
            if *TIMES > 1 { "s" } else { "" }
        )
        .unwrap();
    }
};
```

「#[app(..)]」アトリビュートは、device引数を使って、svd2rustで生成されたPACのパスを指定します。「#[init]」アトリビュートが指定された関数は、アプリケーションとして実行される最初の関数です。この関数は、割り込み禁止状態で実行します。

coreとdeviceという変数があり、この変数を通して、Cortex-Mとペリフェラルにアクセスできます。

### コラム〜RTFMの実装〜

RTFMソースコードを覗いてみると、その多くが手続きマクロによる静的検査と、コードジェネレータであることがわかります。**cortex-m-rt**クレートを使用する場合、割り込みとメイン関数間でのデータ共有に制限があります。常に、全ての割り込みを無効化する**cortex_m::interrupt::Mutex**を使わなければなりません。しかし、全ての割り込みを無効化することは、常に求められる条件ではありません。

例えば、2つの割り込みハンドラがデータを共有する場合、両者の優先度が同じで、プリエンプションが発生しないとすると、ロックは不要です。RTFMでは、ソースコードを静的に解析することで、不要なロックをせずに、共有データにアクセスできるようになっています。このような解析が可能な理由は、appアトリビュート内にアプリケーションの実装を、全て書くためです。

また、RTFMでは、動的な割り込み優先度の変更をサポートしていません。そのため、全ての割り込みハンドラ間の優先度は静的に決定します。これがうまいこと生きており、ある共有データを使用する割り込みハンドラ同士で、最も優先度の高いハンドラはロックを取得しなくても共有データにアクセスできます。

RTFMは、機能性と安全性を両立するアプローチです。しかし、複雑な手続きマクロで実装されているため、自分で機能を追加したり、アプリケーションをデバッグするのは、骨が折れそうです。

### more information

[RTFMのドキュメント]にRTFMの使い方がまとめられています。

[RTFMのドキュメント]: https://japaric.github.io/cortex-m-rtfm/book/en/

[低レイヤ強くなりたい組み込みやさんのブログ]で、RTRMについていくつかエントリを書きました。タスクの利用方法や共有リソースの管理方法について気になった方は、こちらを参照下さい。

[低レイヤ強くなりたい組み込みやさんのブログ]: https://tomo-wait-for-it-yuki.hatenablog.com/archive/category/rtfm
