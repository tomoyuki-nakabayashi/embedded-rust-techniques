# FFI (Foreign Function Interface)

組込みRustは急速に環境が整備されつつありますが、まだまだ不足しているものがたくさんあります。
そこで、C言語の資産活用が重要になります。
本章では、ベアメタル環境でのC言語とのFFIについて説明します。

標準ライブラリが使える場合のFFIについては、実践Rust入門のFFIの章が詳しいです。
標準ライブラリ内でC言語とのFFIに使えるモジュールには、`std::ffi`と`std::os::raw`があります。
残念ながら、どちらのモジュールも`core`には含まれておらず、`#![no_std]`環境では利用できません。

代わりに、[cstr_core]と[cty]クレートが利用できます。

[cstr_core]: https://crates.io/crates/cstr_core
[cty]: https://crates.io/crates/cty