use regex::Regex;
use std::fs;

fn main() {
    let mut state = initalize_state();
    part_one(&mut state);
    let mut state = initalize_state();
    part_two(&mut state);
}

fn part_one(state: &mut State) {
    // Iterate over instructions in state.instructions
    for line in state.instructions.iter() {
        // println!("\tcurrent state of stacks");
        // print_stacks(&state);
        // println!("\n\n\t{}\n<><><><><><><><><><>", line);

        // Extract number information out of line with Regex and store them in Vec
        let re = Regex::new(r"\d+").expect("Couldn't create Regex in part_one!");
        let mut data: Vec<usize> = Vec::new();
        for capture in re.captures_iter(&line) {
            data.push(capture.get(0).unwrap().as_str().parse::<usize>().unwrap());
        }

        // Create Instruction struct (count_crates = first number; pop_stack_index = second number; push_stack_index = third number).
        // This is done for readability. The fields are reversed because pop() is working in reverse.
        let instruction = Instruction {
            push_stack_index: data.pop().unwrap() - 1,
            pop_stack_index: data.pop().unwrap() - 1,
            count_crates: data.pop().unwrap(),
        };

        // Iterate over stack count_crates times
        for _ in 0..instruction.count_crates {
            // Pop from pop_stack_index in state.stacks
            let data = state
                .stacks
                .get_mut(instruction.pop_stack_index)
                .unwrap()
                .pop()
                .unwrap();
            // Push onto push_stack_index in state.stacks
            state
                .stacks
                .get_mut(instruction.push_stack_index)
                .unwrap()
                .push(data);
        }
    }

    // Get the last elements from the stack as the answer
    let mut answer = String::from("");
    for stack in state.stacks.iter_mut() {
        answer.push(stack.pop().unwrap());
    }

    println!("\n Answer part one");
    println!("\tThe top crates of the final stacks are: |{}|\n", answer);
}

fn part_two(state: &mut State) {
    // Iterate over instructions in state.instructions
    for line in state.instructions.iter() {
        // println!("\tcurrent state of stacks");
        // print_stacks(&state);
        // println!("\n\n\t{}\n<><><><><><><><><><>", line);

        // Extract number information out of line with Regex and store them in Vec
        let re = Regex::new(r"\d+").expect("Couldn't create Regex in part_one!");
        let mut data: Vec<usize> = Vec::new();
        for capture in re.captures_iter(&line) {
            data.push(capture.get(0).unwrap().as_str().parse::<usize>().unwrap());
        }

        // Create Instruction struct (count_crates = first number; pop_stack_index = second number; push_stack_index = third number).
        // This is done for readability. The fields are reversed because pop() is working in reverse.
        let instruction = Instruction {
            push_stack_index: data.pop().unwrap() - 1,
            pop_stack_index: data.pop().unwrap() - 1,
            count_crates: data.pop().unwrap(),
        };

        // Iterate over stack count_crates times
        // Pop from pop_stack_index in state.stacks.
        // Store pops in Vec<char> and reverse it.
        let mut data: Vec<char> = Vec::new();
        for _ in 0..instruction.count_crates {
            let value = state
                .stacks
                .get_mut(instruction.pop_stack_index)
                .unwrap()
                .pop()
                .unwrap();
            data.push(value);
        }
        data.reverse();

        // Append reversed Vec<char> to push_stack_index in state.stacks
        state
            .stacks
            .get_mut(instruction.push_stack_index)
            .unwrap()
            .append(&mut data);
    }

    // Get the last elements from the stack as the answer
    let mut answer = String::from("");
    for stack in state.stacks.iter_mut() {
        answer.push(stack.pop().unwrap());
    }

    println!("\n Answer part two");
    println!(
        "\tThe new top crates of the final stacks are: |{}|\n",
        answer
    );
}

fn initalize_state() -> State {
    // Read input file and initialize a State struct from it
    let input = fs::read_to_string("input-d5.txt").expect("Failed to read file!");

    // Split input into two initial state and instructions
    let mut split_input = input.split("\r\n\r\n");
    let initial_state = split_input
        .next()
        .expect("Failed to split input!")
        .to_string();
    let full_instructions = split_input
        .next()
        .expect("Failed to split input!")
        .to_string();

    // Seperate the instructions by line
    let instructions: Vec<String> = full_instructions
        .lines()
        .map(|line| line.to_string())
        .collect();

    // Find how many stacks are needed. By searching for the highest number in initial_state
    let re = Regex::new(r"\d+").expect("Couldn't create Regex");
    let stack_count = re
        .captures_iter(&initial_state)
        .map(|capture| capture.get(0).unwrap().as_str().parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .iter()
        .max()
        .unwrap()
        .to_owned();

    // Use stack_count to create a Vec with highest number of Vec<char> as content.
    let mut stacks: Vec<Vec<char>> = (0..stack_count)
        .map(|_| {
            let stack: Vec<char> = Vec::new();
            stack
        })
        .collect();

    // Iterate over all but last line of initial state to fill the stacks.
    // To stack in the correct order the lines need to be reversed.
    // That also makes skipping the last line easy because it is now the first line and skip() skips the first n iterations.
    for line in initial_state.lines().rev().skip(1) {
        for (i, character) in line.chars().enumerate().skip(1) {
            // Skip iteration if (i-2) % 4 != 0 because these chars don't contain letters
            if (i - 1) % 4 != 0 {
                continue;
            }
            // Push character on appropriate stack if the character isn't " ".
            if character != ' ' {
                stacks.get_mut((i - 1) / 4).unwrap().push(character);
            }
        }
    }

    State {
        instructions: instructions,
        stacks: stacks,
    }
}

struct State {
    instructions: Vec<String>,
    stacks: Vec<Vec<char>>,
}

struct Instruction {
    count_crates: usize,
    pop_stack_index: usize,
    push_stack_index: usize,
}

fn _print_stacks(state: &State) {
    for stack in state.stacks.iter() {
        print!(" ");
        for c in stack.iter() {
            print!("{}", c);
        }
        println!(".");
    }
}
