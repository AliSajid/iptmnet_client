// iptmnet A CLI interface to the IPTMNet Rest API
// Copyright (C) 2022  Ali Sajid Imami
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod cli;

use clap::Parser;
use cli::*;
mod helpers;

use helpers::SearchParameters;
use iptmlib::models::protein::Protein;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Cli::parse();

    println!("Query: {}", args.search);
    println!("Item Type: {}", args.item_type);
    println!("Role: {}", args.role);
    match &args.ptm_type {
        Some(ptm_type) => println!("PTM Type: {}", ptm_type),
        None => println!("PTM Type: All"),
    }
    match &args.organism {
        Some(organism) => {
            println!("Organism: {}", organism);
        }
        None => println!("Organism: All"),
    }

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

    let response = request.send().await?.json::<Vec<Protein>>().await?;

    println!("{:#?}", response);

    Ok(())
}
