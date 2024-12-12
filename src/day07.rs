type Num = usize;

pub fn part_one(input: &str) -> Num {
    solve(input, is_solved)
}

pub fn part_two(input: &str) -> Num {
    solve(input, is_solved_with_concat)
}

fn parse(input: &str) -> Vec<(Num, Vec<Num>)> {
    input
        .lines()
        .map(|line| {
            line.split_once(": ")
                .map(|(result, nums)| {
                    (
                        result.parse::<Num>().unwrap(),
                        nums.split_whitespace()
                            .map(|n| n.parse::<Num>().unwrap())
                            .collect(),
                    )
                })
                .unwrap()
        })
        .collect()
}

fn concatenate(a: Num, b: Num) -> Num {
    let mut zeroes = 0;
    let mut temp = b;
    for i in 0.. {
        if temp == 0 {
            zeroes = i;
            break;
        }
        temp /= 10;
    }

    a * ((10 as Num).pow(zeroes)) + b
}

fn is_solved(nums: &[Num], accum: Num, target: Num) -> bool {
    if accum == target && nums.is_empty() {
        return true;
    }

    if nums.is_empty() {
        return false;
    }

    if let Some(n) = nums.first() {
        is_solved(&nums[1..], accum * n, target) || is_solved(&nums[1..], accum + n, target)
    } else {
        false
    }
}

fn is_solved_with_concat(nums: &[Num], accum: Num, target: Num) -> bool {
    if accum == target && nums.is_empty() {
        return true;
    }

    if nums.is_empty() {
        return false;
    }

    if let Some(n) = nums.first() {
        is_solved_with_concat(&nums[1..], accum * n, target)
            || is_solved_with_concat(&nums[1..], accum + n, target)
            || is_solved_with_concat(&nums[1..], concatenate(accum, *n), target)
    } else {
        false
    }
}

fn solve<F: Fn(&[Num], Num, Num) -> bool>(input: &str, solver: F) -> Num {
    let equations = parse(input);
    let mut result: Num = 0;
    for (target, nums) in equations {
        let accum = nums[0];
        let rest = &nums[1..];
        if solver(rest, accum, target) {
            result += target as Num;
        }
    }
    result
}
