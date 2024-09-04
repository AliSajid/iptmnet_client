// Copyright (C) 2022  Ali Sajid Imami
// SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::fmt::Display;

use serde::{
    Deserialize,
    Serialize,
};

use crate::models::Protein;

#[derive(Default, Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct ProteinVec {
    proteins:    Vec<Protein>,
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
