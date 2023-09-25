use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::BTreeMap;
#[derive(Debug, Clone)]
struct Directory {
    name: String,
    files: Vec<File>,
    parent: Option<Weak<RefCell<Directory>>>,
    dirs: Vec<Rc<RefCell<Directory>>>,
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: u32,
}

impl From<&str> for File {
    fn from(s: &str) -> File {
        let (size, name) = s.split_once(' ').unwrap();
        File {
            name: name.to_owned(),
            size: size.parse().unwrap(),
        }
    }
}

impl Directory {
    fn new(name: &str, parent: Option<Weak<RefCell<Directory>>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Directory {
            name: name.to_string(),
            files: Vec::new(),
            parent,
            dirs: Vec::new(),
        }))
    }

    fn calculate_size(&self) -> u32 {
        let mut sum = 0;

        for file in &self.files {
            sum += file.size;
        }

        for dir in &self.dirs {
            sum += dir.borrow_mut().calculate_size();
        }

        sum
    }

    fn get_below(&self, slash_size: u32) -> Vec<u32> {
        let mut belows = vec![];
    
        for dir in &self.dirs {
    
            let dir_size = dir.borrow_mut().calculate_size();
            if (slash_size - dir_size) < 40_000_000 {
                println!("dir_size = {:?}", dir_size);
                belows.push(dir_size);
            }
            
            let sub_below = dir.borrow().get_below(slash_size);
            belows.extend(sub_below);
        }
    
        belows
    }

    fn calculate_size_below(&self, n: u32) -> u32 {
        fn inner(cwd: &Directory, n: u32) -> (u32, u32) {
            let mut total_sum = 0;
            let mut lower_sum = 0;

            for file in &cwd.files {
                total_sum += file.size;
            }

            for dir in &cwd.dirs {
                let (lower, total) = inner(&dir.borrow(), n);
                total_sum += total;
                lower_sum += lower;
            }

            if total_sum <= n {
                lower_sum += total_sum;
            }

            (lower_sum, total_sum)
        }

        inner(&self, n).0
    }

    fn add_file(&mut self, file: impl Into<File>) {
        self.files.push(file.into());
    }

    fn add_subdir(parent: &Rc<RefCell<Self>>, name: &str) -> Rc<RefCell<Directory>> {
        for dir in &parent.borrow().dirs {
            if dir.borrow().name == name {
                return Rc::clone(&dir);
            }
        }

        let new_dir = Directory::new(name, Some(Rc::downgrade(&parent)));
        parent.borrow_mut().dirs.push(Rc::clone(&new_dir));

        new_dir
    }
}
fn main() {
    let input = include_str!("../data/day7.txt");
    let instructions: Vec<&str> = input.split("\r\n").collect();
    let mut pwd = Directory::new("/", None);
    let root = Rc::clone(&pwd);

    for item in instructions {
        if item.starts_with("$ ") {
            let item = item.strip_prefix("$ ").unwrap();
            if item.starts_with("ls") {
            } else if item.starts_with("cd .") {
                let new_pwd = pwd.borrow().parent.clone().map(|d| d.upgrade());
                if let Some(Some(p)) = new_pwd {
                    pwd = p;
                }
            } else if item.contains(" /") {
                eprintln!("at root /");
                pwd = Rc::clone(&root);
            } else  {
                pwd = Directory::add_subdir(&pwd, item.strip_prefix("cd ").unwrap());
            }
            
        } else if item.starts_with("dir") {
        } else {
            pwd.borrow_mut().add_file(item);
        }
    }
    println!("Size of /: {}", root.borrow().calculate_size());
    let size = root.borrow().calculate_size();
    let mut below = root.borrow().get_below(size);  
    eprintln!("below = {:?}", below.sort());
    let result = root.borrow().calculate_size_below(100_000);
    println!("Answer: {}", result);

}