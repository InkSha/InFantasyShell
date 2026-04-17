use std::collections::{BTreeMap, HashMap};
use std::fmt;

pub type NodeId = u64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NodeType {
    File,
    Directory,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PermBits {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl PermBits {
    pub const fn new(read: bool, write: bool, execute: bool) -> Self {
        Self {
            read,
            write,
            execute,
        }
    }

    fn to_octal_digit(&self) -> u8 {
        (u8::from(self.read) * 4) + (u8::from(self.write) * 2) + u8::from(self.execute)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Permissions {
    pub owner: PermBits,
    pub group: PermBits,
    pub other: PermBits,
}

impl Permissions {
    pub const fn file_default() -> Self {
        Self {
            owner: PermBits::new(true, true, false),
            group: PermBits::new(true, false, false),
            other: PermBits::new(true, false, false),
        }
    }

    pub const fn directory_default() -> Self {
        Self {
            owner: PermBits::new(true, true, true),
            group: PermBits::new(true, false, true),
            other: PermBits::new(true, false, true),
        }
    }

    pub const fn executable_file() -> Self {
        Self {
            owner: PermBits::new(true, true, true),
            group: PermBits::new(true, false, true),
            other: PermBits::new(true, false, true),
        }
    }

    pub fn from_octal(input: &str) -> Result<Self, FsError> {
        if input.len() != 3 || !input.chars().all(|ch| ('0'..='7').contains(&ch)) {
            return Err(FsError::InvalidPermissions(input.to_string()));
        }

        let mut digits = input.chars();
        let owner = digits
            .next()
            .and_then(|ch| ch.to_digit(8))
            .ok_or_else(|| FsError::InvalidPermissions(input.to_string()))? as u8;
        let group = digits
            .next()
            .and_then(|ch| ch.to_digit(8))
            .ok_or_else(|| FsError::InvalidPermissions(input.to_string()))? as u8;
        let other = digits
            .next()
            .and_then(|ch| ch.to_digit(8))
            .ok_or_else(|| FsError::InvalidPermissions(input.to_string()))? as u8;

        Ok(Self {
            owner: Self::bits_from_digit(owner),
            group: Self::bits_from_digit(group),
            other: Self::bits_from_digit(other),
        })
    }

    fn bits_from_digit(digit: u8) -> PermBits {
        PermBits::new(digit & 4 != 0, digit & 2 != 0, digit & 1 != 0)
    }

    pub fn to_octal_string(&self) -> String {
        format!(
            "{}{}{}",
            self.owner.to_octal_digit(),
            self.group.to_octal_digit(),
            self.other.to_octal_digit()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileData {
    pub content: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectoryData {
    pub children: BTreeMap<String, NodeId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NodeKind {
    File(FileData),
    Directory(DirectoryData),
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FsError {
    NotFound(String),
    AlreadyExists(String),
    NotDirectory(String),
    NotFile(String),
    PermissionDenied { path: String, action: &'static str },
    DirectoryNotEmpty(String),
    InvalidPath(String),
    InvalidPermissions(String),
    CannotRemoveRoot,
}

impl fmt::Display for FsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(path) => write!(f, "path not found: {path}"),
            Self::AlreadyExists(path) => write!(f, "path already exists: {path}"),
            Self::NotDirectory(path) => write!(f, "not a directory: {path}"),
            Self::NotFile(path) => write!(f, "not a file: {path}"),
            Self::PermissionDenied { path, action } => {
                write!(f, "permission denied for {action} on {path}")
            }
            Self::DirectoryNotEmpty(path) => write!(f, "directory is not empty: {path}"),
            Self::InvalidPath(path) => write!(f, "invalid path: {path}"),
            Self::InvalidPermissions(input) => write!(f, "invalid chmod mode: {input}"),
            Self::CannotRemoveRoot => write!(f, "cannot remove root directory"),
        }
    }
}

impl std::error::Error for FsError {}

#[derive(Debug)]
pub struct Vfs {
    root: NodeId,
    next_id: NodeId,
    nodes: HashMap<NodeId, Node>,
}

impl Vfs {
    pub fn new() -> Self {
        let root_id = 1;
        let root = Node {
            id: root_id,
            name: String::new(),
            parent: None,
            owner: "root".to_string(),
            permissions: Permissions::directory_default(),
            kind: NodeKind::Directory(DirectoryData {
                children: BTreeMap::new(),
            }),
        };

        let mut nodes = HashMap::new();
        nodes.insert(root_id, root);

        Self {
            root: root_id,
            next_id: root_id + 1,
            nodes,
        }
    }

    pub fn new_mvp_world() -> Result<Self, FsError> {
        let mut vfs = Self::new();

        vfs.create_dir_absolute("/home", "root", Permissions::directory_default())?;
        vfs.create_dir_absolute("/home/player", "player", Permissions::directory_default())?;
        vfs.create_file_absolute(
            "/home/player/readme.txt",
            "player",
            "Welcome to InFantasyShell.\nExplore the world through the virtual filesystem."
                .to_string(),
            Permissions::file_default(),
        )?;
        vfs.create_file_absolute(
            "/home/player/notes.txt",
            "player",
            "Bits are your carrying capacity.\n".to_string(),
            Permissions::file_default(),
        )?;
        vfs.create_dir_absolute("/player", "root", Permissions::directory_default())?;
        vfs.create_dir_absolute("/player/memory", "player", Permissions::directory_default())?;
        vfs.create_dir_absolute("/monster", "root", Permissions::directory_default())?;
        vfs.create_dir_absolute("/monster/slime", "root", Permissions::directory_default())?;
        vfs.create_file_absolute(
            "/monster/slime/hp",
            "root",
            "12\n".to_string(),
            Permissions::file_default(),
        )?;
        vfs.create_file_absolute(
            "/monster/slime/ai.sh",
            "root",
            "echo slime attacks\n".to_string(),
            Permissions::executable_file(),
        )?;
        vfs.create_dir_absolute("/etc", "root", Permissions::directory_default())?;

        Ok(vfs)
    }

    pub fn root_id(&self) -> NodeId {
        self.root
    }

    pub fn stat(&self, cwd: NodeId, path: &str, actor: &str) -> Result<Stat, FsError> {
        let node_id = self.resolve_node(cwd, path, actor)?;
        self.stat_by_id(node_id)
    }

    pub fn stat_by_id(&self, node_id: NodeId) -> Result<Stat, FsError> {
        let node = self.node(node_id)?;

        Ok(Stat {
            name: node.display_name(),
            path: self.absolute_path(node_id)?,
            node_type: node.node_type(),
            size: self.node_size(node_id)?,
            owner: node.owner.clone(),
            permissions: node.permissions.to_octal_string(),
        })
    }

    pub fn absolute_path(&self, node_id: NodeId) -> Result<String, FsError> {
        let mut parts = Vec::new();
        let mut current = Some(node_id);

        while let Some(id) = current {
            let node = self.node(id)?;
            if node.parent.is_none() {
                break;
            }

            parts.push(node.name.clone());
            current = node.parent;
        }

        parts.reverse();

        if parts.is_empty() {
            Ok("/".to_string())
        } else {
            Ok(format!("/{}", parts.join("/")))
        }
    }

    pub fn list_dir(&self, cwd: NodeId, path: &str, actor: &str) -> Result<Vec<DirEntry>, FsError> {
        let dir_id = self.resolve_node(cwd, path, actor)?;
        let path_string = self.absolute_path(dir_id)?;
        self.require_directory_permission(dir_id, actor, path_string.as_str(), DirectoryAction::List)?;

        let node = self.node(dir_id)?;
        let children = match &node.kind {
            NodeKind::Directory(directory) => directory.children.values().copied().collect::<Vec<_>>(),
            NodeKind::File(_) => return Err(FsError::NotDirectory(path_string)),
        };

        children
            .into_iter()
            .map(|child_id| {
                let child = self.node(child_id)?;
                Ok(DirEntry {
                    name: child.display_name(),
                    node_type: child.node_type(),
                    size: self.node_size(child_id)?,
                    permissions: child.permissions.to_octal_string(),
                })
            })
            .collect()
    }

    pub fn read_file(&self, cwd: NodeId, path: &str, actor: &str) -> Result<String, FsError> {
        let node_id = self.resolve_node(cwd, path, actor)?;
        let path_string = self.absolute_path(node_id)?;
        self.require_file_permission(node_id, actor, path_string.as_str(), FileAction::Read)?;

        match &self.node(node_id)?.kind {
            NodeKind::File(file) => Ok(file.content.clone()),
            NodeKind::Directory(_) => Err(FsError::NotFile(path_string)),
        }
    }

    pub fn write_file(
        &mut self,
        cwd: NodeId,
        path: &str,
        actor: &str,
        content: String,
    ) -> Result<(), FsError> {
        let (parent_id, name) = self.resolve_parent(cwd, path, actor)?;
        let parent_path = self.absolute_path(parent_id)?;
        self.require_directory_permission(parent_id, actor, parent_path.as_str(), DirectoryAction::Write)?;

        let existing_id = self.lookup_child(parent_id, &name)?;
        match existing_id {
            Some(node_id) => {
                let existing_path = self.absolute_path(node_id)?;
                self.require_file_permission(node_id, actor, existing_path.as_str(), FileAction::Write)?;

                let node = self.node_mut(node_id)?;
                match &mut node.kind {
                    NodeKind::File(file) => {
                        file.content = content;
                        Ok(())
                    }
                    NodeKind::Directory(_) => Err(FsError::NotFile(existing_path)),
                }
            }
            None => {
                self.create_file_under(parent_id, &name, actor, content, Permissions::file_default())?;
                Ok(())
            }
        }
    }

    pub fn create_dir(&mut self, cwd: NodeId, path: &str, actor: &str) -> Result<(), FsError> {
        let (parent_id, name) = self.resolve_parent(cwd, path, actor)?;
        let parent_path = self.absolute_path(parent_id)?;
        self.require_directory_permission(parent_id, actor, parent_path.as_str(), DirectoryAction::Write)?;
        self.create_directory_under(parent_id, &name, actor, Permissions::directory_default())?;
        Ok(())
    }

    pub fn remove(&mut self, cwd: NodeId, path: &str, actor: &str) -> Result<(), FsError> {
        let node_id = self.resolve_node(cwd, path, actor)?;
        if node_id == self.root {
            return Err(FsError::CannotRemoveRoot);
        }

        let node_path = self.absolute_path(node_id)?;
        let parent_id = self
            .node(node_id)?
            .parent
            .ok_or(FsError::CannotRemoveRoot)?;
        let parent_path = self.absolute_path(parent_id)?;
        self.require_directory_permission(parent_id, actor, parent_path.as_str(), DirectoryAction::Write)?;

        if let NodeKind::Directory(directory) = &self.node(node_id)?.kind {
            if !directory.children.is_empty() {
                return Err(FsError::DirectoryNotEmpty(node_path));
            }
        }

        let node_name = self.node(node_id)?.name.clone();
        if let NodeKind::Directory(parent_directory) = &mut self.node_mut(parent_id)?.kind {
            parent_directory.children.remove(&node_name);
        }
        self.nodes.remove(&node_id);

        Ok(())
    }

    pub fn chmod(
        &mut self,
        cwd: NodeId,
        path: &str,
        actor: &str,
        permissions: Permissions,
    ) -> Result<(), FsError> {
        let node_id = self.resolve_node(cwd, path, actor)?;
        let path_string = self.absolute_path(node_id)?;

        if self.node(node_id)?.owner != actor {
            return Err(FsError::PermissionDenied {
                path: path_string,
                action: "chmod",
            });
        }

        self.node_mut(node_id)?.permissions = permissions;
        Ok(())
    }

    pub fn change_dir(&self, cwd: NodeId, path: &str, actor: &str) -> Result<NodeId, FsError> {
        let node_id = self.resolve_node(cwd, path, actor)?;
        let path_string = self.absolute_path(node_id)?;
        self.require_directory_permission(node_id, actor, path_string.as_str(), DirectoryAction::Traverse)?;
        Ok(node_id)
    }

    pub fn resolve_node(&self, cwd: NodeId, path: &str, actor: &str) -> Result<NodeId, FsError> {
        self.resolve_node_internal(cwd, path, Some(actor))
    }

    pub fn resolve_node_unchecked(&self, cwd: NodeId, path: &str) -> Result<NodeId, FsError> {
        self.resolve_node_internal(cwd, path, None)
    }

    fn resolve_node_internal(
        &self,
        cwd: NodeId,
        path: &str,
        actor: Option<&str>,
    ) -> Result<NodeId, FsError> {
        let trimmed = path.trim();
        if trimmed.is_empty() {
            return Err(FsError::InvalidPath(path.to_string()));
        }

        if trimmed == "/" {
            return Ok(self.root);
        }

        let mut current = if trimmed.starts_with('/') { self.root } else { cwd };
        let segments = trimmed
            .split('/')
            .filter(|segment| !segment.is_empty())
            .collect::<Vec<_>>();

        for segment in segments {
            match segment {
                "." => {}
                ".." => {
                    current = self.node(current)?.parent.unwrap_or(self.root);
                }
                name => {
                    let current_path = self.absolute_path(current)?;
                    if let Some(actor_name) = actor {
                        self.require_directory_permission(
                            current,
                            actor_name,
                            current_path.as_str(),
                            DirectoryAction::Traverse,
                        )?;
                    }

                    let child_id = self
                        .lookup_child(current, name)?
                        .ok_or_else(|| FsError::NotFound(trimmed.to_string()))?;
                    current = child_id;
                }
            }
        }

        Ok(current)
    }

    fn resolve_parent(
        &self,
        cwd: NodeId,
        path: &str,
        actor: &str,
    ) -> Result<(NodeId, String), FsError> {
        let trimmed = path.trim();
        if trimmed.is_empty() || trimmed == "/" {
            return Err(FsError::InvalidPath(path.to_string()));
        }

        let mut parts = trimmed
            .split('/')
            .filter(|segment| !segment.is_empty())
            .collect::<Vec<_>>();

        let name = parts
            .pop()
            .ok_or_else(|| FsError::InvalidPath(path.to_string()))?;

        if matches!(name, "." | "..") {
            return Err(FsError::InvalidPath(path.to_string()));
        }

        let parent_path = if trimmed.starts_with('/') {
            if parts.is_empty() {
                "/".to_string()
            } else {
                format!("/{}", parts.join("/"))
            }
        } else if parts.is_empty() {
            ".".to_string()
        } else {
            parts.join("/")
        };

        let parent_id = self.resolve_node(cwd, parent_path.as_str(), actor)?;
        Ok((parent_id, name.to_string()))
    }

    fn create_dir_absolute(
        &mut self,
        path: &str,
        owner: &str,
        permissions: Permissions,
    ) -> Result<NodeId, FsError> {
        let (parent_id, name) = self.resolve_parent_unchecked(path)?;
        self.create_directory_under(parent_id, &name, owner, permissions)
    }

    fn create_file_absolute(
        &mut self,
        path: &str,
        owner: &str,
        content: String,
        permissions: Permissions,
    ) -> Result<NodeId, FsError> {
        let (parent_id, name) = self.resolve_parent_unchecked(path)?;
        self.create_file_under(parent_id, &name, owner, content, permissions)
    }

    fn resolve_parent_unchecked(&self, path: &str) -> Result<(NodeId, String), FsError> {
        let trimmed = path.trim();
        if trimmed.is_empty() || !trimmed.starts_with('/') || trimmed == "/" {
            return Err(FsError::InvalidPath(path.to_string()));
        }

        let mut parts = trimmed
            .split('/')
            .filter(|segment| !segment.is_empty())
            .collect::<Vec<_>>();
        let name = parts
            .pop()
            .ok_or_else(|| FsError::InvalidPath(path.to_string()))?;

        let parent = if parts.is_empty() {
            self.root
        } else {
            self.resolve_node_unchecked(self.root, format!("/{}", parts.join("/")).as_str())?
        };

        Ok((parent, name.to_string()))
    }

    fn create_directory_under(
        &mut self,
        parent_id: NodeId,
        name: &str,
        owner: &str,
        permissions: Permissions,
    ) -> Result<NodeId, FsError> {
        if self.lookup_child(parent_id, name)?.is_some() {
            return Err(FsError::AlreadyExists(name.to_string()));
        }

        let new_id = self.next_node_id();
        let node = Node {
            id: new_id,
            name: name.to_string(),
            parent: Some(parent_id),
            owner: owner.to_string(),
            permissions,
            kind: NodeKind::Directory(DirectoryData {
                children: BTreeMap::new(),
            }),
        };

        self.insert_child(parent_id, name, new_id)?;
        self.nodes.insert(new_id, node);
        Ok(new_id)
    }

    fn create_file_under(
        &mut self,
        parent_id: NodeId,
        name: &str,
        owner: &str,
        content: String,
        permissions: Permissions,
    ) -> Result<NodeId, FsError> {
        if self.lookup_child(parent_id, name)?.is_some() {
            return Err(FsError::AlreadyExists(name.to_string()));
        }

        let new_id = self.next_node_id();
        let node = Node {
            id: new_id,
            name: name.to_string(),
            parent: Some(parent_id),
            owner: owner.to_string(),
            permissions,
            kind: NodeKind::File(FileData { content }),
        };

        self.insert_child(parent_id, name, new_id)?;
        self.nodes.insert(new_id, node);
        Ok(new_id)
    }

    fn insert_child(&mut self, parent_id: NodeId, name: &str, child_id: NodeId) -> Result<(), FsError> {
        let parent_path = self.absolute_path(parent_id)?;
        match &mut self.node_mut(parent_id)?.kind {
            NodeKind::Directory(directory) => {
                directory.children.insert(name.to_string(), child_id);
                Ok(())
            }
            NodeKind::File(_) => Err(FsError::NotDirectory(parent_path)),
        }
    }

    fn lookup_child(&self, parent_id: NodeId, name: &str) -> Result<Option<NodeId>, FsError> {
        let parent_path = self.absolute_path(parent_id)?;
        match &self.node(parent_id)?.kind {
            NodeKind::Directory(directory) => Ok(directory.children.get(name).copied()),
            NodeKind::File(_) => Err(FsError::NotDirectory(parent_path)),
        }
    }

    fn require_directory_permission(
        &self,
        node_id: NodeId,
        actor: &str,
        path: &str,
        action: DirectoryAction,
    ) -> Result<(), FsError> {
        let node = self.node(node_id)?;
        let permissions = self.permission_bits(node, actor);

        let allowed = match (&node.kind, action) {
            (NodeKind::Directory(_), DirectoryAction::Traverse) => permissions.execute,
            (NodeKind::Directory(_), DirectoryAction::List) => permissions.read && permissions.execute,
            (NodeKind::Directory(_), DirectoryAction::Write) => permissions.write && permissions.execute,
            (NodeKind::File(_), _) => return Err(FsError::NotDirectory(path.to_string())),
        };

        if allowed {
            Ok(())
        } else {
            Err(FsError::PermissionDenied {
                path: path.to_string(),
                action: action.label(),
            })
        }
    }

    fn require_file_permission(
        &self,
        node_id: NodeId,
        actor: &str,
        path: &str,
        action: FileAction,
    ) -> Result<(), FsError> {
        let node = self.node(node_id)?;
        let permissions = self.permission_bits(node, actor);

        let allowed = match (&node.kind, action) {
            (NodeKind::File(_), FileAction::Read) => permissions.read,
            (NodeKind::File(_), FileAction::Write) => permissions.write,
            (NodeKind::Directory(_), _) => return Err(FsError::NotFile(path.to_string())),
        };

        if allowed {
            Ok(())
        } else {
            Err(FsError::PermissionDenied {
                path: path.to_string(),
                action: action.label(),
            })
        }
    }

    fn permission_bits<'a>(&self, node: &'a Node, actor: &str) -> &'a PermBits {
        if node.owner == actor {
            &node.permissions.owner
        } else {
            &node.permissions.other
        }
    }

    fn node_size(&self, node_id: NodeId) -> Result<usize, FsError> {
        match &self.node(node_id)?.kind {
            NodeKind::File(file) => Ok(file.content.len()),
            NodeKind::Directory(directory) => directory
                .children
                .values()
                .copied()
                .map(|child_id| self.node_size(child_id))
                .sum(),
        }
    }

    fn next_node_id(&mut self) -> NodeId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn node(&self, node_id: NodeId) -> Result<&Node, FsError> {
        self.nodes
            .get(&node_id)
            .ok_or_else(|| FsError::NotFound(format!("node:{node_id}")))
    }

    fn node_mut(&mut self, node_id: NodeId) -> Result<&mut Node, FsError> {
        self.nodes
            .get_mut(&node_id)
            .ok_or_else(|| FsError::NotFound(format!("node:{node_id}")))
    }
}

impl Default for Vfs {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy)]
enum DirectoryAction {
    Traverse,
    List,
    Write,
}

impl DirectoryAction {
    fn label(self) -> &'static str {
        match self {
            Self::Traverse => "traverse",
            Self::List => "list",
            Self::Write => "write",
        }
    }
}

#[derive(Clone, Copy)]
enum FileAction {
    Read,
    Write,
}

impl FileAction {
    fn label(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Write => "write",
        }
    }
}

impl Node {
    fn display_name(&self) -> String {
        if self.parent.is_none() {
            "/".to_string()
        } else {
            self.name.clone()
        }
    }

    fn node_type(&self) -> NodeType {
        match self.kind {
            NodeKind::File(_) => NodeType::File,
            NodeKind::Directory(_) => NodeType::Directory,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FsError, Permissions, Vfs};

    fn seeded_vfs() -> Vfs {
        Vfs::new_mvp_world().expect("seeded VFS should build")
    }

    #[test]
    fn resolves_absolute_and_relative_paths() {
        let vfs = seeded_vfs();
        let home = vfs
            .resolve_node(vfs.root_id(), "/home/player", "player")
            .expect("home path should resolve");
        let notes = vfs
            .resolve_node(home, "./notes.txt", "player")
            .expect("relative file should resolve");

        assert_eq!(
            vfs.absolute_path(notes).expect("path should be printable"),
            "/home/player/notes.txt"
        );
        assert_eq!(
            vfs.resolve_node(home, "../player/readme.txt", "player")
                .expect("dot-dot path should resolve"),
            vfs.resolve_node(vfs.root_id(), "/home/player/readme.txt", "player")
                .expect("absolute path should resolve")
        );
    }

    #[test]
    fn reads_and_writes_files() {
        let mut vfs = seeded_vfs();
        let home = vfs
            .resolve_node(vfs.root_id(), "/home/player", "player")
            .expect("home path should resolve");

        vfs.write_file(home, "journal.txt", "player", "entry".to_string())
            .expect("player should create a file");

        assert_eq!(
            vfs.read_file(home, "journal.txt", "player")
                .expect("player should read created file"),
            "entry"
        );
    }

    #[test]
    fn enforces_permissions_for_other_users() {
        let mut vfs = seeded_vfs();
        let monster = vfs
            .resolve_node(vfs.root_id(), "/monster/slime", "root")
            .expect("monster directory should resolve");

        vfs.chmod(monster, "hp", "root", Permissions::from_octal("600").expect("mode should parse"))
            .expect("root should be able to chmod");

        let error = vfs
            .read_file(vfs.root_id(), "/monster/slime/hp", "player")
            .expect_err("player should be blocked");

        assert!(matches!(
            error,
            FsError::PermissionDenied {
                action: "read",
                ..
            }
        ));
    }

    #[test]
    fn reports_recursive_sizes() {
        let vfs = seeded_vfs();
        let home_stat = vfs
            .stat(vfs.root_id(), "/home/player", "player")
            .expect("home stat should work");
        let readme_size = vfs
            .read_file(vfs.root_id(), "/home/player/readme.txt", "player")
            .expect("readme should be readable")
            .len();
        let notes_size = vfs
            .read_file(vfs.root_id(), "/home/player/notes.txt", "player")
            .expect("notes should be readable")
            .len();

        assert_eq!(home_stat.size, readme_size + notes_size);
    }

    #[test]
    fn rejects_non_empty_directory_removal() {
        let mut vfs = seeded_vfs();
        let error = vfs
            .remove(vfs.root_id(), "/home/player", "player")
            .expect_err("non-empty directory removal should fail");

        assert!(matches!(error, FsError::DirectoryNotEmpty(_)));
    }
}
