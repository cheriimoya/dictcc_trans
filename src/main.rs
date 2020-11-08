extern crate reqwest;
extern crate select;
extern crate pico_args;

use select::document::Document;
use select::predicate::Name;

struct Args {
    help: bool,
    version: bool,
    null: bool,
    word: String,
}

fn print_version() {
    let version = "0.0.1";
    println!("DictCC_trans version {}", version);
}

fn print_help() {
    println!("todo write help");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = pico_args::Arguments::from_env();
    let args = Args {
        help: args.contains(["-h", "--help"]),
        version: args.contains(["-v", "--version"]),
        null: args.contains(["-0", "--null"]),
        word: if let Ok(Some(w)) = args.free_from_str() {
            w
        } else {
            panic!("Need word to get translation for");
        },
    };

    if args.version {
        print_version();
        return Ok(());
    }

    if args.help {
        print_help();
        return Ok(());
    }

    let translations = get_translation(&format!(
        "https://www.dict.cc/dcc-gadget.php?s={}", args.word));

    for translation in translations {
        if args.null {
            print!("{}\0", translation);
        } else {
            println!("{}", translation);
        }
    }

    Ok(())
}

fn get_translation(url: &str) -> Vec<String> {
    let resp = reqwest::blocking::get(url).unwrap();
    assert!(resp.status().is_success());

    Document::from_read(resp)
        .unwrap()
        .find(Name("a"))
        .filter(|n| n.attr("style").unwrap() == "color:black")
        .filter_map(|n| Some(n.text()))
        .collect()
}
