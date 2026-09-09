<!--
SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami

SPDX-License-Identifier: GPL-3.0-or-later
-->

# IPTMNet API Client

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/AliSajid/iptmnet_client)
![GitHub tag (latest SemVer)](https://img.shields.io/github/v/tag/AliSajid/iptmnet_client)
[![Continuous integration](https://github.com/AliSajid/hellorltk/actions/workflows/ci.yaml/badge.svg?branch=main&event=push)](https://github.com/AliSajid/iptmnet_client/actions/workflows/ci.yaml)

This project aims to develop a small cross-platform command line interface (CLI) for the IPTMNet API. The primary purpose of this is to be able to search the IPTMNet API for a given query and return the results in a format that is easy to read and understand. Additionally, this can be used to process a list of genes, kinases or phosphosites in parallel.

## Builds

| Platform | Rust Version                                        | Status                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| -------- | --------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Linux    | stable <br/> beta <br/> nightly <br/> MSRV (1.86.0) | ![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/ffc7577cb1032996417a2c97f8715009/raw/ubuntu-stable.json) <br/> ![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/ffc7577cb1032996417a2c97f8715009/raw/ubuntu-beta.json) <br/> ![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/ffc7577cb1032996417a2c97f8715009/raw/ubuntu-nightly.json) <br/> ![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/ffc7577cb1032996417a2c97f8715009/raw/ubuntu-msrv.json)     |
| Windows  | stable <br/> beta <br/> nightly <br/> MSRV (1.86.0) | ![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/ffc7577cb1032996417a2c97f8715009/raw/windows-stable.json) <br/> ![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/ffc7577cb1032996417a2c97f8715009/raw/windows-beta.json) <br/> ![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/ffc7577cb1032996417a2c97f8715009/raw/windows-nightly.json) <br/> ![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/ffc7577cb1032996417a2c97f8715009/raw/windows-msrv.json) |
| macOS    | stable <br/> beta <br/> nightly <br/> MSRV (1.86.0) | ![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/ffc7577cb1032996417a2c97f8715009/raw/macos-stable.json) <br/> ![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/ffc7577cb1032996417a2c97f8715009/raw/macos-beta.json) <br/> ![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/ffc7577cb1032996417a2c97f8715009/raw/macos-nightly.json) <br/> ![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/ffc7577cb1032996417a2c97f8715009/raw/macos-msrv.json)         |

## Current Status

As of the start of this project right now, my focus is on implementing the `/search` endpoint. This endpoint is the primary endpoint that is used to search the IPTMNet API for a given enzyme or substrate. The goal for the v1.0.0 is to achieve complete parity with the [IPTMNet API](https://research.bioinformatics.udel.edu/iptmnet/about/api).

## Features

- **Search Endpoint**: Query the IPTMNet API for enzymes, substrates, and proteins
- **Batch Operations**: Process multiple queries in parallel using batch endpoints
- **Multiple PTM Types**: Support for various post-translational modifications (phosphorylation, acetylation, ubiquitination, etc.)
- **Organism Filtering**: Filter results by taxon IDs
- **Role Filtering**: Filter by enzyme, substrate, or both
- **Cross-Platform**: Works on Linux, Windows, and macOS

## Installation

### From Source

```bash
git clone https://github.com/AliSajid/iptmnet_client.git
cd iptmnet_client
cargo install --path .
```

### Using Cargo

```bash
cargo install --git https://github.com/AliSajid/iptmnet_client.git
```

## Usage

### Basic Search

Search for enzymes or substrates:

```bash
iptmnet search "EGFR"
```

### Search with Filters

Search with specific parameters:

```bash
iptmnet search "EGFR" \
  --item-type protein-gene-name \
  --role enzyme \
  --ptm-type Phosphorylation \
  --organism 9606
```

### Batch Operations

Query multiple PTM enzymes in parallel:

```bash
iptmnet batch-enzymes \
  --param substrate_ac=Q9Y6X9 \
  --param site_residue=S \
  --param site_position=1068 \
  --param substrate_ac=P04637 \
  --param site_residue=T \
  --param site_position=38
```

Query multiple PTM protein-protein interactions in parallel:

```bash
iptmnet batch-ppi \
  --param substrate_ac=Q9Y6X9 \
  --param site_residue=S \
  --param site_position=1068 \
  --param substrate_ac=P04637 \
  --param site_residue=T \
  --param site_position=38
```

## API Parameters

The IPTMNet API supports these query parameters:

- `search_term` (required): The term to search for
- `term_type` (required): "all", "uniprot-id", "protein-gene-name", or "pmid"
- `role` (required): "enzyme", "substrate", or "both"/"either"
- `ptm_type` (optional): Array of PTM types (phosphorylation, acetylation, ubiquitination, etc.)
- `organism` (optional): Taxon IDs to filter by
- `start_index`, `end_index` (optional): Pagination controls
- `paginate` (optional): "true" or "false"

Supported PTM types include: Acetylation, C-Glycosylation, Myristoylation, Ubiquitination, N-Glycosylation, S-Glycosylation, Phosphorylation, S-Nitrosylation, O-Glycosylation, Methylation, Sumoylation

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

- [The IPTMNet website](https://research.bioinformatics.udel.edu/iptmnet/).
- [The IPTMNet API Specification](https://research.bioinformatics.udel.edu/iptmnet/api/doc/).
- [The IPTMNet Paper](https://academic.oup.com/nar/article/46/D1/D542/4626766).
