# cliparser

[![crates.io](https://img.shields.io/crates/v/cliparser.svg)](https://crates.io/crates/cliparser) [![CI](https://github.com/sagiegurari/cliparser/workflows/CI/badge.svg?branch=master)](https://github.com/sagiegurari/cliparser/actions) [![codecov](https://codecov.io/gh/sagiegurari/cliparser/branch/master/graph/badge.svg)](https://codecov.io/gh/sagiegurari/cliparser)<br>
[![license](https://img.shields.io/crates/l/cliparser.svg)](https://github.com/sagiegurari/cliparser/blob/master/LICENSE) [![Libraries.io for GitHub](https://img.shields.io/librariesio/github/sagiegurari/cliparser.svg)](https://libraries.io/cargo/cliparser) [![Documentation](https://docs.rs/cliparser/badge.svg)](https://docs.rs/crate/cliparser/) [![downloads](https://img.shields.io/crates/d/cliparser.svg)](https://crates.io/crates/cliparser)<br>
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)

> Simple command line parser.

* [Overview](#overview)
* [Usage](#usage)
* [Installation](#installation)
* [API Documentation](https://sagiegurari.github.io/cliparser/)
* [Contributing](.github/CONTRIBUTING.md)
* [Release History](CHANGELOG.md)
* [License](#license)

<a name="overview"></a>
## Overview
This library provide a very simple API to parse command line arguments.<br>
It will not cover all possible use cases in order to ensure a simple thin API.

<a name="usage"></a>
## Usage
Simply include the library and invoke the get function to pull all info as follows:

<!--{ "examples/example.rs" | lines: 3 | code: rust }-->
```rust
fn main() {
}
```
<!--{ end }-->

<a name="installation"></a>
## Installation
In order to use this library, just add it as a dependency:

```ini
[dependencies]
cliparser = "^0.3.2"
```

## API Documentation
See full docs at: [API Docs](https://sagiegurari.github.io/cliparser/)

## Contributing
See [contributing guide](.github/CONTRIBUTING.md)

<a name="history"></a>
## Release History

See [Changelog](CHANGELOG.md)

<a name="license"></a>
## License
Developed by Sagie Gur-Ari and licensed under the Apache 2 open source license.
