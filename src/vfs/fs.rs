use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type FSNodeRef = Rc<RefCell<Node>>;

#[derive(Debug)]
pub enum NodeType {
    FILE,
    DIRECTORY,
}

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub node_type: NodeType,
    pub content: Option<String>,
    pub parent: Option<FSNodeRef>,
    pub children: HashMap<String, FSNodeRef>,
}

impl Node {
    pub fn new_directory(name: String) -> Self {
        Node {
            name: name,
            node_type: NodeType::DIRECTORY,
            content: None,
            children: HashMap::new(),
            parent: None,
        }
    }

    pub fn new_file(name: String, content: String) -> Self {
        Node {
            name: name,
            node_type: NodeType::FILE,
            content: Some(content),
            children: HashMap::new(),
            parent: None,
        }
    }
}
