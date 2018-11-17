[![Build Status][build-img]][build-url] [![Crates.io][crates-io-img]][crates-io-url]

# photoprintit-tools

> A collection of tools for photoprintit software

## Installation

```
cargo install photoprintit-tools
```

## Tools

### `prepare`

*Outputs a list of resources necessary to install the given software.*

```
USAGE:
    photoprintit-tools prepare [FLAGS] [OPTIONS] --client-id <client_id> --hps-version <hps_version> --keyaccid <keyaccid> --locale <locale> --platform <platform>

FLAGS:
    -a, --all        Imply platform `a` (any), include complete set of resources for install
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --client-id <client_id>          Client ID
    -d, --download-server <dl_server>    Downlaod server [default: https://dls.photoprintit.com]
    -h, --hps-version <hps_version>      HPS version
    -k, --keyaccid <keyaccid>            KEYACCID
    -l, --locale <locale>                (full) Locale
    -p, --platform <platform>            Platform (a, l, l64, m, w, w64)
```

## License

MIT licensed, see [`LICENSE`](LICENSE)

[build-img]: https://travis-ci.com/flyingP0tat0/photoprintit-tools.svg?branch=master
[build-url]: https://travis-ci.com/flyingP0tat0/photoprintit-tools
[crates-io-img]: https://img.shields.io/crates/v/photoprintit-tools.svg
[crates-io-url]: https://crates.io/crates/photoprintit-tools
