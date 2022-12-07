use crate::day07::Line::{ChangeDirectory, DirectoryListing, FileListing, ListContents};
/// --- Day 7: No Space Left On Device ---
/// https://adventofcode.com/2022/day/7
use crate::get_lines;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use uuid::Uuid;

pub struct Session {
    file_system: FileSystem,
    working_directory: Uuid,
}

impl Session {
    pub fn find_directories_smaller_than(&self, max_size: usize) -> Vec<(Uuid, usize)> {
        let root = self
            .file_system
            .directories
            .get(&self.file_system.root)
            .unwrap(); // FIXME error handling
        self.file_system
            .find_directories_smaller_than(root, max_size)
            .0
    }
    pub fn find_directories_larger_than(&self, min_size: usize) -> Vec<(Uuid, usize)> {
        let root = self
            .file_system
            .directories
            .get(&self.file_system.root)
            .unwrap(); // FIXME error handling
        self.file_system
            .find_directories_larger_than(root, min_size)
            .0
    }
}

impl Default for Session {
    fn default() -> Self {
        let file_system = FileSystem::default();
        let working_directory = file_system.root;
        Self {
            file_system,
            working_directory,
        }
    }
}

pub struct FileSystem {
    directories: HashMap<Uuid, Directory>,
    parents: HashMap<Uuid, Uuid>,
    files: HashMap<Uuid, File>,
    root: Uuid,
}

impl FileSystem {
    pub fn consumed_space(&self) -> usize {
        self.directory_size(self.directories.get(&self.root).unwrap())
    }

    fn directory_size(&self, directory: &Directory) -> usize {
        let mut result = 0;
        for sub in directory.sub_directories.values() {
            let sub = self.directories.get(sub).unwrap();
            result += self.directory_size(sub);
        }
        for file in &directory.files {
            let file = self.files.get(file).unwrap();
            result += file.size;
        }
        result
    }

    pub fn insert_file(&mut self, parent_directory_id: Uuid, file: File) {
        let file_id = file.id;
        self.files.insert(file_id, file);
        // TODO should return Result
        let parent_directory = self
            .directories
            .get_mut(&parent_directory_id)
            .expect("No such parent directory");
        parent_directory.files.push(file_id);
        self.parents.insert(file_id, parent_directory_id);
    }

    pub fn insert_directory(&mut self, parent_directory_id: Uuid, directory: Directory) {
        let directory_id = directory.id;
        let directory_name = directory.name.clone();
        self.directories.insert(directory_id, directory);
        // TODO should return Result
        let parent_directory = self
            .directories
            .get_mut(&parent_directory_id)
            .expect("No such parent directory");
        parent_directory
            .sub_directories
            .insert(directory_name, directory_id);
        self.parents.insert(directory_id, parent_directory_id);
    }

    pub fn find_directories_smaller_than(
        &self,
        parent: &Directory,
        max_size: usize,
    ) -> (Vec<(Uuid, usize)>, usize) {
        let mut list = vec![];
        let mut total_size = 0;
        for sub_directory_id in parent.sub_directories.values() {
            let sub_directory = self.directories.get(sub_directory_id).unwrap(); // TODO better error handling
            let (items, sub_directory_size) =
                self.find_directories_smaller_than(sub_directory, max_size);
            total_size += sub_directory_size;
            list = vec![list, items].concat();
        }
        for file_id in &parent.files {
            if total_size > max_size {
                break;
            }
            let file = self.files.get(file_id).unwrap(); // TODO better error handling
            total_size += file.size;
        }
        if total_size < max_size {
            list.push((parent.id, total_size));
        }
        (list, total_size)
    }

    pub fn find_directories_larger_than(
        &self,
        parent: &Directory,
        min_size: usize,
    ) -> (Vec<(Uuid, usize)>, usize) {
        let mut list = vec![];
        let mut total_size = 0;
        for sub_directory_id in parent.sub_directories.values() {
            let sub_directory = self.directories.get(sub_directory_id).unwrap(); // TODO better error handling
            let (items, sub_directory_size) =
                self.find_directories_larger_than(sub_directory, min_size);
            total_size += sub_directory_size;
            list = vec![list, items].concat();
        }
        for file_id in &parent.files {
            let file = self.files.get(file_id).unwrap(); // TODO better error handling
            total_size += file.size;
        }
        if total_size >= min_size {
            list.push((parent.id, total_size));
        }
        (list, total_size)
    }
}

