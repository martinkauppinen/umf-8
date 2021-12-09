# UMF-8 -- UTF-8 and Morse code converter

## Why?
Because.

## How?
```
cargo run -- --help
```

Converting from UTF-8 to Morse code is done simply, with a space between each
Morse code point and a slash between words:
```
$ cargo run WHAT HATH GOD WROUGHT 🦀
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/umf-8 WHAT HATH GOD WROUGHT '🦀'`
.-- .... .- - / .... .- - .... / --. --- -.. / .-- .-. --- ..- --. .... - / -..... ....---- .--..... .-.--..- .------- .....-
```

Converting from Morse code to UTF-8 requires that the string of Morse code is
within quotes:
```
$ cargo run -- -u ".-- .... .- - / .... .- - .... / --. --- -.. / .-- .-. --- ..- --. .... - / -..... ....---- .--..... .-.--..- .------- .....-"
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/umf-8 -u '.-- .... .- - / .... .- - .... / --. --- -.. / .-- .-. --- ..- --. .... - / -..... ....---- .--..... .-.--..- .------- .....-'`
WHAT HATH GOD WROUGHT 🦀
```

