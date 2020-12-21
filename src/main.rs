use std::fs;

struct Signature {
	data: Vec<(usize, u8)>,
	len: usize,
}

impl Signature {
	fn new(sig: &str) -> Self {
		let mut data = Vec::new();
		let len = sig.split(' ').count();

		for (i, sub) in sig.split(' ').enumerate() {
			if sub == "??" {
				continue;
			}

			let v = u8::from_str_radix(sub, 16).unwrap();

			data.push((i, v));
		}

		Self { data, len }
	}

	fn compare(&self, data: &[u8]) -> bool {
		// avoid bounds checking
		assert!(self.len <= data.len());

		for (i, v) in self.data.iter() {
			if unsafe { data.get_unchecked(*i) } != v {
				return false;
			}
		}

		true
	}
}

fn sig_scan_data(data: &[u8], sig: &Signature) -> Option<usize> {
	for (i, w) in data.windows(sig.len).enumerate() {
		if sig.compare(w) {
			return Some(i);
		}
	}

	None
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
