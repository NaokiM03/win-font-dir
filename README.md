# win-font-dir

## About

This library provides the path to the font directory in Windows.

## Motivation

[dirs](https://github.com/dirs-dev/dirs-rs) crate does not support font directory in Windows. \
https://github.com/dirs-dev/dirs-rs/issues/37 \
https://stackoverflow.com/questions/70526705/why-was-the-user-font-directory-not-added-to-the-knownfolder-api

## How to use

```toml
# Cargo.toml

[dependencies]
win-font-dir = "0.1.0"
```

This library has only two methods.
- `fn user_font_dir() -> Some(PathBuf);`
- `fn system_font_dir() -> Some(PathBuf);`

## License

LightFetch is released under the MIT License
