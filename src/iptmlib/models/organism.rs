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

use serde::Deserialize;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Organism {
    pub species: String,
    pub taxon_code: u32,
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
    pub fn new(species: String, taxon_code: u32, common_name: String) -> Self {
        Organism {
            species,
            taxon_code,
            common_name,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_organism_initialization() {
        let org = Organism::new(String::from("Homo Sapiens"), 9606, String::from("Human"));

        assert_eq!(org.species, "Homo Sapiens");
        assert_eq!(org.taxon_code, 9606);
        assert_eq!(org.common_name, "Human")
    }

    #[test]
    fn test_print() {
        let org = Organism::new(String::from("Homo Sapiens"), 9606, String::from("Human"));

        assert_eq!(format!("{}", org), "Human - 9606 (Homo Sapiens)")
    }
}
