// Copyright (C) 2022  Ali Sajid Imami
// SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod models;
#[allow(unused_imports)]
use models::{
    Organism,
    Protein,
};
// use std::collections::HashMap;

pub enum IPTMResultError {
    DeserializingError(serde_json::Error),
    APIRequestError(reqwest::Error),
}

pub enum IPTMResult {
    Error(IPTMResultError),
    ProteinResults {
        num_results: usize,
        results:     Vec<Protein>,
    },
}
