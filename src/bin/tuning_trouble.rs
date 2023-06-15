use std::fs;

fn main() {
    let input = fs::read_to_string("input-d6.txt").expect("Couldn't read input.txt!");

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &String) {
    // Variable to receive the amount of chars read.
    let mut chars_read = 0;

    // Initialize vector holding chars to be tested for sop
    // And filling it with the first four characters
    let mut sop_test = [' ', ' ', ' ', ' '];
    sop_test[0] = input.chars().nth(0).unwrap();
    sop_test[1] = input.chars().nth(1).unwrap();
    sop_test[2] = input.chars().nth(2).unwrap();
    sop_test[3] = input.chars().nth(3).unwrap();

    // print_sop_test(" after initialization", &sop_test);

    // Iterating over input
    // Because sop_test was already filled with 4 chars
    // we can skip the first four iterations
    // and we check sop_test first and then insert the new char
    for (iteration, character) in input.chars().skip(4).enumerate() {
        // DEBUG test to analyse the behaviour of skip together with enumerate
        // In other words were does iteration start? At 0 or 4?
        // This if reduces the lines in the console that need to be scrolled upwards
        // println!("\nDEBUG  iteration: {}", iteration);
        // if iteration > 12 {
        //     break;
        // }
        // Answer enumerate() is after skip() so iteration starts at 0

        // Variable to store if a duplicate was found
        let mut found_duplicate = false;

        // Compare each char with the rest
        // Because it doesn't matter which char is left or right
        // we only need to test a char of sop_test with the chars after it.

        // print_sop_test("before check", &sop_test);
        'check_sop: for i in 0..4 {
            for j in i + 1..4 {
                if sop_test[i] == sop_test[j] {
                    // DEBUG print statement because i know i will fuck up
                    // println!(
                    //     "DEBUG  sop_test[{}]: {} | sop_test[{}]: {}",
                    //     i, sop_test[i], j, sop_test[j]
                    // );
                    found_duplicate = true;
                    break 'check_sop;
                }
            }
        }

        // If no duplicate character was found then sop was found and the loop can be left
        if !found_duplicate {
            chars_read = iteration + 4;
            // println!("\nDEBUG  chars_read: {}", chars_read);
            // println!("DEBUG  {:?}\n", sop_test);
            break;
        }

        // If a duplicate was found then sop_test needs the next character
        // and the other characters need to move forward discarding the first character.
        sop_test[0] = sop_test[1];
        sop_test[1] = sop_test[2];
        sop_test[2] = sop_test[3];
        sop_test[3] = character;
    }

    // Answer print
    println!("\nAnswer part one");
    println!(
        "The communication device needed to process {} characters.\n",
        chars_read
    );
}

fn part_two(input: &String) {
    // Variable to receive the amount of chars read.
    let mut chars_read = 0;

    // Initialize vector holding chars to be tested for sop
    // And filling it with the first 14 characters
    let mut som_test = [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ];
    for i in 0..14 {
        som_test[i] = input.chars().nth(i).unwrap();
    }

    // print_som_test(" after initialization", &som_test);

    // Iterating over input
    // Because som_test was already filled with 14 chars
    // we can skip the first 14 iterations
    // and we check som_test first and then insert the new char
    for (iteration, character) in input.chars().skip(14).enumerate() {
        // Variable to store if a duplicate was found
        let mut found_duplicate = false;

        // Compare each char with the rest
        // Because it doesn't matter which char is left or right
        // we only need to test a char of som_test with the chars after it.

        // println!();
        // print_som_test("before check", &som_test);
        'check_sop: for i in 0..14 {
            for j in i + 1..14 {
                if som_test[i] == som_test[j] {
                    // DEBUG print statement because i know i will fuck up
                    // println!(
                    //     "DEBUG  som_test[{}]: {} | som_test[{}]: {}",
                    //     i, som_test[i], j, som_test[j]
                    // );
                    found_duplicate = true;
                    break 'check_sop;
                }
            }
        }

        // If no duplicate character was found then sop was found and the loop can be left
        if !found_duplicate {
            chars_read = iteration + 14;
            // println!("\nDEBUG  chars_read: {}", chars_read);
            // println!("DEBUG  {:?}\n", som_test);
            break;
        }

        // If a duplicate was found then som_test needs the next character
        // and the other characters need to move forward discarding the first character.
        for i in 0..14 {
            if i != 13 {
                som_test[i] = som_test[i + 1];
            } else {
                som_test[i] = character;
            }
        }
    }

    // Answer print
    println!("\nAnswer part two");
    println!(
        "The communication device needed to process {} characters.\n",
        chars_read
    );
}

fn _print_sop_test(message: &str, sop_test: &[char; 4]) {
    print!("\t sop_test {}:\n\t", message);
    for c in sop_test {
        print!(" {}", c);
    }
    println!()
}

fn _print_som_test(message: &str, som_test: &[char; 14]) {
    print!("\t som_test {}:\n\t", message);
    for c in som_test {
        print!(" {}", c);
    }
    println!()
}
