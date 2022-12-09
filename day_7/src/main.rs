use std::collections::HashMap;

struct DirEntry {
    #[allow(dead_code)]
    files: HashMap<String, usize>,
    subdirs: HashMap<String, DirEntry>,
}

impl DirEntry {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            subdirs: HashMap::new(),
        }
    }
}

fn main() {
    let lines = aoc::read_lines("input.txt").expect("Couldn't open input.txt for writing");

    let mut root = DirEntry::new();
    let mut cur_dir_name: Vec<String> = Vec::new();

    for line in lines {
        match line {
            Ok(text) => {
                let mut tokens = text.split(' ');
                match tokens.next().unwrap() {
                    "$" => {
                        // New command
                        process_command(tokens, &mut root, &mut cur_dir_name);
                    }
                    "dir" => {
                        // ls, dir entry
                        try_add_subdir(get_dir(&mut root, &cur_dir_name), tokens.next().unwrap());
                    }
                    file_size => {
                        // ls, file entry
                        let file_size: usize = file_size.parse().unwrap();
                        let file_name = tokens.next().unwrap();
                        get_dir(&mut root, &cur_dir_name)
                            .files
                            .insert(file_name.into(), file_size);
                    }
                }
            }
            Err(_) => break,
        }
    }

    let mut total_less_than_100000 = 0;
    for_each_dir_in_tree(&root, &mut |dir_size| {
        if dir_size <= 100000 {
            total_less_than_100000 += dir_size;
        }
    });
    println!(
        "Total size of directories <= 100000: {}",
        total_less_than_100000
    );

    let space_used = calc_size(&root);
    let space_to_free = 30000000 - (70000000 - space_used);
    let mut smallest_suitable_dir: usize = space_used;

    for_each_dir_in_tree(&root, &mut |dir_size| {
        if dir_size < smallest_suitable_dir && dir_size >= space_to_free {
            smallest_suitable_dir = dir_size;
        }
    });

    println!("Size of smallest suitable directory: {}", smallest_suitable_dir);
}

fn for_each_dir_in_tree(dir: &DirEntry, pred: &mut impl FnMut(usize)) {
    let total = calc_size_files(dir) + calc_size_subdirs(dir);
    pred(total);

    for (_, dir_entry) in &dir.subdirs {
        for_each_dir_in_tree(dir_entry, pred);
    }
}

fn process_command(
    mut tokens: std::str::Split<char>,
    root: &mut DirEntry,
    cur_dir: &mut Vec<String>,
) {
    match tokens.next().unwrap() {
        "cd" => process_cd_command(tokens, root, cur_dir),
        "ls" => {} // Nothing to do
        cmd => panic!("Unknown command {}", cmd),
    }
}

fn process_cd_command(
    mut tokens: std::str::Split<char>,
    root: &mut DirEntry,
    cur_dir: &mut Vec<String>,
) {
    match tokens.next().unwrap() {
        "/" => cur_dir.clear(),
        ".." => {
            let _ = cur_dir.pop();
        }
        dir_name => {
            let cur_dir_entry = get_dir(root, cur_dir);
            try_add_subdir(cur_dir_entry, dir_name);
            cur_dir.push(dir_name.into());
        }
    };
}

fn get_dir<'a>(root: &'a mut DirEntry, cur_dir: &Vec<String>) -> &'a mut DirEntry {
    let mut result = root;

    for dir_name in cur_dir {
        result = result
            .subdirs
            .get_mut(dir_name)
            .unwrap_or_else(|| panic!("Couldn't resolve directory {:?}", cur_dir));
    }

    result
}

fn try_add_subdir(dir: &mut DirEntry, subdir_name: &str) {
    if !dir.subdirs.contains_key(subdir_name) {
        dir.subdirs.insert(subdir_name.into(), DirEntry::new());
    }
}

fn calc_size(dir: &DirEntry) -> usize {
    calc_size_files(&dir) + calc_size_subdirs(&dir)
}

fn calc_size_files(dir: &DirEntry) -> usize {
    dir.files.iter().map(|(_, size)| size).sum()
}

fn calc_size_subdirs(dir: &DirEntry) -> usize {
    dir.subdirs
        .iter()
        .fold(0, |accum, (_, entry)| accum + calc_size(&entry))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dir_with_files_100_size() -> DirEntry {
        DirEntry {
            files: HashMap::from([("1".into(), 40), ("2".into(), 60)]),
            subdirs: HashMap::new(),
        }
    }

    fn dir_with_files_300_size() -> DirEntry {
        DirEntry {
            files: HashMap::from([("1".into(), 100), ("2".into(), 200)]),
            subdirs: HashMap::new(),
        }
    }

    #[test]
    fn calc_size_works() {
        let entry = DirEntry {
            files: HashMap::from([("1".into(), 100), ("2".into(), 500)]),
            subdirs: HashMap::from([
                ("4".into(), dir_with_files_100_size()),
                ("5".into(), dir_with_files_300_size()),
            ]),
        };

        assert!(calc_size(&entry) == 1000);
    }
}
