use std::{
	collections::BTreeMap,
	sync::{
		atomic::{AtomicBool, AtomicU32, Ordering},
		Arc, Mutex,
	},
	thread::{self},
};

use crate::{Codon, Nucleotide, Protein, ProteinMap};

use super::key::Key;

pub struct ThreadedProteinLoader {
	result: Arc<Mutex<Option<BTreeMap<Key, Protein>>>>,
	flags: [Arc<AtomicBool>; 3],
	progress: [Arc<AtomicU32>; 3],
	stride_len: u32,
}

impl ThreadedProteinLoader {
	pub fn start(&mut self, source: String) {
		self.reset();

		self.stride_len = source.len() as u32;
		let source = Arc::new(source);
		let target = self.result.clone();

		let s1 = source.clone();
		let s2 = source.clone();
		let s3 = source;

		let t1 = target.clone();
		let t2 = target.clone();
		let t3 = target;

		let f1 = self.flags[0].clone();
		let f2 = self.flags[1].clone();
		let f3 = self.flags[2].clone();

		let p1 = self.progress[0].clone();
		let p2 = self.progress[1].clone();
		let p3 = self.progress[2].clone();

		thread::spawn(move || Self::load_skip(s1, t1, f1, p1, 0));
		thread::spawn(move || Self::load_skip(s2, t2, f2, p2, 1));
		thread::spawn(move || Self::load_skip(s3, t3, f3, p3, 2));
	}

	pub fn take(&mut self) -> Option<ProteinMap> {
		if !self.is_ready() {
			return None;
		}
		let lock = self.result.try_lock();
		match lock {
			Ok(mut guard) => {
				let map = guard.replace(BTreeMap::new());
				Some(ProteinMap::from(map.unwrap()))
			}
			Err(_) => None,
		}
	}

	pub fn is_ready(&self) -> bool {
		self.flags[0].load(Ordering::Relaxed)
			&& self.flags[1].load(Ordering::Relaxed)
			&& self.flags[2].load(Ordering::Relaxed)
	}

	pub fn get_progress(&self) -> f32 {
		let total = self.stride_len as f32 / 3.0;
		let p1 = self.progress[0].load(Ordering::Relaxed) / 3;
		let p2 = self.progress[0].load(Ordering::Relaxed) / 3;
		let p3 = self.progress[0].load(Ordering::Relaxed) / 3;
		(p1 + p2 + p3) as f32 / total
	}

	fn reset(&mut self) {
		self.result = Arc::new(Mutex::new(Some(BTreeMap::new())));

		self.flags[0].store(false, Ordering::Relaxed);
		self.flags[1].store(false, Ordering::Relaxed);
		self.flags[2].store(false, Ordering::Relaxed);

		self.progress[0].store(0, Ordering::Relaxed);
		self.progress[1].store(0, Ordering::Relaxed);
		self.progress[2].store(0, Ordering::Relaxed);
	}

	fn load_skip(
		source: Arc<String>,
		target: Arc<Mutex<Option<BTreeMap<Key, Protein>>>>,
		flag: Arc<AtomicBool>,
		progress: Arc<AtomicU32>,
		skip: usize,
	) {
		const SPACE: u8 = b' ';

		let mut result = BTreeMap::new();
		let mut current = Vec::with_capacity(30000);
		let mut current_str = String::with_capacity(30000);
		let mut protein = false;

		let mut iter = source
			.as_bytes()
			.iter()
			.filter(|&&x| x != SPACE)
			.map(|&x| Nucleotide::parse_raw(x).unwrap());

		for _ in 0..skip {
			iter.next();
		}

		while let (Some(a), Some(b), Some(c)) = (iter.next(), iter.next(), iter.next()) {
			let codon = Codon::new(a, b, c);

			if protein && codon == Codon::STOP {
				if !current.is_empty() {
					current.shrink_to_fit();
					current_str.shrink_to_fit();
					result.insert(Key(current_str), Protein::from(current));
					current = Vec::with_capacity(30000);
					current_str = String::with_capacity(30000);
				}
				protein = false;
			}

			if protein {
				current_str.push(codon.get_acid_shorthand());
				current.push(codon);
			}

			if codon == Codon::start() {
				protein = true;
			}

			progress.fetch_add(1, Ordering::Relaxed);
		}

		if let Ok(mut target) = target.lock() {
			match target.as_mut() {
				Some(x) => x.append(&mut result),
				None => (),
			}
		}

		flag.store(true, Ordering::Relaxed);
	}
}

impl Default for ThreadedProteinLoader {
	fn default() -> Self {
		Self {
			result: Arc::new(Mutex::new(Some(BTreeMap::new()))),
			flags: [
				Arc::new(AtomicBool::new(false)),
				Arc::new(AtomicBool::new(false)),
				Arc::new(AtomicBool::new(false)),
			],
			progress: [
				Arc::new(AtomicU32::new(0)),
				Arc::new(AtomicU32::new(0)),
				Arc::new(AtomicU32::new(0)),
			],
			stride_len: 0,
		}
	}
}
