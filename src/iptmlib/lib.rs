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
// use models::Organism;
use models::Protein;
// use std::collections::HashMap;

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
