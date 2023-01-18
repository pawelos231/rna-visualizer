use rnalib::AminoString;

fn main() {
	let mut buffer = String::new();
	std::io::stdin()
		.read_line(&mut buffer)
		.expect("Coś poszło nie tak!");
	buffer = buffer.trim().into();

	let amino_strings = AminoString::parse(&buffer);
	for amino in amino_strings {
		println!("{amino}");
		let vector_of_proteins = amino.get_proteins();
		if vector_of_proteins.len() == 0 {
			println!("[] - Nie udało się znaleźć zadnych białek :(")
		};
		for protein in vector_of_proteins {
			println!("{protein}");
		}
		println!()
	}
}
