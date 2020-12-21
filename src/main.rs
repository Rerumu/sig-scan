use std::fs;

struct Signature {
	data: Vec<Option<u8>>,
}

fn to_mask(b: &str) -> Option<u8> {
	if b != "??" {
		let v = u8::from_str_radix(b, 16).unwrap();

		Some(v)
	} else {
		None
	}
}

impl Signature {
	fn new(sig: &str) -> Self {
		let data = sig.split_whitespace().map(to_mask).collect();

		Self { data }
	}

	fn compare(&self, data: &[u8]) -> bool {
		self.data
			.iter()
			.zip(data)
			.all(|(m, v)| m.map_or(true, |w| v == &w))
	}
}

fn sig_scan_data(data: &[u8], sig: &Signature) -> Option<usize> {
	data.windows(sig.data.len()).position(|w| sig.compare(w))
}

fn print_usage() {
	println!("usage: sig-scan [options] [files]");
	println!("  -h | --help             show the help message");
	println!("  -s | --signature [aob]  set the signature to use");
}

fn main() {
	let mut list = std::env::args();
	let mut sig = Signature::new("68 ?? 6C 6C 6F");

	list.next(); // skip first arg

	while let Some(arg) = list.next() {
		match arg.as_str() {
			"-h" | "--help" => {
				print_usage();
			}
			"-s" | "--signature" => {
				let s = list.next().expect("signature not provided");

				sig = Signature::new(s.as_str());
			}
			name => {
				let data = fs::read(name).unwrap();
				let index = sig_scan_data(data.as_ref(), &sig);

				println!("{:?}", index);
			}
		}
	}
}
