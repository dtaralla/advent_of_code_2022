use lazy_static::lazy_static;
use regex::Regex;
use std::cell::RefCell;
use std::rc::Rc;

enum CdArg<'a> {
    Root,
    Parent,
    Child(&'a str),
}

impl<'a> From<&'a str> for CdArg<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "/" => Self::Root,
            ".." => Self::Parent,
            s => Self::Child(s),
        }
    }
}

enum Command<'a> {
    Cd(CdArg<'a>),
    Ls,
}

impl<'a> From<&'a str> for Command<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            s if s.starts_with("cd") => Self::Cd(CdArg::from(&s[3..])),
            s if s.starts_with("ls") => Self::Ls,
            s => panic!("Unsupported command: {s}"),
        }
    }
}

trait FsElem<'a> {
    fn matches(&self, node_name: &str) -> bool;
}

struct Directory<'a> {
    size: usize,
    name: &'a str,
    parent: Option<Rc<RefCell<Self>>>,
    child_files: Vec<Rc<RefCell<File<'a>>>>,
    child_folders: Vec<Rc<RefCell<Directory<'a>>>>,
}

impl<'a> From<&'a str> for Directory<'a> {
    fn from(value: &'a str) -> Self {
        Directory {
            size: 0,
            name: value,
            parent: None,
            child_files: vec![],
            child_folders: vec![],
        }
    }
}

impl<'a> FsElem<'a> for Directory<'a> {
    fn matches(&self, node_name: &str) -> bool {
        *self.name == *node_name
    }
}

impl<'a> Directory<'a> {
    fn add_child_file(&mut self, child: File<'a>) {
        self.size += child.size;

        let mut p = self.parent.clone();
        while let Some(d) = p {
            d.borrow_mut().size += child.size;
            p = d.borrow().parent.clone();
        }

        self.child_files.push(Rc::new(RefCell::new(child)));
    }

    fn add_child_folder(&mut self, child: Directory<'a>) {
        assert!(child.child_files.is_empty());
        self.child_folders.push(Rc::new(RefCell::new(child)));
    }

    fn find(this: Rc<RefCell<Self>>, node: &str, recurse: bool) -> Option<Rc<RefCell<Self>>> {
        if recurse {
            for fse in &this.borrow().child_folders {
                let found = Directory::find(fse.clone(), node, true);
                if found.is_some() {
                    return found;
                }
            }
        } else {
            for fse in &this.borrow().child_folders {
                if fse.borrow().matches(node) {
                    return Some(fse.clone());
                }
            }
        }

        None
    }
}

struct DirWalker<'a> {
    init: bool,
    folder: Rc<RefCell<Directory<'a>>>,
    cur_folder: usize,
    cur_folder_walker: Option<Rc<RefCell<DirWalker<'a>>>>,
    cur_folder_level: usize,
}

impl<'a> DirWalker<'a> {
    fn new(fstree: Rc<RefCell<Directory<'a>>>, include_root: bool) -> Self {
        Self {
            init: !include_root,
            folder: fstree,
            cur_folder: 0,
            cur_folder_walker: None,
            cur_folder_level: 0,
        }
    }

    fn current_nesting_level(&self) -> usize {
        self.cur_folder_level
    }
}

impl<'a> Iterator for DirWalker<'a> {
    type Item = Rc<RefCell<Directory<'a>>>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.init {
            self.init = true;
            return Some(self.folder.clone());
        }

        // If we were iterating on a child folder, let the child folder's walker do its job
        if self.cur_folder_walker.is_some() {
            let ret = self.cur_folder_walker.clone().unwrap().borrow_mut().next();
            if ret.is_none() {
                // Child folder has finished iterating; destroy sub-walker
                self.cur_folder_walker = None;
                self.cur_folder_level = 1;
            } else {
                self.cur_folder_level = 1 + &self
                    .cur_folder_walker
                    .clone()
                    .unwrap()
                    .borrow()
                    .cur_folder_level;
                return ret;
            }
        }

        // If we still have folders to iterate recursively, setup a sub-walker for the child folder
        if let Some(d) = self.folder.borrow().child_folders.get(self.cur_folder) {
            self.cur_folder += 1;
            self.cur_folder_walker = Some(Rc::new(RefCell::new(Self::new(d.clone(), false))));
            self.cur_folder_level = 1;
            return Some(d.clone());
        }

        // No more folders!
        self.cur_folder_level = 0;
        None
    }
}

struct FileWalker<'a> {
    folder: Rc<RefCell<Directory<'a>>>,
    cur_file: usize,
    cur_folder: usize,
    cur_folder_walker: Option<Rc<RefCell<FileWalker<'a>>>>,
}

impl<'a> FileWalker<'a> {
    fn new(fstree: Rc<RefCell<Directory<'a>>>) -> Self {
        Self {
            folder: fstree,
            cur_file: 0,
            cur_folder: 0,
            cur_folder_walker: None,
        }
    }

