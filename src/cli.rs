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

use crate::helpers::PtmType;

use crate::helpers::ItemType;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Search string
    ///
    /// This argument specifies the specific string to search for in the IPTMNet database.
    /// You can specify gene or protein names, pubmed IDs or UniProt protein ID.
    #[clap(short, long, value_parser, value_name = "QUERY")]
    pub search: String,

    /// Limit search to specific identifier types
    ///
    /// This argument specifies the type of the argument specified in --search.
    /// The default "All" searches through all the identifiers.
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
        arg_enum,
        default_value = "all"
    )]
    pub item_type: ItemType,

    /// Limit search to specific types of Post-translational Modifications (PTM)
    ///
    /// This argument allows you to specify which type of PTM you are looking for.
    /// The default is to include all.
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
    #[clap(short, long, value_parser, value_name = "PTM_TYPE", arg_enum)]
    pub ptm_type: Option<PtmType>,
}
