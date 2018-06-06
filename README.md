# 📦✨  wasm-pack
> your favorite rust -> wasm workflow tool!

[![Build Status](https://travis-ci.org/ashleygwilliams/wasm-pack.svg?branch=master)](https://travis-ci.org/ashleygwilliams/wasm-pack)
[![Build status](https://ci.appveyor.com/api/projects/status/7jjuo5wewu9lyyfi?svg=true)](https://ci.appveyor.com/project/ashleygwilliams/wasm-pack)

this tool seeks to be a one-stop shop for building and working with rust-
generated webassembly that you would like to interop with JavaScript, in the
browser or with Node.js. `wasm-pack` helps you build and publish rust-generated
web assembly to the npm registry to be used alongside any other javascript
package in workflows that you already use, such as [webpack] or [greenkeeper].

[webpack]: https://webpack.js.org/
[greenkeeper]: https://greenkeeper.io/ 

this project is a part of the [rust-wasm] group. you can find more info by
visiting that repo!

[rust-wasm]: https://github.com/rust-lang-nursery/rust-wasm/

![demo](demo.gif)

## 🔮 prerequisities

- [development environment](docs/prerequisites.md)
- [installation and getting started](docs/setup.md)

## 🎙️ commands

- [`init`](docs/init.md): generate an npm wasm pkg from a rustwasm crate
- [`pack`](docs/pack.md): create a tarball of your rustwasm pkg
- [`publish`](docs/publish.md): publish your rustwasm pkg to a registry

### 📝 logging

We generate a `wasm-pack.log` file if `wasm-pack` errors on you, and you can
customize the log verbosity using the verbosity flag.

| Verbosity     | Result                                              |
| ------------- |-----------------------------------------------------|
| -v            | All Info, Warn, and Errors are logged               |
| -vv           | All Debug, Info, Warn, and Errors are logged        |
| -vvv          | All Trace, Debug, Info, Warn, and Errors are logged |

## 👯 contributing

Read our [guide] on getting up and running for developing `wasm-pack`, and
check out our [contribution policy].

[guide]: doc/contributing.md
[contirbution policy]: contributing.md

## ⚡ quickstart guide

1. write a crate in Rust.
2. add `wasm-bindgen` to your `Cargo.toml`:

  ```toml
  [lib]
  crate-type = ["cdylib"]

  [dependencies]
  wasm-bindgen = "0.2"
  ```
3. add this to the top of your `src/lib.rs`:

  ```rust
  #![feature(proc_macro, wasm_import_module, wasm_custom_section)]

  extern crate wasm_bindgen;

  use wasm_bindgen::prelude::*;
  ```

4. annotate your public functions with `#[wasm_bindgen]`, for example:

  ```rust
  #[wasm_bindgen]
  extern {
      fn alert(s: &str);
  }

  #[wasm_bindgen]
  pub fn greet(name: &str) {
      alert(&format!("Hello, {}!", name));
  }
  ```

5. install this tool: `cargo install wasm-pack`
6. run `wasm-pack init`, optionally, pass a path to a dir or a scope (see above for details)
7. this tool generates files in a `pkg` dir
8. to publish to npm, run `wasm-pack publish` (making sure you are logged in with npm)

[rust-wasm/36]: https://github.com/rust-lang-nursery/rust-wasm/issues/36
[wasm-bindgen]: https://github.com/alexcrichton/wasm-bindgen
