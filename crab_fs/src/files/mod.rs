use std::{path::PathBuf, sync::RwLock};

mod writer;

/// So far it's a tree.
/// Each node is a directory or a file.
/// Directory nodes contain pointers to all content of given directory on disk.
///
/// What can we do?
/// How to handle bigger directories?
/// Do we need to store any values in the nodes - maybe just pointers?
// TODO: Add serialization to file.
pub struct Files {
    base: PathBuf,
    root: FilesNode,
}

#[derive(Debug)]
enum FilesNode {
    // Directory is basically a internal node
    Directory(DirectoryNode),
    // File is bascially a leaf node
    File(FileNode),
}

impl FilesNode {
    fn is_directory(&self) -> bool {
        match self {
            FilesNode::Directory(_) => true,
            FilesNode::File(_) => false,
        }
    }
}

#[derive(Debug)]
struct DirectoryNode {
    keys: Keys,
    values: Vec<FilesNode>,
}

impl DirectoryNode {
    fn empty() -> Self {
        DirectoryNode {
            keys: Keys::empty(),
            values: Vec::new(),
        }
    }

    fn put(&mut self, i: usize, path: &str, chunks: usize) -> &mut FilesNode {
        self.values.push(FilesNode::File(FileNode::new(chunks)));

        let key = NodeKey {
            value: path.to_string(),
            ptr: self.values.len() - 1,
        };
        self.keys.insert(i, key);

        // We can unwrap, as we just inserted the value
        self.values.last_mut().unwrap()
    }

    fn insert_dir(&mut self, i: usize, path: &str) -> &mut FilesNode {
        self.values
            .push(FilesNode::Directory(DirectoryNode::empty()));

        let key = NodeKey {
            value: path.to_string(),
            ptr: self.values.len() - 1,
        };
        self.keys.insert(i, key);

        // We can unwrap, as we just inserted the value
        self.values.last_mut().unwrap()
    }

    fn get_value(&self, index: isize) -> &FilesNode {
        &self.values[index as usize]
    }

    fn get_value_mut(&mut self, index: isize) -> &mut FilesNode {
        // Handle unwrap
        self.values.get_mut(index as usize).unwrap()
    }
}

/// Key should be PartialEq
#[derive(Debug)]
struct NodeKey {
    value: String,
    ptr: usize,
}

#[derive(Debug)]
struct FileNode {
    /// How many chunks this file was splitted into
    chunks: usize,

    /// TODO: Change it to something lightweight...
    lock: RwLock<()>,
}

impl FileNode {
    pub fn new(chunks: usize) -> FileNode {
        Self {
            chunks,
            lock: RwLock::new(()),
        }
    }
}

/// TODO: Add locking, before and after creation!
impl Files {
    pub fn new(path: PathBuf) -> Files {
        Self {
            base: path,
            root: FilesNode::Directory(DirectoryNode::empty()),
        }
    }

    pub fn contains_file(&self, path: PathBuf) -> bool {
        let p = path
            .strip_prefix(&self.base)
            .expect("Path should be inside the base path");
        let components: Vec<&str> = p.iter().map(|it| it.to_str().unwrap()).collect();
        Self::contains_file_inner(&self.root, &components, 0)
    }

    pub fn put(&mut self, path: PathBuf, chunks: usize) {
        // TODO: Fix unwrap here?
        let p = path
            .strip_prefix(&self.base)
            .expect("Path should be inside the base path");
        let components: Vec<&str> = p.iter().map(|it| it.to_str().unwrap()).collect();
        Self::find_node_or_create_node(&mut self.root, &components, chunks, 0);
    }

    // TODO: Just make path a stack?
    fn find_node_or_create_node(node: &mut FilesNode, path: &[&str], chunks: usize, i: usize) {
        let name = path[i];

        match node {
            FilesNode::Directory(node) => {
                let index = node.keys.binary_search(name);
                if index < 0 {
                    if i == path.len() - 1 {
                        // We are at the leaf node. Inserting and returning.
                        node.put(-index as usize - 1, name, chunks);
                        return;
                    } else {
                        let node = node.insert_dir(-index as usize - 1, name);
                        return Self::find_node_or_create_node(node, path, chunks, i + 1);
                    };
                }
                Self::find_node_or_create_node(node.get_value_mut(index), path, chunks, i + 1)
            }
            FilesNode::File(node) => {
                node.chunks += chunks;
            }
        }
    }

    fn contains_file_inner(node: &FilesNode, path: &[&str], i: usize) -> bool {
        match node {
            FilesNode::Directory(node) => {
                let name = path[i];
                let index = node.keys.binary_search(name);
                if index < 0 {
                    return false;
                }
                Self::contains_file_inner(node.get_value(index), path, i + 1)
            }

            // We found the leaf node
            FilesNode::File(_) => true,
        }
    }

