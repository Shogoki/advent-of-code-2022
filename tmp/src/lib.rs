#![feature(iter_array_chunks)]
pub fn process_part1(input: &str) -> String {
    let result = input
        .lines()
        .map(|rucksack| {
           let (comp1, comp2) = rucksack.split_at(rucksack.len()/2);
           let dup_vec = comp1.chars().filter(|item| comp2.contains(*item)).take(1).collect::<Vec<_>>();
           let duplicate: u32= *dup_vec.first().unwrap() as u32;
           if duplicate > 96 {
               return duplicate - 96;
           }
           else {
               return duplicate - 38;
           }
        })
        .sum::<u32>();
    result.to_string()
}


pub fn process_part2(input: &str) -> String {
    let result = input
        .lines()
        .array_chunks().map(|[e1, e2, e3]| {
            let badge: u32 = e1.chars().find(|item| e2.contains(*item) && e3.contains(*item)).unwrap() as u32;
           if badge > 96 {
               return badge - 96;
           }
           else {
               return badge - 38;
           }
        })
        .sum::<u32>();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "45000");
    }
}
