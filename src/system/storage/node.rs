use crate::system::permission::Permissions;
use std::collections::BTreeMap;

pub type NodeId = u64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NodeType {
    File,
    Directory,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectoryData {
    pub children: BTreeMap<String, NodeId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileData {
    pub content: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node {
    pub id: NodeId,
    pub name: String,
    pub parent: Option<NodeId>,
    pub owner: String,
    pub permissions: Permissions,
    pub kind: NodeKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NodeKind {
    File(FileData),
    Directory(DirectoryData),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirEntry {
    pub name: String,
    pub node_type: NodeType,
    pub size: usize,
    pub permissions: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Stat {
    pub name: String,
    pub path: String,
    pub node_type: NodeType,
    pub size: usize,
    pub owner: String,
    pub permissions: String,
}

#[derive(Clone, Copy)]
pub enum DirectoryAction {
    Traverse,
    List,
    Write,
}

impl DirectoryAction {
    pub fn label(self) -> &'static str {
        match self {
            Self::Traverse => "traverse",
            Self::List => "list",
            Self::Write => "write",
        }
    }
}

#[derive(Clone, Copy)]
pub enum FileAction {
    Read,
    Write,
}

impl FileAction {
    pub fn label(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Write => "write",
        }
    }
}

impl Node {
    pub fn display_name(&self) -> String {
        if self.parent.is_none() {
            "/".to_string()
        } else {
            self.name.clone()
        }
    }

    pub fn node_type(&self) -> NodeType {
        match self.kind {
            NodeKind::File(_) => NodeType::File,
            NodeKind::Directory(_) => NodeType::Directory,
        }
    }
}
