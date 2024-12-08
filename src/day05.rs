use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> usize {
    let (comes_before, updates) = parse(input);
    updates
        .iter()
        .filter(|update| is_correct(update, &comes_before))
        .map(|update| update.get(update.len() / 2).unwrap())
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let (comes_before, updates) = parse(input);
    updates
        .iter()
        .filter(|update| !is_correct(update, &comes_before))
        .map(|update| correct(update, &comes_before))
        .map(|update| *update.get(update.len() / 2).unwrap())
        .sum()
}

type ComesBefore = HashMap<usize, HashSet<usize>>;
type Update = Vec<usize>;

fn parse(input: &str) -> (ComesBefore, Vec<Update>) {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let mut comes_before = HashMap::new();
    rules
        .lines()
        .map(|line| line.split_once("|").unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .for_each(|(a, b)| {
            comes_before.entry(a).or_insert(HashSet::new()).insert(b);
        });

    let updates = updates
        .lines()
        .map(|line| {
            line.split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    (comes_before, updates)
}

fn is_correct(update: &Update, comes_before: &ComesBefore) -> bool {
    update.iter().enumerate().all(|(i, page)| {
        update.iter().skip(i + 1).all(|next| {
            comes_before
                .get(page)
                .map(|after| after.contains(next))
                .unwrap_or(false)
        })
    })
}
fn correct(update: &Update, comes_before: &ComesBefore) -> Update {
    let mut update = update.clone();
    let mut i = 0;
    while i < update.len() {
        let mut swap_index = None;
        for j in (i + 1)..update.len() {
            if !comes_before
                .get(&update[i])
                .map(|after| after.contains(&update[j]))
                .unwrap_or(false)
            {
                swap_index = Some(j);
                break;
            }
        }
        if let Some(swap) = swap_index {
            update.swap(i, swap);
        } else {
            i += 1;
        }
    }
    update
}

#[cfg(test)]
mod tests {
    use super::correct;
    use super::parse;
    use super::ComesBefore;

    macro_rules! test_update {
        ($input:expr, $exp:expr) => {
            let comes_before = process_input(include_str!("../inputs/day05.in"));
            let update = $input;
            let corrected = correct(&update, &comes_before);
            assert_eq!(corrected, $exp);
        };
    }

    fn process_input(input: &str) -> ComesBefore {
        let (comes_before, updates) = parse(input);
        comes_before
    }

    #[test]
    fn test_correction_base() {
        test_update!(vec![75, 97, 47, 61, 53], vec![97, 75, 47, 61, 53]);
    }

    #[test]
    fn test_correction_2() {
        test_update!(vec![61, 13, 29], vec![61, 29, 13]);
    }
    #[test]
    fn test_correction_3() {
        test_update!(vec![97, 13, 75, 29, 47], vec![97, 75, 47, 29, 13]);
    }
}
