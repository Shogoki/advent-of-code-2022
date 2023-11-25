use std::fs;
fn main() {

    let content = read_input(String::from("input.txt"));
    let result1 = part1(&content);
    let result2 = part2(&content);

    println!("Part1: {result1}");
    println!("Part2: {result2}");
}


fn read_input(file_path: String) -> String{

    fs::read_to_string(file_path)
        .expect("Should have been able to read the file")

}


fn part1(input: &str) -> String {
    let res = input.split("\n\n").map(|elf| {
        elf
            .lines()
            .map(|food| food.parse::<u32>().unwrap())
            .sum::<u32>()
    }).max().unwrap();

    return res.to_string();
}
fn part2(input: &str) -> String {
    let mut res = input.split("\n\n").map(|elf| {
        elf
            .lines()
            .map(|food| food.parse::<u32>().unwrap())
            .sum::<u32>()
    }).collect::<Vec<_>>();
    res.sort_by(|a,b| b.cmp(a));
    let sum: u32 = res.iter().take(3).sum::<u32>();

    return sum.to_string();
}
