//! The module that implements [`Counts`]

use crate::Codon;

/// A helper struct that counts how many times
/// a given amino acid appears inside an [`AminoString`].
///
/// This only counts the occurences of amino acids with
/// ionisable side chains, i.e. c, w, y, d, e, k, r, h.
///
/// The internal counters can only be increased or read from,
/// which eliminates a domain of bugs related to accidental
/// mutability.
#[derive(Default, Clone, Copy)]
pub struct Counts {
	c: u16,
	w: u16,
	y: u16,
	d: u16,
	e: u16,
	k: u16,
	r: u16,
	h: u16,
}

impl Counts {
	/// Adds onto the internal counter of the amino acid
	/// encoded by the [`Codon`] passed in.
	///
	/// Does nothing if the amino acid has a neutral side
	/// chain.
	pub fn add(&mut self, codon: &Codon) {
		match codon {
			Codon::C => self.c += 1,
			Codon::W => self.w += 1,
			Codon::Y => self.y += 1,
			Codon::D => self.d += 1,
			Codon::E => self.e += 1,
			Codon::K => self.k += 1,
			Codon::R => self.r += 1,
			Codon::H => self.h += 1,
			_ => (),
		}
	}

	/// Returns the amount of times [`Codon::C`] has
	/// been counted.
	pub const fn get_c(&self) -> u16 {
		self.c
	}

	/// Returns the amount of times [`Codon::W`] has
	/// been counted.
	pub const fn get_w(&self) -> u16 {
		self.w
	}

	/// Returns the amount of times [`Codon::Y`] has
	/// been counted.
	pub const fn get_y(&self) -> u16 {
		self.y
	}

	/// Returns the amount of times [`Codon::D`] has
	/// been counted.
	pub const fn get_d(&self) -> u16 {
		self.d
	}

	/// Returns the amount of times [`Codon::E`] has
	/// been counted.
	pub const fn get_e(&self) -> u16 {
		self.e
	}

	/// Returns the amount of times [`Codon::K`] has
	/// been counted.
	pub const fn get_k(&self) -> u16 {
		self.k
	}

	/// Returns the amount of times [`Codon::R`] has
	/// been counted.
	pub const fn get_r(&self) -> u16 {
		self.r
	}

	/// Returns the amount of times [`Codon::H`] has
	/// been counted.
	pub const fn get_h(&self) -> u16 {
		self.h
	}
}
