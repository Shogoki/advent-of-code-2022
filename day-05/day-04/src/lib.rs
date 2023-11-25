pub fn process_part1(input: &str) -> String {
    let result = input
        .lines()
        .map(|pair| {

           let ranges = pair.split(",").map(|r| {
                let nums = r.split("-").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
                (nums[0], nums[1])
           }).collect::<Vec<_>>();
           assert!(ranges.len() == 2);
           let (low1, up1) = ranges[0];
           let (low2, up2) = ranges[1];
           if low1 <= low2 && up1 >= up2 {
               return 1;
           }
           if low2 <= low1 && up2 >= up1 {
               return 1;
           }
           return 0;
        })
        .sum::<u32>();
    result.to_string()
}


pub fn process_part2(input: &str) -> String {
    let result = input
        .lines()
        .map(|pair| {

           let ranges = pair.split(",").map(|r| {
                let nums = r.split("-").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
                (nums[0], nums[1])
           }).collect::<Vec<_>>();
           assert!(ranges.len() == 2);
           let mut r1 = ranges[0].0..=ranges[0].1;
           let r2 = ranges[1].0..=ranges[1].1;
           if r1.any(|x| r2.contains(&x)) {
               return 1;
           }
           return 0;
        })
        .sum::<u32>();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "4");
    }
}
