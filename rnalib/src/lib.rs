//! A library for working with complex DNA/RNA sequences.
//! Provides structures for efficient data storage and manipulation.
//!
//! Created as a submission for the [Motorola Science Cup](https://science-cup.pl)

mod acid;
mod amino_string;
mod codon;
mod consts;
mod nucleotide;
mod protein;
mod protein_table;

pub use acid::*;
pub use amino_string::*;
pub use codon::*;
pub use nucleotide::*;
pub use protein::*;
pub use protein_table::loader::*;
pub use protein_table::*;
