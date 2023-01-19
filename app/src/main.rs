use rnalib::AminoString;
fn main() {
	let mut buffer = String::new();
	std::io::stdin()
		.read_line(&mut buffer)
		.expect("Coś poszło nie tak!");
	buffer = buffer.trim().into();

	let amino_strings = AminoString::parse(&buffer);
	for amino in amino_strings {
		let proteins = amino.get_proteins();
		println!("{amino}, {} protein(s)", proteins.len());
		proteins.iter().for_each(|x| println!("• {x}"));
		println!();
	}
}
