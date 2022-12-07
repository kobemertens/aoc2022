use crate::common::Day;

pub struct Day7;

pub struct Dir<'a> {
    name: String,
    parent: Option<&'a Dir<'a>>,
    files: Vec<File>,
    dirs: Vec<Dir<'a>>,
}

impl<'a> Dir<'a> {
    fn new(name: &str) -> Dir<'a> {
        Dir {
            name: name.to_string(),
            parent: None,
            files: vec![],
            dirs: vec![],
        }
    }

    fn add_dir(&mut self, name: &str) {
        self.dirs.push(Dir {
            name: name.to_string(),
            parent: Some(&self),
            files: vec![],
            dirs: vec![],
        });
    }

    fn find_dir(&self, name: &str) -> Option<&Dir> {
        self.dirs.iter().find(|d| d.name == name)
    }

    fn add_file(&mut self, name: &str, size: usize) {
        self.files.push(File {
            name: name.to_string(),
            size,
        })
    }

    fn get_size(&self) -> usize {
        let files_size: usize = self.files.iter().map(|x| x.size).sum();
        let children_size: usize = self.dirs.iter().map(|x| x.get_size()).sum();
        files_size + children_size
    }
}

pub struct File {
    name: String,
    size: usize,
}

impl<'a> Day<'a> for Day7 {
    type Input = Dir<'a>;
    type Output = usize;

    fn day_number() -> usize {
        7
    }

    fn part1(input: &Self::Input) -> Self::Output {
        input.get_size()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        0
    }

    fn parse(input: &'a str) -> Self::Input {
        let mut root = Dir::new("/");
        let mut cursor = &mut root;
        for line in input.lines().skip(1) {
            if line == "$ ls" {
                continue;
            } else if let Some(x) = line.strip_prefix("dir ") {
                cursor.add_dir(x);
            } else if let Some(x) = line.strip_prefix("$ cd ") {
                match x {
                    ".." => cursor = &mut cursor.parent.unwrap(),
                    y => cursor = &mut cursor.find_dir(y).unwrap(),
                }
            } else {
                let mut parts = line.split(' ');
                let name = parts.next().unwrap();
                let size = parts.next().unwrap().parse::<usize>().unwrap();
                cursor.add_file(name, size);
            }
        }
        root
    }
}
