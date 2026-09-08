# a2pcej-rust

Rust implementation of [a2pcej](https://github.com/kacchan822/a2pcej), converting
ASCII alphabet letters to English phonetic code or Japanese katakana names.

```rust
use a2pcej::{conv_al, A2pcej, Language, Options};

assert_eq!(conv_al("hoge"), "Hotel-Oscar-Golf-Echo");

let mut options = Options::japanese();
options.num = true;
let converter = A2pcej::new(Language::Japanese, options);
assert_eq!(converter.convert("A04"), "エイ（大文字）・ゼロ・ヨン");
```

## CLI

```sh
cargo run -- -m en Examples004
cargo run -- -m ja -n Examples004
cargo run -- -m en -d ', ' -s '(CAPITAL)' Examples003
```

Options match the Python CLI: `-m/--mode`, `-d/--delimiter`,
`-nd/--nodelimiter`, `-s/--sign`, `-ns/--nosign`, and `-n/--num`.

Run tests with `cargo test`.
