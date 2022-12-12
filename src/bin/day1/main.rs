use advent::file;

fn calc_nmax_calories_sum(lines: Vec<Option<i32>>, n: usize) -> i32 {
    let mut totals: Vec<i32> = vec![0];

    for maybe_line in lines {
        match maybe_line {
            Some(line) => {
                totals.last_mut().map(|last| *last += line);
            }
            None => totals.push(0),
        }
    }

    totals.sort();

    return totals.iter().rev().map(|f| f.clone()).take(n).sum();
}

fn main() {
    let lines = file::lines_from_file("./src/bin/day1/input.txt")
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
        assert_eq!(
            calc_nmax_calories_sum(vec![Some(1000i32), Some(2000i32)], 1),
            3000
        );
    }

    #[test]
    fn test_two_groups_first_bigger() {
        assert_eq!(
            calc_nmax_calories_sum(
                vec![
                    Some(1000i32),
                    Some(2000i32),
                    None,
                    Some(500i32),
                    Some(1000i32)
                ],
                1
            ),
            3000
        );
    }

    #[test]
    fn test_two_groups_last_bigger() {
        assert_eq!(
            calc_nmax_calories_sum(
                vec![
                    Some(1000i32),
                    Some(2000i32),
                    None,
                    Some(2500i32),
                    Some(1000i32)
                ],
                1
            ),
            3500
        );
    }

    #[test]
    fn test_two_groups_sum() {
        assert_eq!(
            calc_nmax_calories_sum(
                vec![
                    Some(1000i32),
                    Some(2000i32),
                    None,
                    Some(2500i32),
                    Some(1000i32)
                ],
                2
            ),
            6500
        );
    }
}
