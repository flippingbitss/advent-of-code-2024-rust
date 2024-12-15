pub fn part_one(input: &str) -> usize {
    let input = input.trim_end();
    let total_size = input.chars().filter_map(|c| c.to_digit(10)).sum::<u32>() as usize;

    let mut disk = vec![-1 as isize; total_size];
    let mut disk_idx = 0;
    let mut file_id = 0;
    for (i, c) in input.char_indices() {
        let size = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            for _ in 0..size {
                disk[disk_idx] = file_id;
                disk_idx += 1;
            }
            file_id += 1;
        } else {
            disk_idx += size;
        }
    }

    let mut right = disk.len() - 1;
    let mut left = 0;

    loop {
        if disk[left] != -1 {
            left += 1;
            continue;
        }
        if disk[right] == -1 {
            right -= 1;
            continue;
        }

        if left >= right {
            break;
        }

        disk.swap(left, right);
    }

    disk.into_iter()
        .take_while(|&x| x > -1)
        .enumerate()
        .map(|(i, x)| i * (x as usize))
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let input = input.trim_end();
    let total_size = input.chars().filter_map(|c| c.to_digit(10)).sum::<u32>() as usize;

    let mut disk = vec![-1 as isize; total_size];
    let mut disk_idx: usize = 0;
    let mut file_id: usize = 0;

    let mut spaces = vec![0; total_size];
    let mut blocks = vec![0; total_size];

    for (i, c) in input.char_indices() {
        let size = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            blocks[disk_idx] = size;
            for _ in 0..size {
                disk[disk_idx] = file_id as isize;
                disk_idx += 1;
            }
            file_id += 1;
        } else {
            spaces[disk_idx] = size;
            disk_idx += size;
        }
    }
    let mut right = total_size - 1;
    while right > 0 {
        let block_size = blocks[right];
        if block_size > 0 {
            if let Some((avail_idx, avail_space)) = spaces
                .iter()
                .enumerate()
                .find(|(i, &s)| *i < right && s >= block_size)
            {
                for i in 0..block_size {
                    disk.swap(avail_idx + i, right + i);
                }

                spaces[avail_idx + block_size] += avail_space - block_size;
                spaces[avail_idx] = 0;
            }
        }

        right -= 1;
    }
    disk.into_iter()
        .enumerate()
        .filter(|(_, x)| *x > -1)
        .map(|(i, x)| i * (x as usize))
        .sum()
}
