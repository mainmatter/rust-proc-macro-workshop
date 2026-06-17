# Rust Procedural Macros

> Extend the compiler, one token at a time

This workshop will introduce you to Rust's procedural macro system. You will learn how to write derive macros, attribute macros, and function-like macros through a series of test-driven exercises.

This workshop is designed for people who have basic familiarity with Rust as a language and want to learn how to write procedural macros.

> [!NOTE]
> This workshop has been written by [Mainmatter](https://mainmatter.com/rust-consulting/).\
> It's one of the trainings in [our portfolio of Rust workshops](https://mainmatter.com/services/workshops/rust/).\
> Check out our [landing page](https://mainmatter.com/rust-consulting/) if you're looking for Rust consulting or training!

## Getting started

Open [the companion book for this course](https://rust-exercises.com/proc-macros/) in your browser.
Follow the instructions there to get started.

## Requirements

- **Rust** (follow instructions [here](https://www.rust-lang.org/tools/install)).\
  If Rust is already installed on your system, make sure you are running on the latest compiler version (`cargo --version`).\
  If not, update using `rustup update` (or another appropriate command depending on how you installed Rust on your system).
- _(Optional)_ An IDE with Rust autocompletion support.
  We recommend one of the following:
  - [Zed](https://zed.dev/);
  - [RustRover](https://www.jetbrains.com/rust/);
  - [Visual Studio Code](https://code.visualstudio.com) with the [`rust-analyzer`](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension.

## Solutions

You can find the solutions to the exercises in the [`solutions` branch](https://github.com/mainmatter/rust-proc-macro-workshop/tree/solutions) of this repository.

## References

Throughout the workshop, the following resources might turn out to be useful:

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust documentation](https://doc.rust-lang.org/std/) (you can also open the documentation offline with `rustup doc`!)
- [`proc-macro2`'s documentation](https://docs.rs/proc-macro2)
- [`syn`'s documentation](https://docs.rs/syn)
- [`quote`'s documentation](https://docs.rs/quote)
- [`darling`'s documentation](https://docs.rs/darling)
- [The Reference: Procedural Macros](https://doc.rust-lang.org/reference/procedural-macros.html)

# License

Copyright (c) 2026- Mainmatter GmbH (https://mainmatter.com), released under the
[Creative Commons Attribution-NonCommercial 4.0 International license](https://creativecommons.org/licenses/by-nc/4.0/).
