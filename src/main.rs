extern crate argparse;

use std::process::exit;

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

    let when = if when == "today" { TODAY } else { TOMORROW };

    let parameter = match parameter.as_ref() {
        "Cu Cloudbase where Cu Potential>0" => "zsfclclmask",
        "Thermal Updraft Velocity and B/S ratio" => "wstar_bsratio",
        _ => std::process::exit(1),
    };

    let mut urls = Vec::with_capacity(HOURS.len());

    for h in HOURS.iter() {
        urls.push(
            RASP_URL.to_string() +
            parameter + "." +
            when + "." + h + "00" + SUFFIX
        );
    }

    println!("URLs: {:?}", urls);
}
