use advent::file;

#[derive()]
enum ElfMove {
    A, // Rock
    B, // Paper
    C  // Scissors
}

#[derive()]
enum YourMove {
    X,  // Rock/Lose
    Y,  // Paper/Draw
    Z   // Scissors/Win
}

use ElfMove::*;
use YourMove::*;

fn compute_score1(rounds:Vec<(ElfMove, YourMove)>) -> u32 {
    return rounds.iter().map(|round| {
        return match round {
            (A, Z) | (B, X) | (C, Y) => 0, // Lose: Rock/Scissors, Paper/Rock, Scissors/Paper
            (A, X) | (B, Y) | (C, Z) => 3, // Draw: Rock/Rock, Paper/Paper, Scissors/Scissors
            (A, Y) | (B, Z) | (C, X) => 6  // Win: Rock/Paper, Paper/Scissors, Scissors/Rock
        } + match round {
            (_, X) => 1,
            (_, Y) => 2,
            (_, Z) => 3
        }
    }).sum();
}

fn compute_score2(rounds:Vec<(ElfMove, YourMove)>) -> u32 {
    return rounds.iter().map(|round| {
        return match round {
            (_, X) => 0,
            (_, Y) => 3,
            (_, Z) => 6
        } + match round {
            (B, X) | (A, Y) | (C, Z) => 1, // Rock: Lose to paper, Draw to rock, Win to scissors
            (C, X) | (B, Y) | (A, Z) => 2, // Paper: Lose to scissors, Draw to paper, Win to rock
            (A, X) | (C, Y) | (B, Z) => 3  // Scissors: Lose to rock, Draw to scissors, Win to paper
        };
    }).sum();
}

fn main() {
    let lines: Vec<(ElfMove, YourMove)> = file::lines_from_file("./src/bin/day2/input.txt")
                                    .iter()
                                    .map(|f| {
                                        let values: Vec<&str> = f.split(" ").collect();

                                        let elf_move= match values[0] {
                                            "A" => Ok(A),
                                            "B" => Ok(B),
                                            "C" => Ok(C),
                                            _ => Err(())
                                        }.unwrap();

                                        let your_move =match values[1] {
                                            "X" => Ok(X),
                                            "Y" => Ok(Y),
                                            "Z" => Ok(Z),
                                            _ => Err(())
                                        }.unwrap();

                                        return (elf_move, your_move)
                                        
                                    })
                                    .collect();
    let score = compute_score2(lines);
    println!("Score: {score}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elf_rock_you_rock() {
        assert_eq!(compute_score1(
            vec![
                (A, Y),
                (B, X),
                (C, Z)
            ]), 15);
    }
}