
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn calc_nmax_calories_sum(lines: Vec<Option<i32>>, n: usize) -> i32 {
    let mut totals: Vec<i32> = vec![0];

    for line in lines {
        if line.is_none() {
            totals.push(0);
        } else {
            let last_index = totals.len()-1;
            totals[last_index] += line.unwrap()
        }
    }

    totals.sort();

    return totals
                .iter()
                .rev()
                .map(|f| f.clone())
                .take(n)
                .sum();
}

fn main() {
    let lines = lines_from_file("input.txt")
                                    .iter()
                                    .map(|f| f.parse::<i32>().ok())
                                    .collect();

    let max_sum = calc_nmax_calories_sum(lines, 3);
    println!("Max calories: {max_sum}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_group() {
        assert_eq!(calc_nmax_calories_sum(
            vec![
                Some(1000i32),
                Some(2000i32)
            ], 1), 3000);
    }

    #[test]
    fn test_two_groups_first_bigger() {
        assert_eq!(calc_nmax_calories_sum(
            vec![
                Some(1000i32),
                Some(2000i32),
                None,
                Some(500i32),
                Some(1000i32)
            ], 1), 3000);
    }

    #[test]
    fn test_two_groups_last_bigger() {
        assert_eq!(calc_nmax_calories_sum(
            vec![
                Some(1000i32),
                Some(2000i32),
                None,
                Some(2500i32),
                Some(1000i32)
            ], 1), 3500);
    }

    #[test]
    fn test_two_groups_sum() {
        assert_eq!(calc_nmax_calories_sum(
            vec![
                Some(1000i32),
                Some(2000i32),
                None,
                Some(2500i32),
                Some(1000i32)
            ], 2), 6500);
    }
}