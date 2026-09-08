use std::{env, process};

use a2pcej::{A2pcej, Language, Options};

fn usage() -> ! {
    eprintln!("usage: a2pcej -m en|ja [options] letters...");
    eprintln!("options: -d/--delimiter VALUE, -nd/--nodelimiter, -s/--sign VALUE,");
    eprintln!("         -ns/--nosign, -n/--num");
    process::exit(2)
}

fn take_value(args: &[String], index: &mut usize) -> String {
    *index += 1;
    args.get(*index).cloned().unwrap_or_else(|| usage())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut mode = None;
    let mut delimiter = None;
    let mut sign = None;
    let mut num = false;
    let mut letters = Vec::new();
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "-m" | "--mode" => mode = Some(take_value(&args, &mut index)),
            "-d" | "--delimiter" => delimiter = Some(take_value(&args, &mut index)),
            "-nd" | "--nodelimiter" => delimiter = Some(String::new()),
            "-s" | "--sign" => sign = Some(take_value(&args, &mut index)),
            "-ns" | "--nosign" => sign = Some(String::new()),
            "-n" | "--num" => num = true,
            "-h" | "--help" => usage(),
            value if value.starts_with('-') => usage(),
            value => letters.push(value.to_owned()),
        }
        index += 1;
    }

    let language = mode
        .as_deref()
        .and_then(|value| Language::try_from(value).ok())
        .unwrap_or_else(|| usage());
    if letters.is_empty() {
        usage()
    }
    let mut options = match language {
        Language::English => Options::english(),
        Language::Japanese => Options::japanese(),
    };
    if let Some(value) = delimiter {
        options.delimiter = value;
    }
    if let Some(value) = sign {
        options.sign = value;
    }
    options.num = num;
    let converter = A2pcej::new(language, options);
    for value in letters {
        println!("{}", converter.convert(&value));
    }
}