    fn find_node_inner<'a>(node: &'a FilesNode, path: &[&str], i: usize) -> Option<&'a FilesNode> {
        if i == path.len() {
            return Some(node);
        }

        match node {
            FilesNode::Directory(node) => {
                let name = path[i];
                let index = node.keys.binary_search(name);
                if index < 0 {
                    return None;
                }
                Self::find_node_inner(node.get_value(index), path, i + 1)
            }

            // We found the leaf node
            FilesNode::File(_) => Some(node),
        }
    }

    fn ls(&self, path: PathBuf) -> Option<Vec<DirEntry>> {
        let p = path
            .strip_prefix(&self.base)
            .expect("Path should be inside the base path");
        let components: Vec<&str> = p.iter().map(|it| it.to_str().unwrap()).collect();

        let node = Self::find_node_inner(&self.root, &components, 0)?;

        match node {
            FilesNode::Directory(directory_node) => {
                let mut vec = Vec::new();
                for key in directory_node.keys.keys.iter() {
                    // If pointer exists then value MUST be inside!
                    let value = directory_node
                        .values
                        .get(key.ptr)
                        .expect("Value is missing. Critical error!");
                    vec.push(DirEntry {
                        name: key.value.clone(),
                        is_dir: value.is_directory(),
                    });
                }
                Some(vec)
            }
            // It's a file!
            FilesNode::File(_) => None,
        }
    }
}

#[derive(Debug)]
struct DirEntry {
    name: String,
    is_dir: bool,
}

#[derive(Debug)]
struct Keys {
    keys: Vec<NodeKey>,
}

impl Keys {
    fn empty() -> Self {
        Self { keys: Vec::new() }
    }

    fn binary_search(&self, value: &str) -> isize {
        if self.keys.is_empty() {
            return -1;
        }

        let mut lo = 0;
        let mut hi = self.keys.len() as isize - 1;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;

            match self.keys[mid as usize].value.as_str().cmp(value) {
                std::cmp::Ordering::Less => lo = mid + 1,
                std::cmp::Ordering::Equal => return mid,
                std::cmp::Ordering::Greater => hi = mid - 1,
            }
        }

        -(lo + 1)
    }

    // TODO: Return result on overflow
    fn insert(&mut self, i: usize, key: NodeKey) {
        self.keys.insert(i, key);
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::Files;

    #[test]
    fn path_buf_stack() {
        let path = PathBuf::from("/some/new/fancy/path/file.txt");

        let components: Vec<&str> = path.iter().map(|it| it.to_str().unwrap()).collect();

        assert_eq!(components.len(), 6);
        assert_eq!("/", components[0]);
        assert_eq!("some", components[1]);
        assert_eq!("new", components[2]);
        assert_eq!("fancy", components[3]);
        assert_eq!("path", components[4]);
        assert_eq!("file.txt", components[5]);
    }

    #[test]
    fn create_files_tree() {
        let base = PathBuf::from("/home/");
        let mut files = Files::new(base.join("crab/"));

        files.put(base.join("crab").join("file1.txt"), 14);

        assert!(files.contains_file(PathBuf::from("/home/crab/file1.txt")));
    }

    #[test]
    fn create_file_with_directory() {
        let base = PathBuf::from("/home/");
        let mut files = Files::new(base.join("crab/"));

        files.put(base.join("crab").join("dir").join("file1.txt"), 14);

        assert!(files.contains_file(PathBuf::from("/home/crab/dir/file1.txt")));
    }

    #[test]
    fn create_multiple_files_with_directory() {
        let mut files = Files::new("/home".into());

        files.put("/home/dir/file1.txt".into(), 14);
        assert!(files.contains_file(PathBuf::from("/home/dir/file1.txt")));

        files.put("/home/dir/file2.txt".into(), 10);
        assert!(files.contains_file(PathBuf::from("/home/dir/file2.txt")));
    }

    #[test]
    fn ls() {
        let mut files = Files::new("/home".into());

        files.put("/home/dir/file1.txt".into(), 14);
        assert!(files.contains_file(PathBuf::from("/home/dir/file1.txt")));

        files.put("/home/dir/file2.txt".into(), 10);
        assert!(files.contains_file(PathBuf::from("/home/dir/file2.txt")));

        files.put("/home/dir/dir2/file3.txt".into(), 8);
        assert!(files.contains_file(PathBuf::from("/home/dir/dir2/file2.txt")));

        assert!(files.ls("/home/dir".into()).unwrap().len() == 3);
    }
}
