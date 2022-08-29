use std::fmt;

use clap::ValueEnum;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ItemType {
    All,
    #[clap(name = "uniprot-id")]
    UniProtID,
    ProteinGeneName,
    PMID,
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
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
            _ => write!(f, "{:?}", self),
        }
    }
}
