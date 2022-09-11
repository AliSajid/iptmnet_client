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

use std::fmt::Display;

use serde::Deserialize;

use crate::models::organism::Organism;

type Synonym = String;

#[derive(Clone, Eq, PartialEq, Debug, Deserialize)]
pub struct Protein {
    enzyme_num: u32,
    enzyme_role: bool,
    gene_name: String,
    iptm_id: String,
    isoforms: u32,
    organism: Organism,
    protein_name: String,
    ptm_dependent_ppi_num: u32,
    ptm_dependent_ppi_role: bool,
    sites: u32,
    substrate_num: u32,
    substrate_role: bool,
    synonyms: Vec<Synonym>,
    uniprot_ac: String,
}

impl Display for Protein {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.gene_name, self.organism.common_name)
    }
}

#[allow(clippy::too_many_arguments)]
impl Protein {
    pub fn new(
        gene_name: String,
        substrate_num: u32,
        organism: Organism,
        substrate_role: bool,
        synonyms: Vec<Synonym>,
        isoforms: u32,
        sites: u32,
        enzyme_role: bool,
        ptm_dependent_ppi_role: bool,
        protein_name: String,
        ptm_dependent_ppi_num: u32,
        iptmid: String,
        enzyme_num: u32,
        uniprot_ac: String,
    ) -> Self {
        Protein {
            gene_name,
            substrate_num,
            organism,
            substrate_role,
            synonyms,
            isoforms,
            sites,
            enzyme_role,
            ptm_dependent_ppi_role,
            protein_name,
            ptm_dependent_ppi_num,
            iptm_id: iptmid,
            enzyme_num,
            uniprot_ac,
        }
    }

    pub fn has_enzyme_role(&self) -> bool {
        self.enzyme_role
    }

    pub fn has_substrate_role(&self) -> bool {
        self.substrate_role
    }
    pub fn has_ptm_dependent_ppi_role(&self) -> bool {
        self.ptm_dependent_ppi_role
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protein_instantiation() {
        let org = Organism::new(
            String::from("Homo sapiens"),
            String::from("9606"),
            String::from("Human"),
        );
        let prot = Protein::new(
            String::from("PAK1IP1"),
            0,
            org,
            true,
            ["PIP1", "WDR84"].map(String::from).to_vec(),
            0,
            18,
            false,
            false,
            String::from("p21-activated protein kinase-interacting protein 1;"),
            0,
            String::from("Q9NWT1"),
            0,
            String::from("PK1IP_HUMAN"),
        );

        assert_eq!(prot.gene_name, "PAK1IP1");
        assert_eq!(prot.substrate_num, 0);
        assert_eq!(prot.organism.common_name, "Human");
        assert!(prot.substrate_role);
        assert_eq!(prot.synonyms[0], "PIP1");
        assert_eq!(prot.synonyms[1], "WDR84");
        assert_eq!(prot.isoforms, 0);
        assert_eq!(prot.sites, 18);
        assert!(!prot.enzyme_role);
        assert!(!prot.ptm_dependent_ppi_role);
        assert_eq!(
            prot.protein_name,
            "p21-activated protein kinase-interacting protein 1;"
        );
        assert_eq!(prot.ptm_dependent_ppi_num, 0);
        assert_eq!(prot.iptm_id, "Q9NWT1");
        assert_eq!(prot.enzyme_num, 0)
    }

    #[test]
    fn test_protein_display() {
        let org = Organism::new(
            String::from("Homo sapiens"),
            String::from("9606"),
            String::from("Human"),
        );
        let prot = Protein::new(
            String::from("PAK1IP1"),
            0,
            org,
            true,
            ["PIP1", "WDR84"].map(String::from).to_vec(),
            0,
            18,
            false,
            false,
            String::from("p21-activated protein kinase-interacting protein 1;"),
            0,
            String::from("Q9NWT1"),
            0,
            String::from("PK1IP_HUMAN"),
        );

        assert_eq!(format!("{}", prot), "PAK1IP1 - Human")
    }

    #[test]

    fn test_protein_getters() {
        let org = Organism::new(
            String::from("Homo sapiens"),
            String::from("9606"),
            String::from("Human"),
        );
        let prot = Protein::new(
            String::from("PAK1IP1"),
            0,
            org,
            true,
            ["PIP1", "WDR84"].map(String::from).to_vec(),
            0,
            18,
            false,
            false,
            String::from("p21-activated protein kinase-interacting protein 1;"),
            0,
            String::from("Q9NWT1"),
            0,
            String::from("PK1IP_HUMAN"),
        );

        assert!(!prot.has_enzyme_role());
        assert!(prot.has_substrate_role());
        assert!(!prot.has_ptm_dependent_ppi_role());
    }
}
