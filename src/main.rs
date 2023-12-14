use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::io;

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
    let pairs = read_file("target_levels.txt".to_string());

    // For each pair, calculate the total fragments needed, and how many I have already used.
    // We can do this by asking for pair's skill the level, and then using that to calculate the total fragments needed and how many I have already used.
    let mut total_fragments_needed = 0;
    let mut total_fragments_used = 0;

    let mut res: Vec<String> = Vec::new();

    for (name, core) in pairs {
        print!("Enter current level for {}: ", name);
        io::stdout().flush().unwrap();
        let mut current_level_str = String::new();
        io::stdin().read_line(&mut current_level_str).unwrap();

        let current_level: usize = current_level_str
            .trim()
            .parse()
            .expect("Please enter a number");

        let (fragments_vector, fragments_needed) = match core {
            Core::Skill => (
                &skill_core_fragments,
                skill_core_fragments.iter().sum::<i32>() - skill_core_fragments[current_level as usize - 1],
            ),
            Core::Mastery => (
                &mastery_core_fragments,
                mastery_core_fragments.iter().sum::<i32>() - mastery_core_fragments[current_level as usize - 1],
            ),
            Core::Enhancement => (
                &enhancement_core_fragments,
                enhancement_core_fragments.iter().sum::<i32>() - enhancement_core_fragments[current_level as usize - 1],
            ),
        };

        let fragments_used: i32 = fragments_vector[..current_level].iter().sum();

        res.push(format!(
            "{}: Total Needed: {}, Used: {}",
            name, fragments_needed, fragments_used
        ));

        total_fragments_needed += fragments_needed;
        total_fragments_used += fragments_used;
    }

    let percentage_done = (total_fragments_used as f32 / total_fragments_needed as f32) * 100.0;

    // Empty line.
    println!();

    // Print the results.
    for i in res {
        println!("{}", i);
    }

    println!(
        "Overall: Total Needed: {}, Used: {}, Completion: {:.2}%",
        total_fragments_needed, total_fragments_used, percentage_done
    );
}
