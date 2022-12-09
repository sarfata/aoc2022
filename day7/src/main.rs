use std::cell::Cell;
use std::cell::RefCell;
use std::env;
use std::fs;
use std::rc::Rc;

mod listing;
use listing::InputLine;
use std::collections::HashMap;

#[derive(Debug)]
struct FileTree {
    subdirectories: RefCell<HashMap<String, Rc<FileTree>>>,
    filesize: Cell<usize>,
}

impl FileTree {
    fn from() -> FileTree {
        FileTree {
            subdirectories: RefCell::from(HashMap::new()),
            filesize: Cell::from(0),
        }
    }
    fn pretty(&self) {
        Self::pretty_i("/", self, "".to_owned());
    }
    fn pretty_i(name: &str, tree: &FileTree, indent: String) {
        println!(
            "{indent}{name} \t\t{} ({})",
            tree.size(),
            tree.filesize.get()
        );
        tree.subdirectories
            .borrow()
            .iter()
            .for_each(|(n, t)| Self::pretty_i(n, t, indent.clone() + "  "))
    }
    fn size(&self) -> usize {
        let mut size = self.filesize.get();
        if self.subdirectories.borrow().len() > 0 {
            let subdirs_size: usize = self
                .subdirectories
                .borrow()
                .iter()
                .map(|(_n, t)| t.size())
                .sum();
            size += subdirs_size;
        }
        size
    }

    fn find_dirs_i(&self, v: &mut Vec<usize>) {
        self.subdirectories.borrow().iter().for_each(|(_n, t)| {
            v.push(t.size());
            t.find_dirs_i(v);
        })
    }
    fn find_dirs(&self) -> Vec<usize> {
        let mut v = vec![];
        self.find_dirs_i(&mut v);
        v
    }
    fn less_than_10000(&self) -> usize {
        self.find_dirs()
            .iter()
            .map(|s| if *s <= 100000 { s } else { &0 })
            .sum()
    }
}

/*
*
*/
fn build_tree(line_it: &mut std::slice::Iter<InputLine>) -> Result<Rc<FileTree>, &'static str> {
    let root = Rc::from(FileTree::from());

    let mut dstack = vec![root.clone()];

    while let Some(line) = line_it.next() {
        match line {
            InputLine::CdRoot => {
                dstack = vec![root.clone()];
            }
            InputLine::CdFolder { s } => match s.as_str() {
                ".." => {
                    dstack.pop();
                }
                _ => {
                    let mut subdir = Rc::from(FileTree::from());
                    if let Some(cwd) = dstack.last() {
                        let mut subdirs = cwd.subdirectories.borrow_mut();
                        if let Some(subd) = subdirs.get(s) {
                            println!("Revisiting already visited folder {s}");
                            subdir = subd.clone();
                        } else {
                            subdirs.insert(s.clone(), subdir.clone());
                        }
                    }
                    dstack.push(subdir);
                }
            },
            InputLine::ListingOutput(listing::ListingOutput::File { name: _, size }) => {
                if let Some(cwd) = dstack.last() {
                    let new_size = cwd.filesize.get() + size;
                    cwd.filesize.set(new_size);
                }
            }
            InputLine::LsCmd => {
                if let Some(cwd) = dstack.last() {
                    cwd.filesize.set(0);
                }
            }
            InputLine::ListingOutput(listing::ListingOutput::Directory { name: _ }) => (
                /* Ignore directory listing. We will only include them if user visits them. */
            ),
        }
    }

    Ok(root.clone())
}

fn main() -> Result<(), &'static str> {
    let filepath = env::args().nth(1).unwrap_or(String::from("input"));
    let data = fs::read_to_string(filepath).map_err(|_| "Unable to read file")?;

    let parsed = data
        .lines()
        .filter_map(|l| l.try_into().ok())
        .collect::<Vec<InputLine>>();

    let tree = build_tree(&mut parsed.iter())?;
    tree.pretty();

    println!(
        "Total size of dir with size at most 100000 is {}",
        tree.less_than_10000()
    );

    Ok(())
}

#[test]
fn test_data() {
    let data = fs::read_to_string("test").expect("cannot read test file");

    let parsed = data
        .lines()
        .filter_map(|l| l.try_into().ok())
        .collect::<Vec<InputLine>>();
    let tree = build_tree(&mut parsed.iter()).expect("cannot build tree");

    assert_eq!(tree.less_than_10000(), 95437);
}

#[test]
fn visit_same_dir_twice() {
    let data = fs::read_to_string("test_double").expect("cannot read test file");

    let parsed = data
        .lines()
        .filter_map(|l| l.try_into().ok())
        .collect::<Vec<InputLine>>();
    let tree = build_tree(&mut parsed.iter()).expect("cannot build tree");
    assert_eq!(tree.less_than_10000(), 95437);
}
