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

use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Serialize, Deserialize)]
pub enum ItemType {
    All,
    #[clap(name = "uniprot-id")]
    UniProtID,
    ProteinGeneName,
    PMID,
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Serialize, Deserialize)]
pub enum PtmType {
    Acetylation,
    CGlycosylation,
    NGlycosylation,
    OGlycosylation,
    SGlycosylation,
    Methylation,
    Myristoylation,
    SNitrosylation,
    Phosphorylation,
    Sumoylation,
    Ubiquitination,
}

impl fmt::Display for PtmType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PtmType::CGlycosylation => write!(f, "C-Glycosylation"),
            PtmType::NGlycosylation => write!(f, "N-Glycosylation"),
            PtmType::SGlycosylation => write!(f, "S-Glycosylation"),
            PtmType::OGlycosylation => write!(f, "O-Glycosylation"),
            PtmType::SNitrosylation => write!(f, "S-Nitrosylation"),
            _ => write!(f, "{self:?}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Serialize, Deserialize)]
pub enum Role {
    Enzyme,
    Substrate,
    Either,
    Both,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Role::Both => write!(f, "Enzyme and Substrate"),
            Role::Either => write!(f, "Enzyme or Substrate"),
            _ => write!(f, "{self:?}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct SearchParameters {
    search_term: String,
    term_type: ItemType,
    role: Role,
    ptm_type: Option<PtmType>,
    organism: Option<String>,
}

impl SearchParameters {
    pub fn new(
        search_term: String,
        term_type: ItemType,
        role: Role,
        ptm_type: Option<PtmType>,
        organism: Option<String>,
    ) -> Self {
        Self {
            search_term,
            term_type,
            role,
            ptm_type,
            organism,
        }
    }
}

impl Default for SearchParameters {
    fn default() -> Self {
        Self {
            search_term: String::new(),
            term_type: ItemType::All,
            role: Role::Both,
            ptm_type: None,
            organism: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_parameters() {
        let search_parameters = SearchParameters::new(
            "test".to_string(),
            ItemType::All,
            Role::Both,
            Some(PtmType::Acetylation),
            Some("test".to_string()),
        );

        assert_eq!(search_parameters.search_term, "test");
        assert_eq!(search_parameters.term_type, ItemType::All);
        assert_eq!(search_parameters.role, Role::Both);
        assert_eq!(search_parameters.ptm_type, Some(PtmType::Acetylation));
        assert_eq!(search_parameters.organism, Some("test".to_string()));
    }

    #[test]
    fn test_search_parameters_default() {
        let search_parameters = SearchParameters::default();

        assert_eq!(search_parameters.search_term, "");
        assert_eq!(search_parameters.term_type, ItemType::All);
        assert_eq!(search_parameters.role, Role::Both);
        assert_eq!(search_parameters.ptm_type, None);
        assert_eq!(search_parameters.organism, None);
    }

    #[test]
    fn test_item_type_display() {
        assert_eq!(format!("{}", ItemType::All), "All");
        assert_eq!(format!("{}", ItemType::UniProtID), "UniProtID");
        assert_eq!(format!("{}", ItemType::ProteinGeneName), "ProteinGeneName");
        assert_eq!(format!("{}", ItemType::PMID), "PMID");
    }

    #[test]
    fn test_ptm_type_display() {
        assert_eq!(PtmType::CGlycosylation.to_string(), "C-Glycosylation");
        assert_eq!(PtmType::NGlycosylation.to_string(), "N-Glycosylation");
        assert_eq!(PtmType::SGlycosylation.to_string(), "S-Glycosylation");
        assert_eq!(PtmType::OGlycosylation.to_string(), "O-Glycosylation");
        assert_eq!(PtmType::SNitrosylation.to_string(), "S-Nitrosylation");
    }

    #[test]
    fn test_role_display() {
        assert_eq!(Role::Both.to_string(), "Enzyme and Substrate");
        assert_eq!(Role::Either.to_string(), "Enzyme or Substrate");
    }

    #[test]
    fn test_item_type_serde() {
        let item_type = ItemType::All;
        let serialized = serde_json::to_string(&item_type).unwrap();
        let deserialized: ItemType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(item_type, deserialized);
    }
}
