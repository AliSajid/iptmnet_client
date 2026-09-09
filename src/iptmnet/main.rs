// Copyright (C) 2022  Ali Sajid Imami
// SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod cli;

use clap::Parser;
use cli::*;
mod helpers;

use helpers::SearchParameters;
use iptmlib::models::{
    BatchPtmEnzymesResponse,
    BatchPtmPpiResponse,
    ProteinVec,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match args.command {
        Some(Commands::Search {
            search,
            item_type,
            role,
            ptm_type,
            organism,
        }) => {
            let baseurl = "https://research.bioinformatics.udel.edu/iptmnet/api/search";

            let parameters = SearchParameters::new(search, item_type, role, ptm_type, organism);

            let client = reqwest::Client::new();
            let request = client.get(baseurl).query(&parameters);

            let response = request.send().await?.json::<ProteinVec>().await?;
            println!("{response}");
        }
        Some(Commands::BatchEnzymes {
            batch_params,
            ptm_type: _,
            organism: _,
        }) => {
            let baseurl = "https://research.bioinformatics.udel.edu/iptmnet/api/batch_ptm_enzymes";

            // Parse batch parameters
            let batch_params: Vec<iptmlib::models::BatchParam> =
                serde_json::from_str(&batch_params)
                    .map_err(|e| format!("Failed to parse batch_params: {}", e))?;

            let client = reqwest::Client::new();
            let request = client.post(baseurl).json(&batch_params);

            let response = request
                .send()
                .await?
                .json::<BatchPtmEnzymesResponse>()
                .await?;
            println!("{response:?}");
        }
        Some(Commands::BatchPpi {
            batch_params,
            ptm_type: _,
            organism: _,
        }) => {
            let baseurl = "https://research.bioinformatics.udel.edu/iptmnet/api/batch_ptm_ppi";

            // Parse batch parameters
            let batch_params: Vec<iptmlib::models::BatchParam> =
                serde_json::from_str(&batch_params)
                    .map_err(|e| format!("Failed to parse batch_params: {}", e))?;

            let client = reqwest::Client::new();
            let request = client.post(baseurl).json(&batch_params);

            let response = request.send().await?.json::<BatchPtmPpiResponse>().await?;
            println!("{response:?}");
        }
        None => {
            // Default to search command
            let baseurl = "https://research.bioinformatics.udel.edu/iptmnet/api/search";

            let parameters = SearchParameters::new(
                String::new(),
                crate::helpers::ItemType::All,
                crate::helpers::Role::Both,
                None,
                None,
            );

            let client = reqwest::Client::new();
            let request = client.get(baseurl).query(&parameters);

            let response = request.send().await?.json::<ProteinVec>().await?;
            println!("{response}");
        }
    }

    Ok(())
}