    fn next_from_subwalker(&mut self) -> Option<Rc<RefCell<File<'a>>>> {
        let ret = self.cur_folder_walker.clone().unwrap().borrow_mut().next();
        if ret.is_none() {
            // Child folder has finished iterating; destroy sub-walker
            self.cur_folder_walker = None;
        }
        return ret;
    }
}

impl<'a> Iterator for FileWalker<'a> {
    type Item = Rc<RefCell<File<'a>>>;

    fn next(&mut self) -> Option<Self::Item> {
        // If we were iterating on a child folder, let the child folder's walker do its job
        if self.cur_folder_walker.is_some() {
            let ret = self.next_from_subwalker();
            if ret.is_some() {
                return ret;
            }
        }

        // If we still have files to iterate, return next
        if let Some(f) = self.folder.borrow().child_files.get(self.cur_file) {
            self.cur_file += 1;
            return Some(f.clone());
        }

        // If we still have folders to iterate recursively, setup a sub-walker for the next child
        // folder *with contents* and return its first result.
        while let Some(d) = self
            .folder
            .clone()
            .borrow()
            .child_folders
            .get(self.cur_folder)
        {
            self.cur_folder += 1;
            self.cur_folder_walker = Some(Rc::new(RefCell::new(Self::new(d.clone()))));
            let ret = self.next_from_subwalker();
            if ret.is_some() {
                return ret;
            }
        }

        // No more files, no more folders!
        None
    }
}

struct File<'a> {
    size: usize,
    name: &'a str,
}

impl<'a> FsElem<'a> for File<'a> {
    fn matches(&self, node_name: &str) -> bool {
        *self.name == *node_name
    }
}

impl<'a> From<&'a str> for File<'a> {
    fn from(value: &'a str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+) (.*)").unwrap();
        }

        let matches = RE.captures(value).unwrap();
        File {
            size: matches.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            name: matches.get(2).unwrap().as_str(),
        }
    }
}

enum InputLine<'a> {
    Command(Command<'a>),
    Directory(Directory<'a>),
    File(File<'a>),
}

impl<'a> From<&'a str> for InputLine<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            s if s.starts_with('$') => Self::Command(Command::from(&s[2..])),
            s if s.starts_with("dir") => Self::Directory(Directory::from(&s[4..])),
            s => Self::File(File::from(s)),
        }
    }
}

// To help with debugging!
fn print_dir(fstree: Rc<RefCell<Directory>>) {
    let mut karen = DirWalker::new(fstree.clone(), true);
    while let Some(d) = karen.next() {
        let lvl = karen.current_nesting_level();
        println!(
            "{}{}: {}",
            "|----".repeat(lvl),
            d.borrow().name,
            d.borrow().size
        );
    }
}

fn build_fstree(input: &str) -> anyhow::Result<Rc<RefCell<Directory>>> {
    let fstree = Rc::new(RefCell::new(Directory {
        size: 0,
        name: "/",
        parent: None,
        child_files: vec![],
        child_folders: vec![],
    }));

    let mut cur_dir = fstree.clone();

    for line in input.lines() {
        match InputLine::from(line) {
            InputLine::Command(Command::Cd(CdArg::Root)) => {
                cur_dir = fstree.clone();
            }
            InputLine::Command(Command::Cd(CdArg::Parent)) => {
                cur_dir = cur_dir.clone().borrow().parent.clone().unwrap();
            }
            InputLine::Command(Command::Cd(CdArg::Child(dir_name))) => {
                cur_dir = Directory::find(cur_dir.clone(), dir_name, false).unwrap();
            }
            InputLine::Directory(mut d) => {
                d.parent = Some(cur_dir.clone());
                cur_dir.borrow_mut().add_child_folder(d);
            }
            InputLine::File(f) => {
                cur_dir.borrow_mut().add_child_file(f);
            }
            InputLine::Command(Command::Ls) => {}
        }
    }

    Ok(fstree)
}

pub fn run(input: &str) -> anyhow::Result<String> {
    let fstree = build_fstree(input)?;

    let mut sum = 0;
    for d in DirWalker::new(fstree.clone(), true) {
        if d.borrow().size <= 100000 {
            sum += d.borrow().size;
        }
    }

    print_dir(fstree.clone());

    Ok(sum.to_string())
}

pub fn run2(input: &str) -> anyhow::Result<String> {
    const TARGET_FREE_SIZE: usize = 30_000_000;

    let fstree = build_fstree(input)?;

    assert!(70_000_000 >= fstree.borrow().size);
    let initial_free_size = 70_000_000 - fstree.borrow().size;

    let mut smallest_dir_size_to_rm = usize::MAX;
    for d in DirWalker::new(fstree.clone(), true) {
        let cur_f_size = d.borrow().size;
        if initial_free_size + cur_f_size >= TARGET_FREE_SIZE
            && cur_f_size < smallest_dir_size_to_rm
        {
            smallest_dir_size_to_rm = cur_f_size;
        }
    }

    if smallest_dir_size_to_rm > TARGET_FREE_SIZE {
        anyhow::bail!(
            "Couldn't find any directory big enough to delete to reach {} free space",
            TARGET_FREE_SIZE
        );
    }

    Ok(smallest_dir_size_to_rm.to_string())
}
