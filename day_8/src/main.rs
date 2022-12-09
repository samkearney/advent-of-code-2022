fn main() {
    let lines = aoc::read_lines("input.txt").expect("Couldn't open input.txt for reading");

    let mut trees: Vec<Vec<u32>> = Vec::new();

    for line in lines {
        match line {
            Ok(text) => {
                trees.push(build_tree_line(&text));
            }
            Err(_) => break,
        }
    }

    let mut num_visible = 0;
    let mut max_score = 0;

    for (row_index, tree_row) in trees.iter().enumerate() {
        assert!(tree_row.len() == trees.len());

        for col_index in 0..tree_row.len() {
            let (visible, score) = check_visibility_and_score(&trees, row_index, col_index);

            if visible {
                num_visible += 1;
            }

            if score > max_score {
                max_score = score;
            }
        }
    }

    println!("Num visible: {}", num_visible);
    println!("Max score: {}", max_score)
}

fn build_tree_line(text: &str) -> Vec<u32> {
    text.chars()
        .map(|char| char.to_digit(10).unwrap())
        .collect()
}

fn check_visibility_and_score(
    trees: &[Vec<u32>],
    row_index: usize,
    col_index: usize,
) -> (bool, u32) {
    let tree_height: u32 = trees[row_index][col_index];
    let tree_row = &trees[row_index];
    let tree_col: Vec<u32> = trees.iter().map(|tree_row| tree_row[col_index]).collect();

    let (left_vis, left_score) = get_score_left(tree_height, &tree_row[..col_index]);
    let (right_vis, right_score) = get_score_right(tree_height, &tree_row[col_index + 1..]);
    let (up_vis, up_score) = get_score_left(tree_height, &tree_col[..row_index]);
    let (down_vis, down_score) = get_score_right(tree_height, &tree_col[row_index + 1..]);

    (
        left_vis || right_vis || up_vis || down_vis,
        left_score * right_score * up_score * down_score,
    )
}

fn get_score_left(tree_height: u32, trees: &[u32]) -> (bool, u32) {
    let mut result = 0;

    for tree in trees.iter().rev() {
        result += 1;
        if *tree >= tree_height {
            return (false, result);
        }
    }

    (true, result)
}

fn get_score_right(tree_height: u32, trees: &[u32]) -> (bool, u32) {
    let mut result = 0;

    for tree in trees.iter() {
        result += 1;
        if *tree >= tree_height {
            return (false, result);
        }
    }

    (true, result)
}
