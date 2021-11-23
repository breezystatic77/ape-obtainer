use clap::{App, Arg};
use ipfsapi::IpfsApi;
use std::fs;
use std::path::{Path, PathBuf};

use crate::ape::Ape;

mod ape;

fn download_ape(path: &Path, id: u32, api: &IpfsApi) -> () {
	let path = filename(path, id);
	if Path::new(&path).exists() {
		println!("{} already exists, skipping download.", path);
		return ();
	}

	println!("Obtaining ape #{}...", id);
	let bytes = api
		.cat(&format!(
			"QmeSjSinHpPnmXmspMjwiXyN6zS4E9zccariGR3jxcaWtq/{}",
			id
		))
		.expect("Could not obtain ape data");
	let metadata: Ape = serde_json::from_str(
		&String::from_utf8(bytes.collect())
			.expect("Could not convert ape data to string"),
	)
	.expect("Could not serialize ape data");
	println!("Obtained ape data: {}", metadata);
	println!("Obtaining ape image...");
	let image_hash = &metadata.image.as_str()[7..];
	let image = loop {
		match api.cat(image_hash) {
			Ok(res) => {
				break res.collect::<Vec<u8>>();
			}
			Err(err) => {
				println!("Error obtaining ape image: {}, retrying...", err)
			}
		}
	};

	println!(
		"Obtained {:.2}MB image",
		image.len() as f32 / 1024.0 / 1024.0
	);
	fs::write(&path, &image).expect("Unable to save image");
}

fn filename(path: &Path, id: u32) -> String {
	path
		.join(format!("{:0>4}.png", id))
		.to_str()
		.unwrap()
		.to_owned()
}

fn main() {
	let matches = App::new("Ape Obtainer")
		.author("breezystatic77")
		.about("Obtain BAYC Apes.")
		.arg(
			Arg::with_name("host")
				.short("h")
				.long("host")
				.help("IPFS host name, defaults to 'localhost'")
				.takes_value(true),
		)
		.arg(
			Arg::with_name("port")
				.short("p")
				.long("port")
				.help("IPFS port, defaults to 8080")
				.takes_value(true),
		)
		.arg(
			Arg::with_name("location")
				.short("l")
				.long("location")
				.help("Location to download obtained apes to. defaults to './apes'")
				.takes_value(true),
		)
		.get_matches();
	let host = matches.value_of("host").unwrap_or("localhost");
	let port = matches
		.value_of("part")
		.map(|v| v.parse::<u16>().expect("Port must be a number"))
		.unwrap_or(8080);
	let path = Path::new(matches.value_of("location").unwrap_or("./apes"));

	let api = IpfsApi::new(host, port);

	fs::create_dir_all(path).expect("Unable to create downloads directory.");
	for id in 0..10000 {
		download_ape(path, id, &api);
	}
	println!("All apes obtained, excellent work.");
}
