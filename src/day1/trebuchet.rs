use std::{vec, str::Lines};

#[allow(dead_code)]
fn get_calibration_values(input: Lines<'_>) -> Vec<(usize, usize)> {
    let mut results = vec![];
    for line in input {
        let line_result = line.chars()
            .fold(None, |res : Option<(usize, usize)>, char| {
                match char.to_digit(10) {
                    None => res,
                    Some(d) => match res {
                        None => Some((d as usize, d as usize)),
                        Some((f, _)) => Some((f, d as usize))
                    }
                }
            })
            .unwrap();

        results.push(line_result);
    }

    results
}

#[allow(dead_code)]
fn sum_calibration_values(input: Lines<'_>) -> usize {
    get_calibration_values(input)
        .iter()
        .map(|(x, y)| x * 10 + y )
        .sum()
}

#[allow(dead_code)]
fn get_calibration_values_part_two(input: Lines<'_>) -> Vec<(usize, usize)> {
    // Now we can't match character by character so we'll do 'position' (works like index_of)
    let matches = vec![
        "1", "one",
        "2", "two",
        "3", "three",
        "4", "four",
        "5", "five",
        "6", "six",
        "7", "seven",
        "8", "eight",
        "9", "nine"
    ];

    let mut results = vec![];
    for line in input {
        let smallest = matches.iter()
            .enumerate()
            .filter_map(|(i, &m)| line.find(m).map(|p| (i, p)))
            .min_by(|(_, x), (_, y)| x.cmp(y))
            .map(|(i, _)| i / 2 + 1)
            .unwrap();

        let largest = matches.iter()
            .enumerate()
            .filter_map(|(i, &m)| line.rfind(m).map(|p| (i, p)))
            .min_by(|(_, x), (_, y)| y.cmp(x))
            .map(|(i, _)| i / 2 + 1)
            .unwrap();

        results.push((smallest, largest));
    }

    results
}

#[allow(dead_code)]
fn sum_calibration_values_part_two(input: Lines<'_>) -> usize {
    get_calibration_values_part_two(input)
        .iter()
        .map(|(x, y)| x * 10 + y )
        .sum()
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::sum_calibration_values;
    use super::sum_calibration_values_part_two;

    #[test]
    fn example() {
        let file_path = "src/day1/test_files/example.txt";
        let mut file = File::open(file_path)
            .unwrap_or_else(|_| panic!("Could not read file `{}`", file_path));

        let mut input = String::new();
        file.read_to_string(&mut input)
            .unwrap();

        let result = sum_calibration_values(input.lines());

        assert_eq!(142, result);
    }

    #[test]
    fn challenge() {
        let file_path = "src/day1/test_files/challenge.txt";
        let mut file = File::open(file_path)
            .unwrap_or_else(|_| panic!("Could not read file `{}`", file_path));

        let mut input = String::new();
        file.read_to_string(&mut input)
            .unwrap();

        let result = sum_calibration_values(input.lines());

        assert_eq!(53651, result);
    }

    #[test]
    fn part2_example() {
        let file_path = "src/day1/test_files/part2_example.txt";
        let mut file = File::open(file_path)
            .unwrap_or_else(|_| panic!("Could not read file `{}`", file_path));

        let mut input = String::new();
        file.read_to_string(&mut input)
            .unwrap();

        let result = sum_calibration_values_part_two(input.lines());

        assert_eq!(281, result);
    }

    #[test]
    fn part2_challenge() {
        let file_path = "src/day1/test_files/challenge.txt";
        let mut file = File::open(file_path)
            .unwrap_or_else(|_| panic!("Could not read file `{}`", file_path));

        let mut input = String::new();
        file.read_to_string(&mut input)
            .unwrap();

        let result = sum_calibration_values_part_two(input.lines());

        assert_eq!(53894, result);
    }
}