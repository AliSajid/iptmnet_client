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

use std::fmt::{Display, Formatter, Result};

use serde_aux::prelude::*;

use serde::Deserialize;

/// # The Organism Struct
///
/// The `Organism` Struct encodes the representation of a given organism in the iptmnet
/// database.
///
/// It contains the same information as the one available in the iPTMNet API
///
/// ## Example:
///
/// ### Using the constructor method:
///```
/// use iptmlib::models::organism::Organism;
/// let organism_fun = Organism::new("Homo sapiens", "9606", "Human");
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Organism {
    /// The Species of the organism.
    ///
    /// Example: "Homo sapiens"
    pub species: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    /// The Taxon Code for the organism.
    ///
    /// This taxon code is the one that is assigned by InterPro/Uniprot.
    /// The list can be viewed at <http://www.ebi.ac.uk/interpro/taxonomy/uniprot>
    ///
    /// Example: 9606 for Homo sapiens
    pub taxon_code: u32,

    /// The common name of the organism, if it exists.
    ///
    /// This common name is pulled from the same InterPro/UniProt database.
    ///
    /// Example: Human
    pub common_name: String,
}

impl Display for Organism {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{} - {} ({})",
            self.common_name, self.taxon_code, self.species
        )
    }
}

impl Organism {
    pub fn new(species: &str, taxon_code: &str, common_name: &str) -> Self {
        let taxon_code_parsed: u32 = taxon_code.parse().unwrap();
        Organism {
            species: species.to_string(),
            taxon_code: taxon_code_parsed,
            common_name: common_name.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_organism_initialization() {
        let org = Organism::new("Homo Sapiens", "9606", "Human");

        assert_eq!(org.species, "Homo Sapiens");
        assert_eq!(org.taxon_code, 9606);
        assert_eq!(org.common_name, "Human")
    }

    #[test]
    fn test_print() {
        let org = Organism::new("Homo Sapiens", "9606", "Human");

        assert_eq!(format!("{}", org), "Human - 9606 (Homo Sapiens)")
    }
}
