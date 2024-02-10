use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;

#[macro_use] extern crate prettytable;
use prettytable::Table;

// This program will calculate the fragments needed for a given set of levels, and how many I have already used.
// As well as percentages done overall.
#[derive(Debug)]
enum Core {
    Skill,
    Mastery,
    Enhancement,
}

// Function will read a file that contains String:Type, and return a vector of those in a tuple.
fn read_file(path: String) -> Vec<(String, Core)> {
    let mut pairs: Vec<(String, Core)> = Vec::new();
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split(":");
        let name = split.next().unwrap().to_string();
        let core = match split.next().unwrap() {
            "Skill" => Core::Skill,
            "Mastery" => Core::Mastery,
            "Enhancement" => Core::Enhancement,
            _ => panic!("Invalid Core type"),
        };
        pairs.push((name, core));
    }
    pairs
}

fn main() {
    // Create a vector that will lisst required fragments for each level.
    let skill_core_fragments = vec![
        0, 30, 35, 40, 45, 50, 55, 60, 65, 200, 80, 90, 100, 110, 120, 130, 140, 150, 160, 350,
        170, 180, 190, 200, 210, 220, 230, 240, 250, 500,
    ];
    let mastery_core_fragments = vec![
        50, 15, 18, 20, 23, 25, 28, 30, 33, 100, 40, 45, 50, 55, 60, 65, 70, 75, 80, 175, 85, 90,
        95, 100, 105, 110, 115, 120, 125, 250,
    ];
    let enhancement_core_fragments = vec![
        75, 23, 27, 30, 34, 38, 42, 45, 49, 150, 60, 68, 75, 83, 90, 98, 105, 113, 120, 263, 128,
        135, 143, 150, 158, 165, 173, 180, 188, 375,
    ];

    // Read a vector of pairs from a file.
    let pairs = read_file("C:\\Users\\Rietty\\Documents\\Projects\\FragCalc\\target_levels.txt".to_string());

    // Create a table that will print the name of the skill, type of core, the fragments needed to max it (sum of the vector).
    let mut table = Table::new();

    table.add_row(row!["Name", "Type", "Fragments Needed", "Fragments Used", "Percentage Done"]);

    for (name, core) in pairs {
        let fragments: usize = match core {
            Core::Skill => skill_core_fragments.iter().sum(),
            Core::Mastery => mastery_core_fragments.iter().sum(),
            Core::Enhancement => enhancement_core_fragments.iter().sum(),
        };

        // Get the current level of the skill, and calculate the fragments used.
        // For example if it is level 5 and enhancement, it will use enhancement_core_fragments[0..5].iter().sum()
        let fragments_used: usize;
        let mut level = String::new();
        println!("What level is {}?", name);
        io::stdin().read_line(&mut level).unwrap();
        let level: usize = level.trim().parse().unwrap();
        match core {
            Core::Skill => {
                fragments_used = skill_core_fragments[0..level].iter().sum();
            }
            Core::Mastery => {
                fragments_used = mastery_core_fragments[0..level].iter().sum();
            }
            Core::Enhancement => {
                fragments_used = enhancement_core_fragments[0..level].iter().sum();
            }
        }

        table.add_row(row![name, format!("{:?}", core), fragments, fragments_used, format!("{:.2}%", (fragments_used as f64 / fragments as f64) * 100.0)]);
    }

    // Print the table.
    table.printstd();
}