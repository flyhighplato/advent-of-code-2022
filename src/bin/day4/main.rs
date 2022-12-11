
use advent::file;

fn contains_range(container: (u32, u32), containee: (u32, u32)) -> bool {
    return containee.0 >= container.0 && containee.1 <= container.1
}


fn overlaps_range(r1: (u32, u32), r2: (u32, u32)) -> bool {
    return (r1.0 <= r2.0 && r2.0 <= r1.1) || (r1.0 <= r2.0 && r2.0 <= r1.1) || 
            (r2.0 <= r1.0 && r1.0 <= r2.1) || (r2.0 <= r1.1 && r1.1 <= r2.1)
}

fn main() {
    let total_contains: u32 = file::lines_from_file("./src/bin/day4/input.txt").iter().map(|line|{
        let range_vals: Vec<u32> = line.split(",")
                                        .flat_map(|l| l.split("-"))
                                        .map(|s| s.parse::<u32>().unwrap())
                                        .collect();

        let r1 = (range_vals[0], range_vals[1]);
        let r2 = (range_vals[2], range_vals[3]);

        println!("r1: {:?}, r2: {:?}", r1, r2);

        let contains = contains_range(r1, r2) || contains_range(r2, r1);

        println!("contains? {contains}");
        return if contains  {1}  else {0}
    }).sum();

    println!("Total contains: {total_contains}");

    let total_overlaps: u32 = file::lines_from_file("./src/bin/day4/input.txt").iter().map(|line|{
        let range_vals: Vec<u32> = line.split(",")
                                        .flat_map(|l| l.split("-"))
                                        .map(|s| s.parse::<u32>().unwrap())
                                        .collect();

        let r1 = (range_vals[0], range_vals[1]);
        let r2 = (range_vals[2], range_vals[3]);

        println!("r1: {:?}, r2: {:?}", r1, r2);

        let contains = overlaps_range(r1, r2);

        println!("overlaps? {contains}");
        return if contains  {1}  else {0}
    }).sum();

    println!("Total overlaps: {total_overlaps}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_true() {
        assert_eq!(contains_range((2,8), (3,7)), true);
    }

    #[test]
    fn test_contains_fakse() {
        assert_eq!(contains_range((2,8), (3,9)), false);
    }

    #[test]
    fn test_overlaps_true() {
        assert_eq!(overlaps_range((1,6), (3,7)), true);
    }
}