# rust-classes

This repository contains resources to Rust classes, a brief introduction to the Rust programming language.

## Preconditions

### First Time Installation

We assume you are running Linux or MacOs and you are familiar with another programming language
like C++, Java, Python or Golang.

To run the code examples you have to install Rust and cargo following the instructions on the
official Rust site: [T1].

(NOTE: cargo [T2] is the Rust package manager you need daily to make a Rust package - so called crate -,
or to download your package's dependencies from the Rust official package repository crates.io [T3])

Clippy [T4] is a lint tool to catch a common mistake and is highly recommended to use daily to improve your Rust code.

Rustfmt [T5] helps to keep your code base consistently formatted and is highly recommended for every editor you use.

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    rustup component add clippy
    cargo install rustfmt

If you installed Rust earlier, verify you have recent updates installed:

    rustup update
    cargo --version

(At the time of writing: cargo 1.43.0 (3532cf738 2020-03-17))

Additionally, you should also read the Standard Rust Book [R1].

### Development Environment

There are many editors around like vim/emacs, but we prefer using an IDE with debugging options.

Step 1: If you have not installed IDEA before, feel free to use the new Toolbox App [T6] from Jetbrains to install IntelliJ.

Step 2: Toolbox App ```Settings: Enable Shell Scripts``` and add the selected directory to your shell $PATH

Step 3: Toolbox App / Intellij IDEA / ```Settings: Shell Script: idea``` (enabled)

Step 4: Open IntelliJ and install the Rust plugin under ```Preferences / Plugins```

Step 5: Restart Intellij and configure under ```Preferences / Languages & Frameworks / Rust```
 - Expand declarative macros: Expand with experimental engine

Step 6: Intellij - Configure under ```Preferences / Languages & Frameworks / Rust / Cargo```
 - External Linter: Clippy

Step 7: Configure under ```Preferences / Languages & Frameworks / Rust / Rustfmt```
 - Run rustfmt on Save

Step 8: checkout this git repo in your terminal and open ```idea .```

# Resources

## Learning the Rust Language

- [R1] https://doc.rust-lang.org/book/
- [R2] https://stevedonovan.github.io/rust-gentle-intro/readme.html (see chapter 10, Rust pain points)
- [R3] https://www.youtube.com/watch?v=zF34dRivLOw (a 2 hours Rust Crash Course on Youtube)

## Tools

- [T1] https://www.rust-lang.org/learn/get-started (Install and Learn Rust)
- [T2] https://github.com/rust-lang/rust-clippy
- [T3] https://doc.rust-lang.org/cargo/
- [T4] https://crates.io/
- [T5] https://github.com/rust-lang/rustfmt
- [T6] https://www.jetbrains.com/toolbox-app/
