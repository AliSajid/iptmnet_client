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

use crate::models::Protein;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Default, Clone, Eq, PartialEq, Debug, Deserialize)]
pub struct ProteinVec {
    proteins: Vec<Protein>,
    num_results: usize,
}

#[allow(dead_code)]
impl ProteinVec {
    fn new(proteins: Vec<Protein>) -> Self {
        let num_results = proteins.len();
        Self {
            proteins,
            num_results,
        }
    }

    fn add_protein(&mut self, protein: Protein) {
        self.proteins.push(protein);
        self.num_results += 1;
    }

    fn get_proteins(&self) -> &Vec<Protein> {
        &self.proteins
    }

    fn get_num_results(&self) -> usize {
        self.num_results
    }
}

impl Display for ProteinVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Protein List: {} results", self.num_results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Organism;

    #[test]
    fn test_protein_vec() {
        let mut pv = ProteinVec::default();
        let human = Organism::new("Homo Sapiens", "9606", "Human");
        assert_eq!(pv.get_num_results(), 0);
        let p = Protein::new(
            String::from("PK1IP_HUMAN"),
            0,
            human,
            false,
            Vec::new(),
            0,
            0,
            false,
            false,
            String::from("PK1IP_HUMAN"),
            0,
            String::from("PK1IP_HUMAN"),
            0,
            String::from("PK1IP_HUMAN"),
        );
        pv.add_protein(p);
        assert_eq!(pv.get_num_results(), 1);
    }

    #[test]
    fn test_protein_vec_new() {
        let human = Organism::new("Homo Sapiens", "9606", "Human");
        let p = Protein::new(
            String::from("PK1IP_HUMAN"),
            0,
            human,
            false,
            Vec::new(),
            0,
            0,
            false,
            false,
            String::from("PK1IP_HUMAN"),
            0,
            String::from("PK1IP_HUMAN"),
            0,
            String::from("PK1IP_HUMAN"),
        );
        let pv = ProteinVec::new(vec![p]);
        assert_eq!(pv.get_num_results(), 1);
    }

    #[test]
    fn test_protein_vec_get_proteins() {
        let human = Organism::new("Homo Sapiens", "9606", "Human");
        let p = Protein::new(
            String::from("PK1IP_HUMAN"),
            0,
            human,
            false,
            Vec::new(),
            0,
            0,
            false,
            false,
            String::from("PK1IP_HUMAN"),
            0,
            String::from("PK1IP_HUMAN"),
            0,
            String::from("PK1IP_HUMAN"),
        );
        let pv = ProteinVec::new(vec![p]);
        assert_eq!(pv.get_proteins().len(), 1);
    }

    #[test]
    fn test_protein_vec_display() {
        let human = Organism::new("Homo Sapiens", "9606", "Human");
        let p1 = Protein::new(
            String::from("PK1IP_HUMAN"),
            0,
            human,
            false,
            Vec::new(),
            0,
            0,
            false,
            false,
            String::from("PK1IP_HUMAN"),
            0,
            String::from("PK1IP_HUMAN"),
            0,
            String::from("PK1IP_HUMAN"),
        );
        let human = Organism::new("Homo Sapiens", "9606", "Human");
        let p2 = Protein::new(
            String::from("PK2IP_HUMAN"),
            0,
            human,
            false,
            Vec::new(),
            0,
            0,
            false,
            false,
            String::from("PK2IP_HUMAN"),
            0,
            String::from("PK2IP_HUMAN"),
            0,
            String::from("PK2IP_HUMAN"),
        );
        let pv = ProteinVec::new(vec![p1, p2]);
        assert_eq!(format!("{}", pv), "Protein List: 2 results");
    }
}
