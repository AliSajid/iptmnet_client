
# IPTMNet API Client

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/AliSajid/iptmnet_client)
![GitHub tag (latest SemVer)](https://img.shields.io/github/v/tag/AliSajid/iptmnet_client)
[![Continuous integration](https://github.com/AliSajid/hellorltk/actions/workflows/ci.yaml/badge.svg?branch=main&event=push)](https://github.com/AliSajid/iptmnet_client/actions/workflows/ci.yaml)

This project aims to develop a small cross-platform command line interface (CLI) for the IPTMNet API. The primary purpose of this is to be able to search the IPTMNet API for a given query and return the results in a format that is easy to read and understand. Additionally, this can be used to process a list of genes, kinases or phosphosites in parallel.

## Builds

| Platform | Rust Version |Status |
| -------- | ------ | ------ |
| Linux    | stable <br/> beta <br/> nightly <br/> MSRV (1.56.1) | ![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/ffc7577cb1032996417a2c97f8715009/raw/ubuntu-stable.json) <br/> ![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/ffc7577cb1032996417a2c97f8715009/raw/ubuntu-beta.json) <br/> ![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/ffc7577cb1032996417a2c97f8715009/raw/ubuntu-nightly.json) <br/> ![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/ffc7577cb1032996417a2c97f8715009/raw/ubuntu-msrv.json) |
| Windows  | stable <br/> beta <br/> nightly <br/> MSRV (1.56.1) | ![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/ffc7577cb1032996417a2c97f8715009/raw/windows-stable.json) <br/> ![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/ffc7577cb1032996417a2c97f8715009/raw/windows-beta.json) <br/> ![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/ffc7577cb1032996417a2c97f8715009/raw/windows-nightly.json) <br/> ![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/ffc7577cb1032996417a2c97f8715009/raw/windows-msrv.json) |
| macOS    | stable <br/> beta <br/> nightly <br/> MSRV (1.56.1) | ![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/ffc7577cb1032996417a2c97f8715009/raw/macos-stable.json) <br/> ![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/ffc7577cb1032996417a2c97f8715009/raw/macos-beta.json) <br/> ![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/ffc7577cb1032996417a2c97f8715009/raw/macos-nightly.json) <br/> ![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/ffc7577cb1032996417a2c97f8715009/raw/macos-msrv.json) |

## Current Status

As of the start of this project right now, my focus is on implementing the `/search` endpoint. This endpoint is the primary endpoint that is used to search the IPTMNet API for a given enzyme or substrate. The goal for the v1.0.0 is to achieve complete parity with the [IPTMNet API](https://research.bioinformatics.udel.edu/iptmnet/about/api).

## License

This project is licensed under the GNU General Public License v3.0.

```text
    iptmnet A CLI interface to the IPTMNet Rest API
    Copyright (C) 2022  Ali Sajid Imami

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
```

## Related

* The IPTMNet website can be found [here](https://research.bioinformatics.udel.edu/iptmnet/).
* The IPTMNet API Specification can be found [here](https://research.bioinformatics.udel.edu/iptmnet/api/doc/).
* The IPTMNet Paper can be found [here](https://academic.oup.com/nar/article/46/D1/D542/4626766).
