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

pub mod models;
use models::protein::Protein;

pub enum IPTMResultError {
    DeserializingError(serde_json::Error),
    APIRequestError(reqwest::Error),
}

pub enum IPTMResult {
    Error(IPTMResultError),
    ProteinResults {
        num_results: usize,
        results: Vec<Protein>,
    },
}

pub fn get_search_data(json_data: &str) -> IPTMResult {
    let deserialized: Result<Vec<Protein>, serde_json::Error> = serde_json::from_str(json_data);

    match deserialized {
        Ok(json) => IPTMResult::ProteinResults {
            num_results: json.len(),
            results: json,
        },
        Err(error) => IPTMResult::Error(IPTMResultError::DeserializingError(error)),
    }
}
