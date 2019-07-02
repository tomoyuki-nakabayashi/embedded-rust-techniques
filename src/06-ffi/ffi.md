# 6. FFI (Foreign Function Interface)

組込みRustは急速に環境が整備されつつありますが、まだまだ不足しているものがたくさんあります。そこで、C言語の資産活用が重要になります。本章では、ベアメタル環境でのC言語とのFFIについて説明します。

標準ライブラリが使える場合のFFIについては、実践Rust入門のFFIの章が詳しいです。標準ライブラリ内でC言語とのFFIに使えるモジュールには、`std::ffi`と`std::os::raw`があります。残念ながら、どちらのモジュールも`core`には含まれておらず、`#![no_std]`環境では利用できません。

代わりに、[cty]クレートと[cstr_core]クレートとを利用します。

[cstr_core]: https://crates.io/crates/cstr_core
[cty]: https://crates.io/crates/cty

ctyクレートは、コンパイラによって暗黙変換される低レベルのプリミティブ型を扱います。このようなプリミティブ型には、C言語の`unsigned int`を表現する`c_uint`などがあります。

```rust,ignore
unsafe fn foo(num: u32) {
    let c_num: c_uint = num;  // 暗黙変換
}
```

cstr_coreクレートは、文字列のようなより複雑な型を変換するユーティリティを提供します。