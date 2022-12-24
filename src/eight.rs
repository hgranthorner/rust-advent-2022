pub const SAMPLE: &str = "30373
25512
65332
33549
35390";

#[derive(Clone, Copy, Debug)]
pub struct Tree {
    pub height: u32,
    pub visible: bool,
}

impl Tree {
    pub fn new(height: u32) -> Self {
        Self {
            height,
            visible: false,
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Vec<Tree>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .into_iter()
                .map(|c| Tree::new(c.to_digit(10).unwrap()))
                .collect()
        })
        .collect()
}

fn mark_visible_trees(data: &mut [Tree]) {
    let mut highest = 0;
    for tree in data {
        if tree.height <= highest {
            break;
        }
        if !tree.visible {
            tree.visible = true;
            highest = tree.height;
        }
    }
}

fn solve_first(input: &str) -> usize {
	let mut rows_of_trees = parse_input(input);
	for row in rows_of_trees.iter_mut() {
		mark_visible_trees(row);
		row.reverse();
		mark_visible_trees(row);
	}
	let mut cols_of_trees = Vec::<Vec<Tree>>::new();
	let row_len = rows_of_trees.first().unwrap().len();
	for i in 0..row_len {
		cols_of_trees.push(Vec::with_capacity(rows_of_trees.len()));
		for row in &rows_of_trees {
			let tree = *row.get(i).unwrap();
			cols_of_trees.last_mut().unwrap().push(tree);
		}
	}
	for col in cols_of_trees.iter_mut() {
		mark_visible_trees(col);
		col.reverse();
		mark_visible_trees(col);
	}
	println!("{:?}", cols_of_trees);
	cols_of_trees.into_iter().flatten().filter(|t| t.visible).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let parsed = parse_input(SAMPLE);
        assert_eq!(5, parsed.len());
        assert_eq!(5, parsed.first().unwrap().len());
    }

    #[test]
    fn test_mark_visible_trees() {
        let mut data: Vec<Tree> = vec![1_u32, 2, 3, 1, 2].into_iter().map(Tree::new).collect();
        mark_visible_trees(&mut data);
        assert_eq!(3, data.into_iter().filter(|t| t.visible).count());
    }

    #[test]
    fn test_solve_first() {
    	let result = solve_first(SAMPLE);
    	assert_eq!(21, result);
    }
}
