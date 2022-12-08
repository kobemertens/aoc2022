use crate::common::Day;

pub struct Day7;

pub struct Dir {
    name: String,
    parent: usize,
    files: Vec<File>,
    dirs: Vec<usize>,
}

impl Dir {
    fn add_file(&mut self, name: &str, size: usize) {
        self.files.push(File {
            _name: name.to_string(),
            size,
        })
    }

    fn get_size(&self, file_system: &FileSystem) -> usize {
        let files_size: usize = self.files.iter().map(|x| x.size).sum();
        let children_size: usize = self
            .dirs
            .iter()
            .map(|&x| file_system.dirs[x].get_size(&file_system))
            .sum();
        files_size + children_size
    }
}

pub struct File {
    _name: String,
    size: usize,
}

pub struct FileSystem {
    dirs: Vec<Dir>,
}

impl FileSystem {
    fn new() -> FileSystem {
        FileSystem {
            dirs: vec![Dir {
                name: "/".to_string(),
                parent: 0,
                files: vec![],
                dirs: vec![],
            }],
        }
    }

    fn add_dir(&mut self, name: &str, parent: usize) {
        let dir_index = self.dirs.len();
        self.dirs.push(Dir {
            name: name.to_string(),
            parent,
            files: vec![],
            dirs: vec![],
        });

        self.dirs[parent].dirs.push(dir_index);
    }

    fn get_directory_size(&self, dir_index: usize) -> usize {
        self.dirs[dir_index].get_size(&self)
    }

    fn find_dir(&self, name: &str, parent: usize) -> usize {
        self.dirs[parent]
            .dirs
            .iter()
            .map(|&x| (x, &self.dirs[x]))
            .find(|x| x.1.name == name)
            .unwrap()
            .0
    }
}

impl<'a> Day<'a> for Day7 {
    type Input = FileSystem;
    type Output = usize;

    fn day_number() -> usize {
        7
    }

    fn part1(input: &Self::Input) -> Self::Output {
        (0..input.dirs.len())
            .map(|x| input.get_directory_size(x))
            .filter(|&x| x <= 100000)
            .sum()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let capacity = 70000000;
        let needed = 30000000;
        let taken = input.get_directory_size(0);
        let free = capacity - taken;
        let space_to_free = needed - free;

        (0..input.dirs.len())
            .map(|x| input.get_directory_size(x))
            .filter(|&x| x >= space_to_free)
            .min()
            .unwrap()
    }

    fn parse(input: &'a str) -> Self::Input {
        let mut file_system = FileSystem::new();
        let mut cursor = 0;

        for line in input[0..input.len() - 1].lines().skip(1) {
            if line == "$ ls" {
                continue;
            } else if let Some(x) = line.strip_prefix("dir ") {
                file_system.add_dir(x, cursor);
            } else if let Some(x) = line.strip_prefix("$ cd ") {
                match x {
                    ".." => cursor = file_system.dirs[cursor].parent,
                    y => cursor = file_system.find_dir(y, cursor),
                }
            } else {
                let mut parts = line.split(' ');
                let size = parts.next().unwrap().parse::<usize>().unwrap();
                let name = parts.next().unwrap();
                file_system.dirs[cursor].add_file(name, size);
            }
        }
        file_system
    }
}
