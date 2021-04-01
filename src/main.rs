use std::env;
use std::fs;
use std::io::Write;

const SNIPPETS_FILE: &str = "/home/felipe/Documents/projs/snip/snips.txt";

fn parse_pairs<'a>(text: &'a String) -> Option<Vec<(&'a str, &'a str)>> {
    if text.is_empty() {
        return None;
    }

    let mut pairs = Vec::new();

    let mut lines = text.lines();

    while let Some(line) = lines.next() {
        let pos = line.find('\'')?;
        if pos > 0 {
            let (mut key, mut value) = line.split_at(pos);
            key = key.trim();
            value = value.trim();

            value = value.strip_prefix("'")?;
            value = value.strip_suffix("'")?;

            pairs.push((key, value));
        } else {
            // TODO: Maybe add better format checking
            return None;
        }
    }

    Some(pairs)
}

fn write_snippets(filename: &str, pairs: &Vec<(&str, &str)>) -> std::io::Result<()> {
    let mut file = fs::File::create(filename)?;

    for (key, value) in pairs {
        let line = format!("{} '{}'\n", key, value);
        file.write(line.as_bytes())?;
    }

    Ok(())
}

fn read_file(filename: &str) -> String {
    let result = fs::read_to_string(filename);

    match result {
        Ok(value) => value,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                let _ = fs::File::create(filename);
            }
            String::new()
        }
    }
}

fn list(name: Option<&str>) {
    let data = read_file(SNIPPETS_FILE);

    if data.is_empty() {
        println!("No snippets were found.");
    }

    match parse_pairs(&data) {
        Some(pairs) => {
            match name {
                Some(name_value) => {
                    for (key, value) in &pairs {
                        if key.contains(name_value) {
                            println!("{}: {}", key, value);
                        }
                    }
                },
                None => {
                    for (key, value) in &pairs {
                        println!("{}: {}", key, value);
                    }
                }
            }
        },
        None => {
            println!("Syntax error found in {}", SNIPPETS_FILE);
        }
    }
}

fn add(name: &str, value: &str) {
    let data = read_file(SNIPPETS_FILE);

    match parse_pairs(&data) {
        Some(mut pairs) => {
            for (key, value) in &pairs {
                if *key == name {
                    println!("{} already exists with value '{}'", key, value);
                    return;
                }
            }

            pairs.push((name, value));

            let _ = write_snippets(SNIPPETS_FILE, &pairs);
        },
        None => {
            let mut pairs = Vec::new();
            pairs.push((name, value));

            let _ = write_snippets(SNIPPETS_FILE, &pairs);
        }
    }

    println!("{} added", name);
}

fn set(name: &str, value: &str) {
    let data = read_file(SNIPPETS_FILE);

    match parse_pairs(&data) {
        Some(mut pairs) => {
            let mut pos = 0;
            for (key, _) in &pairs {
                if *key == name {
                    break;
                }
                pos += 1;
            }

            if pos < pairs.len() {
                pairs.swap_remove(pos);
            }

            pairs.push((name, value));

            let _ = write_snippets(SNIPPETS_FILE, &pairs);
        },
        None => {
            let mut pairs = Vec::new();
            pairs.push((name, value));

            let _ = write_snippets(SNIPPETS_FILE, &pairs);
        }
    }

    println!("{} setted", name);
}

fn remove(name: &str) {
    let data = read_file(SNIPPETS_FILE);

    match parse_pairs(&data) {
        Some(mut pairs) => {
            let mut pos = 0;
            for (key, _) in &pairs {
                if *key == name {
                    break;
                }
                pos += 1;
            }

            if pos < pairs.len() {
                pairs.swap_remove(pos);
                println!("{} removed", name);
                let _ = write_snippets(SNIPPETS_FILE, &pairs);
            } else {
                println!("{} not exists", name);
            }
        },
        None => {
        }
    }
}

fn find(name: &str) {
    let data = read_file(SNIPPETS_FILE);

    match parse_pairs(&data) {
        Some(pairs) => {
            for (key, value) in &pairs {
                if *key == name {
                    println!("{}", value);
                }
            }
        },
        None => {
        }
    }
}

fn help() {
    println!("
Options:
    --help, -h
        print this help
    --list, -l [name]
        list existing snippets matching name if provided
    --add,  -a <name> <value>
        add snippet
    --set,  -s <name> <value>
        set the value of a existing snippet
    --remove,-r <name>
        remove a existing snippet
    ");
}

fn unknown_argument(command: &str) {
    println!("unknown command {}, check --help for usage", command);
}

fn wrong_number_of_arguments(command: &str) {
    println!("Wrong number of arguments for {}, check --help for usage", command);
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    match args[1].as_str() {
        "--help" | "-h" => help(),
        "--list" | "-l" => {
            if args.len() == 2 {
                list(None);
            } else if args.len() == 3 {
                list(Some(args[2].as_str()));
            } else {
                wrong_number_of_arguments(args[1].as_str());
            }
        },
        "--add" | "-a" => {
            if args.len() == 4 {
                add(args[2].as_str(), args[3].as_str());
            } else {
                wrong_number_of_arguments(args[1].as_str());
            }
        },
        "--set" | "-s" => {
            if args.len() == 4 {
                set(args[2].as_str(), args[3].as_str());
            } else {
                wrong_number_of_arguments(args[1].as_str());
            }
        },
        "--remove" | "-r" => {
            if args.len() == 3 {
                remove(args[2].as_str());
            } else {
                wrong_number_of_arguments(args[1].as_str());
            }
        },
        _ => {
            if args[1].as_str().starts_with("-") {
                unknown_argument(args[1].as_str());
            } else {
                find(args[1].as_str());
            }
        }
    }
}
