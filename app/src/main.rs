use rnalib::AminoString;

fn main() {
	let mut buffer = String::new();
	std::io::stdin()
		.read_line(&mut buffer)
		.expect("konsola sie wyjebała");
	buffer = buffer.trim().into();

	let amino_strings = AminoString::parse(&buffer);
	for amino in amino_strings {
		println!("{amino}");
		for protein in amino.get_proteins() {
			println!("{protein}");
		}
		println!()
	}
}
