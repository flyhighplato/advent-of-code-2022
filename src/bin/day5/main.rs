use advent::file;
use std::collections::VecDeque;

// fn move_crate(mut stacks: Vec<VecDeque<String>>, count: u32, src_ix: usize, dest_ix: usize) -> Vec<VecDeque<String>> {
//     for _ in 0..count {
//         let mut_stacks = &mut stacks;

//         let src = &mut mut_stacks[src_ix];
//         let el = src.pop_back();

//         if el.is_some() {
//             let dest = &mut mut_stacks[dest_ix];
//             dest.push_back(el.unwrap())
//         }
//     }

//     return stacks
// }

fn make_crates(lines: Vec<String>) -> Vec<VecDeque<String>> {
    let temp_stacks = lines
        .last()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_, c)| c.clone() != ' ')
        .map(|(ix, _)| {
            let mut stack: Vec<String> = vec![];

            for line_ix in (0..lines.len() - 1).rev() {
                let crate_char = lines[line_ix]
                    .chars()
                    .collect::<Vec<char>>()
                    .get(ix)
                    .unwrap()
                    .clone();

                if crate_char == ' ' {
                    break;
                }

                stack.push(crate_char.to_string());
            }
            return stack;
        })
        .map(|v| VecDeque::from(v))
        .collect::<Vec<VecDeque<String>>>();

    return temp_stacks;
}

fn main() {
    let lines = file::lines_from_file("./src/bin/day5/input.txt");
    let res = lines
        .iter()
        .enumerate()
        .find(|(ix, l)| l.len() == 0)
        .unwrap()
        .0;

    let header = Vec::from(&lines[..res]);

    let mut stacks = make_crates(header);

    println!("Before");
    for (ix, c) in stacks.iter().enumerate() {
        println!("{ix}: {:?}", c);
    }

    let body: Vec<String> = Vec::from(&lines[res + 1..]);

    body.iter().for_each(|l| {
        let tokens: Vec<&str> = l.split(" ").collect();
        let count = tokens[1].parse::<u32>().unwrap();
        let src_ix = tokens[3].parse::<usize>().unwrap() - 1;
        let dest_ix = tokens[5].parse::<usize>().unwrap() - 1;

        let mut temp: Vec<String> = Vec::new();

        let mut_stacks = &mut stacks;
        for _ in 0..count {
            let src = &mut mut_stacks[src_ix];
            let el = src.pop_back();

            if el.is_some() {
                let dest = &mut mut_stacks[dest_ix];
                //dest.push_back(el.unwrap())
                temp.push(el.unwrap());
            }
        }

        for t in temp.iter().rev() {
            let dest = &mut mut_stacks[dest_ix];
            dest.push_back(t.to_owned())
        }
    });

    println!("After");
    for (ix, c) in stacks.iter().enumerate() {
        println!("{ix}: {:?}", c);
    }

    println!("\r\nTops");
    let tops: Vec<String> = stacks
        .iter()
        .map(|s| s.back().unwrap().to_owned())
        .collect();

    println!("{:?}", tops.join(""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // fn move_one() {
    //     let start= vec![
    //         VecDeque::from(vec!["A".to_string(), "B".to_string()]),
    //         VecDeque::from(vec![])
    //     ];

    //     let end:Vec<VecDeque<String>> = vec![
    //         VecDeque::from(vec!["A".to_string()]),
    //         VecDeque::from(vec!["B".to_string()])
    //     ];

    //     assert_eq!(
    //         move_crate(start, 1, 0, 1),
    //         end
    //     );
    // }

    // #[test]
    // fn move_two() {
    //     let start= vec![
    //         VecDeque::from(vec!["A".to_string(), "B".to_string(), "C".to_string()]),
    //         VecDeque::from(vec![])
    //     ];

    //     let end = vec![
    //         VecDeque::from(vec!["A".to_string()]),
    //         VecDeque::from(vec!["C".to_string(), "B".to_string()])
    //     ];

    //     assert_eq!(
    //         move_crate(start, 2, 0, 1),
    //         end
    //     );
    // }
    #[test]
    fn test_make_crates() {
        assert_eq!(
            make_crates(vec![
                "        [Q]".to_string(),
                "[N]     [C]".to_string(),
                "[Z] [M] [P]".to_string(),
                " 1   2   3 ".to_string(),
            ]),
            vec![
                VecDeque::from(vec!["Z".to_string(), "N".to_string()]),
                VecDeque::from(vec!["M".to_string()]),
                VecDeque::from(vec!["P".to_string(), "C".to_string(), "Q".to_string()])
            ]
        );
    }
}
