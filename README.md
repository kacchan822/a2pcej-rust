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

### Options

| Short | Long | Description |
| --- | --- | --- |
| `-m` | `--mode` | Conversion mode: `en` or `ja` (required) |
| `-d` | `--delimiter` | Use a custom delimiter |
| `-nd` | `--nodelimiter` | Use no delimiter |
| `-s` | `--sign` | Use a custom uppercase sign |
| `-ns` | `--nosign` | Use no uppercase sign |
| `-n` | `--num` | Convert digits to phonetic names |

## Testing

```sh
cargo test
```
