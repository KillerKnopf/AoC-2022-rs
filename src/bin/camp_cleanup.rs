use std::fs;

fn main() {
    let assigned_limits = process_input();

    part_one(&assigned_limits);
    part_two(&assigned_limits);
}

fn part_one(assigned_limits: &Vec<AssignedLimits>) {
    let mut sum_pairs = 0;

    for ranges in assigned_limits.iter() {
        let r1 = &ranges.range_1;
        let r2 = &ranges.range_2;

        if r1.start == r2.start || r1.end == r2.end {
            sum_pairs += 1;
            continue;
        }
        if r1.start < r2.start {
            if r1.end > r2.end {
                sum_pairs += 1;
            }
        } else {
            if r1.end < r2.end {
                sum_pairs += 1;
            }
        }
    }

    println!("\n Answer part one");
    println!(
        "\tIn {} pairs are one partners areas completely contained within the others areas.\n",
        sum_pairs
    );
}

fn part_two(assigned_limits: &Vec<AssignedLimits>) {
    let mut sum_of_overlaps = 0;
    // Vector with AssignedLimits that is made from AreaLimits
    // Create vec of sequential usizes from AreaLimits and put into AssignedAreas

    // Iterate over vec of AssignedLimits to turn pairs of AreaLimits into pairs of vec<usize>
    // Collect these pairs into AssignedAreas. Collect these into a vec of AssignedAreas
    let assigned_areas: Vec<AssignedAreas> = assigned_limits
        .iter()
        .map(|limits| AssignedAreas {
            areas_1: (limits.range_1.start..limits.range_1.end + 1).collect::<Vec<usize>>(),
            areas_2: (limits.range_2.start..limits.range_2.end + 1).collect::<Vec<usize>>(),
        })
        .collect();

    for areas in assigned_areas.iter() {
        // println!("{:?}\n{:?}\n", areas.areas_1, areas.areas_2);
        for area in areas.areas_1.iter() {
            if areas.areas_2.contains(&area) {
                sum_of_overlaps += 1;
                break;
            }
        }
    }

    println!("\n Answer part two");
    println!(
        "\t{} pairs have overlapping areas assigned.\n",
        sum_of_overlaps
    );
}

fn process_input() -> Vec<AssignedLimits> {
    // Get input from file
    let input = fs::read_to_string("inputs/input-d4.txt").expect("Couldn't read file!");

    // Split input into lines
    let lines = input.lines();

    // Extract limits from each line
    let limits: Vec<AssignedLimits> = lines
        .map(|line| {
            // println!("\n{}", line);

            // Split line into each assignment
            let assignments: Vec<&str> = line.split(",").collect();
            // println!("{:?}", assignments);

            // Split assignment into its start and end area
            let limits: Vec<(usize, usize)> = assignments
                .iter()
                .map(|range| {
                    let mut edges = range.split("-");
                    (
                        edges.next().unwrap().parse::<usize>().unwrap(),
                        edges.next().unwrap().parse::<usize>().unwrap(),
                    )
                })
                .collect();
            // println!("{:?}", limits);

            // Create AssignedLimits and AreaLimits struct from limits
            AssignedLimits {
                range_1: AreaLimits {
                    start: limits[0].0,
                    end: limits[0].1,
                },
                range_2: AreaLimits {
                    start: limits[1].0,
                    end: limits[1].1,
                },
            }
        })
        .collect();

    limits
}

struct AssignedLimits {
    range_1: AreaLimits,
    range_2: AreaLimits,
}

struct AreaLimits {
    start: usize,
    end: usize,
}

struct AssignedAreas {
    areas_1: Vec<usize>,
    areas_2: Vec<usize>,
}