impl Default for FileSystem {
    fn default() -> Self {
        let root = Directory {
            id: Default::default(),
            name: "/".to_string(),
            sub_directories: Default::default(),
            files: Default::default(),
        };
        let root_id = root.id;
        let mut directories = HashMap::new();
        directories.insert(root_id, root);
        Self {
            directories,
            parents: HashMap::new(),
            files: HashMap::new(),
            root: root_id,
        }
    }
}

pub struct Directory {
    id: Uuid,
    name: String,
    sub_directories: HashMap<String, Uuid>,
    files: Vec<Uuid>,
}

impl Debug for Directory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Directory{{ name: {}, id: {} }}", self.name, self.id)
    }
}

pub struct File {
    id: Uuid,
    // name: String,
    size: usize,
}

#[derive(Debug)]
pub enum Line {
    ListContents,
    ChangeDirectory(String),
    DirectoryListing(String),
    FileListing(usize, String),
}

impl Line {
    pub fn execute(&self, session: &mut Session) {
        match self {
            ListContents => {}
            ChangeDirectory(target) => {
                if target == ".." {
                    let parent_id = session
                        .file_system
                        .parents
                        .get(&session.working_directory)
                        .unwrap();
                    session.working_directory = *parent_id;
                } else if target == "/" {
                    session.working_directory = session.file_system.root;
                } else {
                    let current = session
                        .file_system
                        .directories
                        .get(&session.working_directory)
                        .unwrap();
                    let target = current.sub_directories.get(target).unwrap_or_else(|| {
                        panic!(
                            "Directory {:?} does not contain a sub-directory named {}",
                            current, target
                        )
                    });
                    session.working_directory = *target;
                }
            }
            DirectoryListing(name) => {
                let directory = Directory {
                    id: Uuid::new_v4(),
                    name: name.to_string(),
                    sub_directories: Default::default(),
                    files: Default::default(),
                };
                session
                    .file_system
                    .insert_directory(session.working_directory, directory);
            }
            FileListing(size, _name) => {
                let file = File {
                    id: Uuid::new_v4(),
                    // name: _name.to_string(),
                    size: *size,
                };
                session
                    .file_system
                    .insert_file(session.working_directory, file);
            }
        }
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut components = line.split(' ');
        let first_token = components.next().unwrap(); // TODO error handling
        match first_token {
            "$" => {
                let second_token = components.next().unwrap(); // TODO error handling
                match second_token {
                    "cd" => {
                        let argument = components.next().unwrap().to_string();
                        Ok(ChangeDirectory(argument))
                    }
                    "ls" => Ok(ListContents),
                    other_command => {
                        eprintln!("Unknown command: {}", other_command);
                        panic!()
                    }
                }
            }
            "dir" => {
                let name = components.next().unwrap().to_string();
                Ok(DirectoryListing(name))
            }
            number => {
                let size = number.parse::<usize>().unwrap();
                let name = components.next().unwrap().to_string();
                Ok(FileListing(size, name))
            }
        }
    }
}

pub fn get_input() -> impl Iterator<Item = Line> {
    get_lines("day-07.txt")
        .map(|line| line.parse::<Line>())
        .map(Result::unwrap)
}

#[cfg(test)]
mod tests {

    use crate::day07::{get_input, Session};

    #[test]
    fn part1() {
        let mut session = Session::default();
        get_input().for_each(|line| line.execute(&mut session));
        let result: usize = session
            .find_directories_smaller_than(100_000)
            .iter()
            .map(|item| item.1)
            .sum();

        println!("Part 1: {}", result);
    }

    #[test]
    fn part2() {
        let mut session = Session::default();
        get_input().for_each(|line| line.execute(&mut session));
        let consumed = session.file_system.consumed_space();
        let unused = 70_000_000 - consumed;
        let required = 30_000_000 - unused;
        let result: usize = session
            .find_directories_larger_than(required)
            .iter()
            .map(|item| item.1)
            .min()
            .unwrap();

        println!("Part 2: {}", result);
    }
}
