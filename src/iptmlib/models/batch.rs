// Copyright (C) 2022  Ali Sajid Imami
// SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{
    Deserialize,
    Serialize,
};

/// Represents a batch parameter for PTM enzyme queries
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchParam {
    /// Substrate UniProt accession
    pub substrate_ac:  String,
    /// PTM site residue (e.g., "S", "T", "Y")
    pub site_residue:  String,
    /// PTM site position
    pub site_position: String,
}

impl BatchParam {
    /// Creates a new BatchParam with the given parameters
    pub fn new(substrate_ac: String, site_residue: String, site_position: String) -> Self {
        Self {
            substrate_ac,
            site_residue,
            site_position,
        }
    }
}

/// Represents a PTM enzyme result from batch query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResultPtmEnzyme {
    /// UniProt accession
    pub uniprot_id: String,
    /// Enzyme name
    pub name:       String,
}

impl BatchResultPtmEnzyme {
    /// Creates a new BatchResultPtmEnzyme with the given parameters
    pub fn new(uniprot_id: String, name: String) -> Self {
        Self { uniprot_id, name }
    }
}

/// Represents a substrate result from batch query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResultSubstrate {
    /// UniProt accession
    pub uniprot_id: String,
    /// Substrate name
    pub name:       String,
}

impl BatchResultSubstrate {
    /// Creates a new BatchResultSubstrate with the given parameters
    pub fn new(uniprot_id: String, name: String) -> Self {
        Self { uniprot_id, name }
    }
}

/// Represents a source of PTM information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    /// Source name
    pub name: String,
    /// Source URL
    pub url:  String,
}

impl Source {
    /// Creates a new Source with the given parameters
    pub fn new(name: String, url: String) -> Self {
        Self { name, url }
    }
}

/// Represents a PTM enzyme batch result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResultPtmEnzymes {
    /// PTM type (e.g., "Phosphorylation", "Acetylation")
    pub ptm_type:      String,
    /// Substrate information
    pub substrate:     BatchResultSubstrate,
    /// PTM site
    pub site:          String,
    /// PTM site position
    pub site_position: String,
    /// PTM enzyme information
    pub ptm_enzyme:    BatchResultPtmEnzyme,
    /// Score (confidence)
    pub score:         i32,
    /// Sources of PTM information
    pub source:        Vec<Source>,
    /// PubMed IDs
    pub pmids:         i32,
}

impl BatchResultPtmEnzymes {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ptm_type: String,
        substrate: BatchResultSubstrate,
        site: String,
        site_position: String,
        ptm_enzyme: BatchResultPtmEnzyme,
        score: i32,
        source: Vec<Source>,
        pmids: i32,
    ) -> Self {
        Self {
            ptm_type,
            substrate,
            site,
            site_position,
            ptm_enzyme,
            score,
            source,
            pmids,
        }
    }
}

/// Represents a PTM PPI batch result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResultPtmPpi {
    /// PTM type (e.g., "Phosphorylation", "Acetylation")
    pub ptm_type:         String,
    /// Substrate information
    pub substrate:        BatchResultSubstrate,
    /// PTM site
    pub site:             String,
    /// PTM site position
    pub site_position:    String,
    /// Interacting protein (PTM enzyme)
    pub interactant:      BatchResultPtmEnzyme,
    /// Association type
    pub association_type: String,
    /// Score (confidence)
    pub score:            i32,
    /// Sources of PTM information
    pub source:           Vec<Source>,
    /// PubMed IDs
    pub pmids:            i32,
}

impl BatchResultPtmPpi {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ptm_type: String,
        substrate: BatchResultSubstrate,
        site: String,
        site_position: String,
        interactant: BatchResultPtmEnzyme,
        association_type: String,
        score: i32,
        source: Vec<Source>,
        pmids: i32,
    ) -> Self {
        Self {
            ptm_type,
            substrate,
            site,
            site_position,
            interactant,
            association_type,
            score,
            source,
            pmids,
        }
    }
}

/// Represents a batch PTM enzymes response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchPtmEnzymesResponse {
    /// Array of batch results
    pub results: Vec<BatchResultPtmEnzymes>,
}

impl BatchPtmEnzymesResponse {
    /// Creates a new BatchPtmEnzymesResponse with the given results
    pub fn new(results: Vec<BatchResultPtmEnzymes>) -> Self {
        Self { results }
    }
}

/// Represents a batch PTM PPI response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchPtmPpiResponse {
    /// Array of batch results
    pub results: Vec<BatchResultPtmPpi>,
}

impl BatchPtmPpiResponse {
    /// Creates a new BatchPtmPpiResponse with the given results
    pub fn new(results: Vec<BatchResultPtmPpi>) -> Self {
        Self { results }
    }
}
