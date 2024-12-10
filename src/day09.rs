use std::{fs, usize};

fn is_empty(blocks: &Vec<u32>) -> bool {
    for block in (0..blocks.len()).step_by(2) {
        if blocks[block] != 0 {
            return false;
        }
    }

    return true;
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut blocks: Vec<u32> = contents
        .trim()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .collect();
    // Edge cases - empty, 1 block with no empty
    let mut p_block_start: usize = 0;
    let mut p_block_end: usize = blocks.len() - 1;
    let mut p_block_empty: usize = 1;
    let mut pos = 0;
    let mut checksum = 0;

    // For each bit we process we mutate the blocks
    // By the end of it, blocks should be empty as we
    // processed everything
    while !is_empty(&blocks) {
        // Go by each cell count all from the start
        // and decrease the number as we move "bit by bit"
        if blocks[p_block_start] != 0 {
            // we move pointers in the flat alternating list
            // of file/empty list so each next odd index
            // is the double of file index (ID)
            checksum += pos * p_block_start / 2;
            blocks[p_block_start] -= 1;
            // Pos keeps track of the position of
            // the currently processed bit
            pos += 1;
        } else if blocks[p_block_empty] != 0 {
            // When we finish a file block, next one is
            // an empty block, so populate it with
            // file blocks from the end
            //
            // If the last positioned file is all
            // processed, move to the second last etc.
            if blocks[p_block_end] == 0 {
                p_block_end -= 2;
            }
            // Process one bit from the end
            checksum += pos * p_block_end / 2;
            blocks[p_block_end] -= 1;
            blocks[p_block_empty] -= 1;
            pos += 1;
        } else {
            // When we finished a group of file and empty blocks
            // move to the next pair
            p_block_start += 2;
            p_block_empty += 2;
        }
    }

    println!("Part 1 Answer: {checksum} \n");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut blocks: Vec<u32> = contents
        .trim()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .collect();
    // Edge cases - empty, 1 block with no empty
    let mut p_block_start: usize = 0;
    let mut p_block_empty: usize = 1;
    let mut pos = 0;
    let mut checksum = 0;

    while !is_empty(&blocks) {
        // Similarly to the first part, move in pairs
        // Main processing does the p_block_start pointer
        if blocks[p_block_start] != 0 {
            // We set 10 + digit to mark the empty bits
            // created by moving a file from the end
            // We need this to correctly keep track of the pos
            if blocks[p_block_start] >= 10 {
                // Update the pos by the length of the empty
                // space created by the moved file
                pos += blocks[p_block_start] as usize - 10;
                // We just need to process the pos, once it's done
                // we are done with this block so set 0
                blocks[p_block_start] = 0;
            } else {
                // If it's smaller than 10 - it's a normal file block,
                // handle as before
                checksum += pos * p_block_start / 2;
                blocks[p_block_start] -= 1;
                pos += 1;
            }
        } else if blocks[p_block_empty] != 0 {
            // Now we process the empty block. We move all the files we can to here
            // from last to first, for each moved file update block counts accordingly
            // calculate the checksum
            for p_block_end in (p_block_start..blocks.len()).step_by(2).rev() {
                if blocks[p_block_end] <= blocks[p_block_empty] {
                    for _ in 0..blocks[p_block_end] {
                        checksum += pos * p_block_end / 2;
                        pos += 1;
                    }
                    blocks[p_block_empty] -= blocks[p_block_end];
                    blocks[p_block_end] += 10;
                }
            }

            // We cannot fit anything in this empty space anymore,
            // so update the pos accordingly and then set it as processed (aka set it 0)
            pos += blocks[p_block_empty] as usize;
            blocks[p_block_empty] = 0;
        } else {
            // If we handled the pair or file/empty block, move on
            p_block_empty += 2;
            p_block_start += 2;
        }
    }

    println!("Part 2 Answer: {checksum} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
