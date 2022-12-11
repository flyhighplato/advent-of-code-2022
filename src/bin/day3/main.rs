use std::collections::{HashSet, hash_map::RandomState};
use advent::file;

fn char_to_priority(c: char) -> u32 {
    let value = c as u32;
    return if value > 96 { value - 96 } else { value - 64 + 26}
}

fn get_rucksack_priorities(s: &String) -> Vec<u32> {
    let midpoint = s.len()/2;
    let left_set: HashSet<char, RandomState> = HashSet::from_iter((&s[..midpoint]).chars());
    let right_set: HashSet<char, RandomState> = HashSet::from_iter((&s[midpoint..]).chars());


    return left_set.intersection(&right_set)
                                    .map(|c| char_to_priority(c.clone()))
                                    .collect::<Vec<u32>>();
}

fn sum_rucksack_priorities1(lines: Vec<String>) -> u32 {
    return lines.iter().map(|s| get_rucksack_priorities(s).iter().sum::<u32>()).sum();
}

fn sum_rucksack_priorities2(lines: Vec<String>) -> u32 {
    return lines.chunks(3)
        .map(|chunk| {
            let maybe_intersecting_priorities: Option<HashSet<u32>> = chunk.iter()
                .fold(None as Option<HashSet<u32>>, |accum, el| {
                    let item_set = HashSet::from_iter(
                        el.chars().map(|c| char_to_priority(c))
                    );

                    return match accum {
                        Some(ref set) => Some(HashSet::from_iter(set.intersection(&item_set).cloned())),
                        None => Some(item_set)
                    };
                });

            return match maybe_intersecting_priorities {
                Some(set) => set.iter().sum(),
                None => 0
            };
        })
        .sum()
}


fn main() {
    let sum1 = sum_rucksack_priorities1(file::lines_from_file("./src/bin/day3/input.txt"));
    println!("Sum1: {sum1}");

    let sum2: u32 = sum_rucksack_priorities2(file::lines_from_file("./src/bin/day3/input.txt"));
    println!("Sum2: {sum2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_different() {
        assert_eq!(get_rucksack_priorities(&"ab".to_string()).iter().sum::<u32>(), 0);
    }

    #[test]
    fn example() {
        assert_eq!(sum_rucksack_priorities1(vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ]),157);
    }
}