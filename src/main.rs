extern crate argparse;

use argparse::{ArgumentParser,Store};

const TODAY: &'static str = "curr";
const TOMORROW: &'static str = "curr+1";
const RASP_URL: &'static str = "http://rasp.linta.de/GERMANY/";
const SUFFIX: &'static str = "lst.d2.png";
const HOURS: [&'static str; 13] = ["07", "08", "09", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19"];


fn main() {
    let mut parameter = "".to_string();
    let mut when = "today".to_string();

	// `parameter` and `when` variables are mutable thus we cannot borrow them more than once at a time
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
