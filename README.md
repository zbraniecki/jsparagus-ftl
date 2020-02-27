== Synopsis ==

This is an experiment of using `fluent-rs` lexer branch approach to parsing JavaScript.

It uses lexer/parser combo, mostly to improve the debugging experience and simplify the
tokenization phase.

The appoach is very POC level. Some decisions were made that will have to be revisited like
using `Peekable` which makes the `Lexer` opaque. Instead, we could implement peeking inside of `Lexer`
just like `fluent-rs` does, and then get access to custom methods on the `Lexer` like `take_if` etc.

Generally speaking tho, this performance should be similar, for the given `simple` script,
to what we could end up with for the runtime scenario.

== Usage ==

```
cargo run --bin lexer ./benches/simple.js
cargo run --bin parser ./benches/simple.js

cargo run --bin lexer ./benches/complex.js
cargo run --bin parser ./benches/complex.js

cargo bench lexer
cargo bench parser

cargo bench simple
cargo bench complex
```
