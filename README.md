# KlicK

KlicK – Klimabilanzen an Kläranlagen mit einem Klick

[![License](https://img.shields.io/badge/license-AGPLv3-blue.svg?style=flat)](https://codeberg.org/slowtec/klick/raw/branch/master/LICENSE)

## Getting started

This tool is used at https://klimabilanzklaeranlage.de/

### Cloning the repository

    git clone https://codeberg.org/slowtec/klick
    cd klick

### Install Rust

Install Rust by following the instructions on the
[Getting started](https://www.rust-lang.org/learn/get-started) page.

### Install just

We use [just](https://github.com/casey/just) as a command runner to automate common tasks:

```sh
cargo install just
```

For a list of available tasks (named _recipes_) run the following command:

```sh
just
```

## License

Copyright (c) 2023 [slowtec GmbH](https://slowtec.de)

This project is licensed unter the [AGPLv3 license](https://www.gnu.org/licenses/agpl-3.0.html).
