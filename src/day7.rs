use crate::common::Day;

pub struct Day7;

pub struct Dir {
    name: String,
    parent: Option<usize>,
    files: Vec<File>,
    dirs: Vec<usize>,
}

impl Dir {
    fn add_file(&mut self, name: &str, size: usize) {
        self.files.push(File {
            name: name.to_string(),
            size,
        })
    }

    fn get_size(&self, dir_table: &DirTable) -> usize {
        let files_size: usize = self.files.iter().map(|x| x.size).sum();
        let children_size: usize = self
            .dirs
            .iter()
            .map(|&x| dir_table.dirs[x].get_size(&dir_table))
            .sum();
        files_size + children_size
    }
}

pub struct File {
    name: String,
    size: usize,
}

pub struct DirTable {
    dirs: Vec<Dir>,
}

impl DirTable {
    fn new() -> DirTable {
        DirTable { dirs: vec![] }
    }

    fn find_dir(&self, name: &str) -> Option<usize> {
        Some(
            self.dirs
                .iter()
                .enumerate()
                .find(|(_, x)| x.name == name)?
                .0,
        )
    }

    fn add_dir(&mut self, name: &str, parent: Option<usize>) {
        let dir_index = self.dirs.len();
        self.dirs.push(Dir {
            name: name.to_string(),
            parent: Some(parent.unwrap_or(0)),
            files: vec![],
            dirs: vec![],
        });

        if let Some(parent_index) = parent {
            self.dirs[parent_index].dirs.push(dir_index);
        }
    }

    fn get_directory_size(&self, dir_index: usize) -> usize {
        self.dirs[dir_index].get_size(&self)
    }
}

impl<'a> Day<'a> for Day7 {
    type Input = DirTable;
    type Output = usize;

    fn day_number() -> usize {
        7
    }

    fn part1(input: &Self::Input) -> Self::Output {
        let temp: Vec<(usize, String)> = input
            .dirs
            .iter()
            .map(|x| x.name.clone())
            .enumerate()
            .collect();
        println!("{:?}", temp);
        (0..input.dirs.len())
            .map(|x| input.get_directory_size(x))
            .filter(|&x| x <= 100000)
            .sum()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        0
    }

    fn parse(input: &'a str) -> Self::Input {
        let mut dir_table = DirTable::new();
        dir_table.add_dir("/", None);
        let mut cursor = 0;

        for line in input[0..input.len() - 1].lines().skip(1) {
            println!("Cursor: {}, line: {}", cursor, line);
            if line == "$ ls" {
                continue;
            } else if let Some(x) = line.strip_prefix("dir ") {
                dir_table.add_dir(x, Some(cursor));
            } else if let Some(x) = line.strip_prefix("$ cd ") {
                match x {
                    ".." => cursor = dir_table.dirs[cursor].parent.unwrap(),
                    y => cursor = dir_table.find_dir(y).unwrap(),
                }
            } else {
                let mut parts = line.split(' ');
                let size = parts.next().unwrap().parse::<usize>().unwrap();
                let name = parts.next().unwrap();
                dir_table.dirs[cursor].add_file(name, size);
            }
        }
        dir_table
    }
}
