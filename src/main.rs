extern crate argparse;
extern crate hyper;

use std::io;
use std::fs;
use std::path::Path;
use std::process::Command;

use std::process::exit;

use hyper::Client;

use argparse::{ArgumentParser,Store};

const TODAY: &'static str = "curr";
const TOMORROW: &'static str = "curr+1";
const RASP_URL: &'static str = "http://rasp.linta.de/GERMANY/";
const SUFFIX: &'static str = "lst.d2.png";
const HOURS: [&'static str; 13] = ["07", "08", "09", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19"];
const IMAGES_DIR: &'static str = "images_data";


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
            .add_option(&["--when"], Store, "When; today or tomorrow.");

        ap.parse_args_or_exit();
    }

    // today or tomorrow
    let when_filename = if when == "today" { "today" } else { "tomorrow" };
    let when = if when == "today" { TODAY } else { TOMORROW };

    // Which parameter
    let parameter_url = match parameter.as_ref() {
        "Cu Cloudbase where Cu Potential>0" => "zsfclclmask",
        "Thermal Updraft Velocity and B/S ratio" => "wstar_bsratio",
        _ => std::process::exit(1),
    };

    // Create URLs
    let mut urls = Vec::with_capacity(HOURS.len());
    for h in HOURS.iter() {
        urls.push(
            RASP_URL.to_string() +
            parameter_url + "." +
            when + "." + h + "00" + SUFFIX
        );
    }

    // Create a dir for images
    fs::create_dir(IMAGES_DIR).unwrap_or_else(|why| {
        println!("{:?}!", why.kind());
    });

    // Iterate each url and download a content
    for url in urls {
        let client = Client::new();
        let mut res = client.get(&url).send().unwrap();

        let splited_url: Vec<&str> = url.split("/").collect();
        let file_name = String::from(IMAGES_DIR.to_string() + "/" + splited_url[4]);

        // Create a file and save the content
        let path = Path::new(&file_name);
        let mut file = fs::File::create(&path).unwrap();
        io::copy(&mut res, &mut file).unwrap();
    }

    // Convert PNGs into GIF
    Command::new("convert")
        .arg("-delay")
        .arg("50")
        .arg("-loop")
        .arg("0")
        .arg(IMAGES_DIR.to_string() + "/*png")
        .arg(parameter.replace("/", "") + "_" + when_filename + ".gif")
        .status().unwrap();

    // Remove the dir with images
    fs::remove_dir_all(IMAGES_DIR).unwrap_or_else(|why| {
        println!("{:?}!", why.kind());
    });
}
