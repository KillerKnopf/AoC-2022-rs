use std::fs;

fn main() {
    let input_file = "input.txt";

    let content = fs::read_to_string(input_file).expect("Couldn't read file!");
    // println!("{}", content);

    let inventories = content.split("\r\n\r\n").collect::<Vec<&str>>();
    // println!("{:?}", inventories);
    // println!("count of inventories: {}", inventories.len());

    let calories: Vec<u32> = inventories
        .iter()
        .map(|inventory| {
            let cals: Vec<u32> = inventory
                .split("\r\n")
                .map(|line| str::parse::<u32>(line).unwrap_or(0))
                .collect();
            let mut sum = 0;
            for cal in cals {
                sum += cal;
            }
            sum
        })
        .collect();

    part_one(&calories);
    part_two(&calories);
}

fn part_two(calories: &Vec<u32>) {
    // println!("\n----------\n");
    // for cal in calories {
    //     println!("{}", cal);
    // }

    // Tuple that store the highest, second highest and third highest calorie sum (1st, 2nd, 3rd)
    let mut top_three = (0, 0, 0);

    // Finding the three highest calory sums
    for cals in calories {
        if cals > &top_three.0 {
            top_three.2 = top_three.1;
            top_three.1 = top_three.0;
            top_three.0 = cals.clone();

            continue;
        }
        if cals > &top_three.1 {
            top_three.2 = top_three.1;
            top_three.1 = cals.clone();
            continue;
        }
        if cals > &top_three.2 {
            top_three.2 = cals.clone();
            continue;
        }
    }

    // println!(
    //     "\nDEBUG top_three -> {}, {}, {}",
    //     top_three.0, top_three.1, top_three.2
    // );

    let total_calories = top_three.0 + top_three.1 + top_three.2;

    // println!("\n----------\n");

    println!("\nAnswer part two:");
    println!(
        "\tThe three elves with the most calories have packed {} calories.\n",
        total_calories
    );
}

fn part_one(calories: &Vec<u32>) {
    let most_calories = calories.iter().max().unwrap();

    println!("\nAnswer part one:");
    println!(
        "\tThe elf with the most calories packed has {} calories.\n",
        most_calories
    );
}
