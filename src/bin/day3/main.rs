use std::collections::{HashSet, hash_map::RandomState};
use advent::file;

fn get_rucksack_priorities(s: &String) -> Vec<u32> {
    let midpoint = s.len()/2;
    let left_set: HashSet<char, RandomState> = HashSet::from_iter((&s[..midpoint]).chars());
    let right_set: HashSet<char, RandomState> = HashSet::from_iter((&s[midpoint..]).chars());


    return left_set.intersection(&right_set)
                                    .map(|c| { 
                                        let value = c.clone() as u32;
                                        //println!("converting {c} which is {value}");
                                        return if value > 96 { value - 96 } else { value - 64 + 26}
                                    })
                                    .collect::<Vec<u32>>();
}

fn sum_rucksack_priorities(lines: Vec<String>) -> u32 {
    return lines.iter().map(|s| get_rucksack_priorities(s).iter().sum::<u32>()).sum();
}


fn main() {
    let sum1 = sum_rucksack_priorities(file::lines_from_file("./src/bin/day3/input.txt"));
    println!("Sum1: {sum1}");

    let sum2: u32 = file::lines_from_file("./src/bin/day3/input.txt").chunks(3).map(|chunk| {
        let mut set: Option<HashSet<u32>> = None;

        
        for item in chunk {
            let item_set = HashSet::from_iter(
                item.chars().map(|c| { 
                    let value = c.clone() as u32;
                    return if value > 96 { value - 96 } else { value - 64 + 26}
                })
            );

            if(set.is_none()) {
                set = Some(item_set);
            } else {
                set = Some(HashSet::from_iter(set.unwrap().intersection(&item_set).cloned()));
            }
        }

        let sum: u32 = set.unwrap().iter().sum();
        return sum;
    }).sum();

    println!("Sum2: {sum2}");
}

mod tests {
    use super::*;

    #[test]
    fn both_different() {
        assert_eq!(get_rucksack_priorities(&"ab".to_string()).iter().sum::<u32>(), 0);
    }

    #[test]
    fn example() {
        assert_eq!(sum_rucksack_priorities(vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ]),157);
    }
}