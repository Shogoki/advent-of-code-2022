pub fn process_part1(input: &str) -> String {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let firstline: String = map.lines().rev().take(1).collect::<Vec<_>>().first().unwrap().to_string();
    let mut stacks: Vec<Vec<char>> = firstline.split_whitespace()
        .map(|_| Vec::<char>::new()).collect::<Vec<_>>();

    map.lines().rev().skip(1).for_each(|line| {
        let mut stack = 0; //first crate is at index 1
        while stack < stacks.len() {
            let index = stack * 4 +1;
            let item = line.chars().nth(index).unwrap();
            if item != ' ' {
                stacks[stack].push(item);
            }
            stack += 1;
        }
    });
    dbg!(stacks.clone());

    moves.lines()
        .for_each(|action| {
           let parts = action.split_whitespace().map(|x| x.parse::<usize>().unwrap_or_else(|_| 0)).filter(|x| *x != 0 ).take(3).collect::<Vec<_>>();
           let (amount, from, to) = (parts[0], parts[1], parts[2]);
           for _ in 0..amount {
               let cargo = stacks[from-1].pop().unwrap();
               stacks[to-1].push(cargo);
           }
        });
    let result: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();

    return result;
}


pub fn process_part2(input: &str) -> String {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let firstline: String = map.lines().rev().take(1).collect::<Vec<_>>().first().unwrap().to_string();
    let mut stacks: Vec<Vec<char>> = firstline.split_whitespace()
        .map(|_| Vec::<char>::new()).collect::<Vec<_>>();

    map.lines().rev().skip(1).for_each(|line| {
        let mut stack = 0; //first crate is at index 1
        while stack < stacks.len() {
            let index = stack * 4 +1;
            let item = line.chars().nth(index).unwrap();
            if item != ' ' {
                stacks[stack].push(item);
            }
            stack += 1;
        }
    });
    dbg!(stacks.clone());

    moves.lines()
        .for_each(|action| {
           let parts = action.split_whitespace().map(|x| x.parse::<usize>().unwrap_or_else(|_| 0)).filter(|x| *x != 0 ).take(3).collect::<Vec<_>>();
           let (amount, from, to) = (parts[0], parts[1], parts[2]);
           let taken = (0..amount).map(|_| {
               stacks[from-1].pop().unwrap()
           }).collect::<Vec<char>>();
           taken.iter().rev().for_each(|cargo| stacks[to-1].push(*cargo));
        });
    let result: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();
    return result;

}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "MCD");
    }
}
