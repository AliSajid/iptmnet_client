// Copyright (C) 2022  Ali Sajid Imami
// SPDX-FileCopyrightText: 2022 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::{
    Parser,
    Subcommand,
};

use crate::helpers::{
    ItemType,
    PtmType,
    Role,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Search for enzymes or substrates
    Search {
        /// Search string
        ///
        /// This argument specifies the specific string to search for in the
        /// IPTMNet database. You can specify gene or protein names,
        /// pubmed IDs or UniProt protein ID.
        #[clap(short, long, value_parser, value_name = "QUERY")]
        search: String,

        /// Limit search to specific identifier types
        ///
        /// This argument specifies the type of the argument specified in
        /// --search. The default "All" searches through all the
        /// identifiers.
        ///
        /// Possible values include:
        /// 1. All (default)
        /// 2. uniprot-id
        /// 3. protein-gene-name
        /// 4. pmid
        #[clap(
            short,
            long,
            value_parser,
            value_name = "ITEM_TYPE",
            default_value = "all"
        )]
        item_type: ItemType,

        /// Limit search to specific types of Post-translational Modifications
        /// (PTM)
        ///
        /// This argument allows you to specify which type of PTM you are
        /// looking for. The default is to include all.
        ///
        /// Possible values include:
        /// 1. acetylation
        /// 2. c-glycosylation
        /// 3. n-glycosylation
        /// 4. o-glycosylation
        /// 5. s-glycosylation
        /// 6. methylation
        /// 7. myristoylation
        /// 8. phosphorylation
        /// 9. sumoylation
        /// 10. ubiquitination
        #[clap(short, long, value_parser, value_name = "PTM_TYPE")]
        ptm_type: Option<PtmType>,

        /// The role for the search term
        ///
        /// This argument allows you subset the search results based on the role
        /// it plays. A role can be as Enzyme, Substrate, or Enzyme or
        /// Substrate both The default is to include use both.
        ///
        /// Possible values include:
        /// 1. enzyme: Look for only enzymes
        /// 2. substrate: Look for only substrates
        /// 3. both: Look for both enzymes and substrates (default)
        /// 4. either: Look for either enzymes or substrates or both
        #[clap(short, long, value_parser, value_name = "ROLE")]
        role: Role,

        /// The organism to restrict the search to
        ///
        /// This argument allows you to subset the search results based on the
        /// organism of interest. This argument accepts common names of the
        /// organisms. The default is empty, which includes all organisms in the
        /// database.
        #[clap(long, value_parser, value_name = "ORGANISM")]
        organism: Option<String>,
    },

    /// Batch query for PTM enzymes
    BatchEnzymes {
        /// Batch parameters (JSON array)
        ///
        /// This argument specifies the batch parameters for PTM enzyme queries.
        /// Format: [{"substrate_ac": "string", "site_residue": "string",
        /// "site_position": "string"}, ...]
        #[clap(short, long, value_parser, value_name = "BATCH_PARAMS")]
        batch_params: String,

        /// Limit search to specific types of Post-translational Modifications
        /// (PTM)
        ///
        /// This argument allows you to specify which type of PTM you are
        /// looking for. The default is to include all.
        ///
        /// Possible values include:
        /// 1. acetylation
        /// 2. c-glycosylation
        /// 3. n-glycosylation
        /// 4. o-glycosylation
        /// 5. s-glycosylation
        /// 6. methylation
        /// 7. myristoylation
        /// 8. phosphorylation
        /// 9. sumoylation
        /// 10. ubiquitination
        #[clap(short, long, value_parser, value_name = "PTM_TYPE")]
        ptm_type: Option<PtmType>,

        /// The organism to restrict the search to
        ///
        /// This argument allows you to subset the search results based on the
        /// organism of interest. This argument accepts common names of the
        /// organisms. The default is empty, which includes all organisms in the
        /// database.
        #[clap(long, value_parser, value_name = "ORGANISM")]
        organism: Option<String>,
    },

    /// Batch query for PTM PPI
    BatchPpi {
        /// Batch parameters (JSON array)
        ///
        /// This argument specifies the batch parameters for PTM PPI queries.
        /// Format: [{"substrate_ac": "string", "site_residue": "string",
        /// "site_position": "string"}, ...]
        #[clap(short, long, value_parser, value_name = "BATCH_PARAMS")]
        batch_params: String,

        /// Limit search to specific types of Post-translational Modifications
        /// (PTM)
        ///
        /// This argument allows you to specify which type of PTM you are
        /// looking for. The default is to include all.
        ///
        /// Possible values include:
        /// 1. acetylation
        /// 2. c-glycosylation
        /// 3. n-glycosylation
        /// 4. o-glycosylation
        /// 5. s-glycosylation
        /// 6. methylation
        /// 7. myristoylation
        /// 8. phosphorylation
        /// 9. sumoylation
        /// 10. ubiquitination
        #[clap(short, long, value_parser, value_name = "PTM_TYPE")]
        ptm_type: Option<PtmType>,

        /// The organism to restrict the search to
        ///
        /// This argument allows you to subset the search results based on the
        /// organism of interest. This argument accepts common names of the
        /// organisms. The default is empty, which includes all organisms in the
        /// database.
        #[clap(long, value_parser, value_name = "ORGANISM")]
        organism: Option<String>,
    },
}
