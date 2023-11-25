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

fn normalize_selection(own: &str) -> String {
    return match own {
        "X" => "A",
        "Y" => "B",
        "Z" => "C",
        _   => panic!("invalid selection")
    }.to_string();
}

fn get_round_points(enemy: &str, own: &str) -> u32 {
    let selection_points = match own{
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => panic!("Should not happen")
    };
    let result_points = match (enemy, own) {
        ("A", "B") => 6,
        ("A", "C") => 0,
        ("B", "A") => 0,
        ("B", "C") => 6,
        ("C", "A") => 6,
        ("C", "B") => 0,
        _ => 3
    };
    return selection_points + result_points;

}
fn get_selection(enemy: &str, outcome: &str) -> String {
    // X= LOOSE, Y = DRAW, Z = WIN 
    // A= ROCK, B = PAPER, C = SCISSORS
    return match (enemy, outcome) {
        ("A", "X") => "C",
        ("A", "Z") => "B",
        ("B", "X") => "A",
        ("B", "Z") => "C",
        ("C", "X") => "B",
        ("C", "Z") => "A",
        _ => enemy // DRAW
    }.to_string()

}

fn part1(input: &str) -> String {

    let res = input.lines()
        .map(|round| {
            let r = round.split(" ").collect::<Vec<_>>();
            let sel = normalize_selection(r[1]);
            get_round_points(r[0], &sel)
        }).sum::<u32>();

    return res.to_string();
}
fn part2(input: &str) -> String {
    let res = input.lines()
        .map(|round| {
            let r = round.split(" ").collect::<Vec<_>>();

            let sel = get_selection(&r[0],&r[1]);
            get_round_points(r[0], &sel)
        }).sum::<u32>();

    return res.to_string();
}
