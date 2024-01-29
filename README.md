# KlicK

KlicK – Klimabilanzen an Kläranlagen mit einem Klick

[![License](https://img.shields.io/badge/license-AGPLv3-blue.svg?style=flat)](https://codeberg.org/slowtec/klick/raw/branch/master/LICENSE)

## Getting started

This tool is used at https://klimabilanzklaeranlage.de/

### Cloning the repository

    git clone https://codeberg.org/slowtec/klick
    cd klick

### Install Rust

On NixOS run:

```sh
nix develop
```

For a list of available tasks (named _recipes_) run the following command:

```sh
just
```

### Deployment

```shell
just build-release
scp target/x86_64-unknown-linux-musl/release/klick klick:/home/klick-app/klick-v0.3.x
ssh klick
systemctl stop klick-app
cp /tmp/klick 
ln -s /home/klick-app/klick-v0.3.x /home/klick-app/klick
systemctl start klick-app
```

## License

Copyright (c) 2023 - 2024 [slowtec GmbH](https://slowtec.de)

This project is licensed unter the [AGPLv3 license](https://www.gnu.org/licenses/agpl-3.0.html).
