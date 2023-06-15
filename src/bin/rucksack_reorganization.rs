use std::{collections::HashMap, fs};

fn main() {
    // Get inventories (lines of input file split in half)
    let inventories = process_input();

    // Create lookup table for item values
    // a to z = 1 to 26
    // A to Z = 27 to 52
    let table = create_lookup_table();

    // Calculate solution for part one
    part_one(&inventories, &table);

    // Calculate solution for part two
    part_two(&inventories, &table);
}

fn part_one(inventories: &Vec<Inventory>, table: &HashMap<char, u8>) {
    let mut sum: u32 = 0;

    // Check for each character in compartment_1 if it is also in compartment_2
    // If hit then look up value of character and add to sum
    for inventory in inventories {
        for character in inventory.compartment_1.chars() {
            if inventory.compartment_2.contains(character) {
                let char_value = table.get(&character).unwrap().clone();
                sum += char_value as u32;
                break;
            }
        }
    }

    println!("\nAnswer part one");
    println!(
        "\tThe sum of the value of all missplaced items is: {}\n",
        sum
    );
}

fn part_two(inventories: &Vec<Inventory>, table: &HashMap<char, u8>) {
    // Join compartments and create Groups by taking three consecutive inventories
    let full_inventories: Vec<String> = inventories
        .iter()
        .map(|inventory| {
            let mut temp = String::from(&inventory.compartment_1);
            temp.extend(inventory.compartment_2.chars());
            temp
        })
        .collect();

    let mut groups: Vec<Group> = Vec::new();

    for i in 0..full_inventories.len() {
        if i % 3 != 0 {
            continue;
        };
        groups.push(Group {
            member1: full_inventories.get(i).unwrap().to_string(),
            member2: full_inventories.get(i + 1).unwrap().to_string(),
            member3: full_inventories.get(i + 2).unwrap().to_string(),
        });
    }

    // For each group iterate over every character in member1 and search it in member2 and member3
    // If it is found in both then add to sum and break;
    let mut sum = 0;

    for group in groups.iter_mut() {
        for character in group.member1.chars() {
            if group.member2.contains(character) {
                if group.member3.contains(character) {
                    let char_value = table.get(&character).unwrap().clone();
                    sum += char_value as u32;
                    break;
                }
            }
        }
    }

    println!("\nAnswer part two");
    println!("\tThe sum of the value of all badges is: {}\n", sum);
}

fn process_input() -> Vec<Inventory> {
    // Get content of input.txt as String
    let content = fs::read_to_string("inputs/input-d3.txt").expect("Couldn't read file!");

    // Seperate lines
    let lines: Vec<String> = content.split("\r\n").map(|line| line.to_string()).collect();
    // println!("{:?}", lines);

    // Create inventories from each line (that means split line in half)
    let inventories: Vec<Inventory> = lines
        .iter()
        .map(|line| {
            let len = line.len();
            Inventory {
                compartment_1: line[0..len / 2].to_string(),
                compartment_2: line[len / 2..len].to_string(),
            }
        })
        .collect();

    inventories
}

fn create_lookup_table() -> HashMap<char, u8> {
    let keys = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let mut table: HashMap<char, u8> = HashMap::new();

    for (i, c) in keys.chars().enumerate() {
        table.insert(c, (i + 1) as u8);
    }

    table
}

struct Inventory {
    compartment_1: String,
    compartment_2: String,
}

struct Group {
    member1: String,
    member2: String,
    member3: String,
}
