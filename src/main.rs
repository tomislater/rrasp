extern crate argparse;

use argparse::{ArgumentParser,Store};


fn main() {
	let mut parameter = "".to_string();
	let mut when = "today".to_string();

	{
		let mut ap = ArgumentParser::new();

		ap.refer(&mut parameter)
			.add_option(&["--parameter"], Store, "Parameter, please look at http://rasp.linta.de/GERMANY/index_en.html.")
			.required();

		ap.refer(&mut when)
			.add_option(&["--when"], Store, "When, today or tomorrow.");

		ap.parse_args_or_exit();
	}

    println!("Rrasp!");
	println!("Parameter: {}, When: {}", parameter, when);
}