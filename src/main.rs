extern crate ini;
use crate::ini::Ini;
use std::process;

fn main() {
    let Ok(ide_data) = std::fs::read_to_string("data\\peds.ide") else {
        eprintln!("Failed to read peds.ide");
        process::exit(1);
    };

    let Ok(ini) = Ini::load_from_file("fastman92limitAdjuster_GTASA.ini") else {
        eprintln!("Failed to read fastman92limitAdjuster_GTASA.ini");
        process::exit(1);
    };

    let limit = ini
        .section(Some("IDE LIMITS"))
        .and_then(|s| s.get("peds"))
        .unwrap_or("278");

    let limit: usize = limit.parse().unwrap_or(278);

    match gta_ide_parser::parse_file(&ide_data) {
        Ok((_, sections)) => {
            for (name, items) in sections {
                if name.eq_ignore_ascii_case("peds") {
                    if limit < items.len() {
                        eprintln!(
                            "Peds Limit Error: expected {}, found {}",
                            limit,
                            items.len()
                        );
                        process::exit(1);
                    }
                    break;
                }
            }
        }
        Err(e) => {
            eprintln!("Error parsing IDE file: {}", e);
            process::exit(1);
        }
    }

    eprintln!("peds.ide limit check passed.");
    process::exit(0);
}
