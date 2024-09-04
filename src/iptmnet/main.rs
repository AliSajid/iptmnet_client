// Copyright (C) 2022  Ali Sajid Imami
// SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod cli;

use clap::Parser;
use cli::*;
mod helpers;

use helpers::SearchParameters;
use iptmlib::models::ProteinVec;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Cli::parse();

    let baseurl = "https://research.bioinformatics.udel.edu/iptmnet/api/search";

    let parameters = SearchParameters::new(
        args.search,
        args.item_type,
        args.role,
        args.ptm_type,
        args.organism,
    );

    let client = reqwest::Client::new();
    let request = client.get(baseurl).query(&parameters);

    let response = request.send().await?.json::<ProteinVec>().await?;
    // render the response as JSON
    println!("{response}");

    Ok(())
}
