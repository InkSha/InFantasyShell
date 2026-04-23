use crate::system::{
    bit::DataSize,
    error::FsError,
    permission::{PermBits, Permissions},
};
use node::{
    DirEntry, DirectoryAction, DirectoryData, FileAction, FileData, Node, NodeId, NodeKind, Stat,
};
use std::collections::{BTreeMap, HashMap};

mod dir;
mod file;
pub mod node;

pub struct Storage {
    root: NodeId,
    next_id: NodeId,
    nodes: HashMap<NodeId, Node>,

    pub total: DataSize,
    pub used: DataSize,
}

impl Storage {
    pub fn default() -> Self {
        let total = DataSize::Gigabyte(50);
        let used = DataSize::Gigabyte(10);

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
            total,
            used,
            root: root_id,
            next_id: root_id + 1,
            nodes,
        }
    }

    pub fn new_mvp_world(&mut self) -> Result<bool, FsError> {
        self.create_dir_absolute("/home", "root", Permissions::directory_default())?;
        self.create_dir_absolute("/home/player", "player", Permissions::directory_default())?;
        self.create_file_absolute(
            "/home/player/readme.txt",
            "player",
            "Welcome to InFantasyShell.\nExplore the world through the virtual filesystem."
                .to_string(),
            Permissions::file_default(),
        )?;
        self.create_file_absolute(
            "/home/player/aaaaaa",
            "player",
            "Welcome to InFantasyShell.\nExplore the world through the virtual filesystem."
                .to_string(),
            Permissions::file_default(),
        )?;
        self.create_file_absolute(
            "/home/player/bbbbbb",
            "player",
            "Welcome to InFantasyShell.\nExplore the world through the virtual filesystem."
                .to_string(),
            Permissions::file_default(),
        )?;
        self.create_file_absolute(
            "/home/player/ffffff",
            "player",
            "Welcome to InFantasyShell.\nExplore the world through the virtual filesystem."
                .to_string(),
            Permissions::file_default(),
        )?;
        self.create_file_absolute(
            "/home/player/zzzzzz",
            "player",
            "Welcome to InFantasyShell.\nExplore the world through the virtual filesystem."
                .to_string(),
            Permissions::file_default(),
        )?;
        self.create_file_absolute(
            "/home/player/yyyyyy",
            "player",
            "Welcome to InFantasyShell.\nExplore the world through the virtual filesystem."
                .to_string(),
            Permissions::file_default(),
        )?;
        self.create_file_absolute(
            "/home/player/qqqqqqq",
            "player",
            "Welcome to InFantasyShell.\nExplore the world through the virtual filesystem."
                .to_string(),
            Permissions::file_default(),
        )?;
        self.create_file_absolute(
            "/home/player/notes.txt",
            "player",
            "Bits are your carrying capacity.\n".to_string(),
            Permissions::file_default(),
        )?;
        self.create_dir_absolute("/player", "root", Permissions::directory_default())?;
        self.create_dir_absolute("/player/memory", "player", Permissions::directory_default())?;
        self.create_dir_absolute("/monster", "root", Permissions::directory_default())?;
        self.create_dir_absolute("/monster/slime", "root", Permissions::directory_default())?;
        self.create_file_absolute(
            "/monster/slime/hp",
            "root",
            "12\n".to_string(),
            Permissions::file_default(),
        )?;
        self.create_file_absolute(
            "/monster/slime/ai.sh",
            "root",
            "echo slime attacks\n".to_string(),
            Permissions::executable_file(),
        )?;
        self.create_dir_absolute("/etc", "root", Permissions::directory_default())?;

        Ok(true)
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
        self.require_directory_permission(
            dir_id,
            actor,
            path_string.as_str(),
            DirectoryAction::List,
        )?;

        let node = self.node(dir_id)?;
        let children = match &node.kind {
            NodeKind::Directory(directory) => {
                directory.children.values().copied().collect::<Vec<_>>()
            }
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
        self.require_directory_permission(
            parent_id,
            actor,
            parent_path.as_str(),
            DirectoryAction::Write,
        )?;

        let existing_id = self.lookup_child(parent_id, &name)?;
        match existing_id {
            Some(node_id) => {
                let existing_path = self.absolute_path(node_id)?;
                self.require_file_permission(
                    node_id,
                    actor,
                    existing_path.as_str(),
                    FileAction::Write,
                )?;

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
                self.create_file_under(
                    parent_id,
                    &name,
                    actor,
                    content,
                    Permissions::file_default(),
                )?;
                Ok(())
            }
        }
    }

    pub fn create_dir(&mut self, cwd: NodeId, path: &str, actor: &str) -> Result<(), FsError> {
        let (parent_id, name) = self.resolve_parent(cwd, path, actor)?;
        let parent_path = self.absolute_path(parent_id)?;
        self.require_directory_permission(
            parent_id,
            actor,
            parent_path.as_str(),
            DirectoryAction::Write,
        )?;
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
        self.require_directory_permission(
            parent_id,
            actor,
            parent_path.as_str(),
            DirectoryAction::Write,
        )?;

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
        self.require_directory_permission(
            node_id,
            actor,
            path_string.as_str(),
            DirectoryAction::Traverse,
        )?;
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

        let mut current = if trimmed.starts_with('/') {
            self.root
        } else {
            cwd
        };
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

    fn insert_child(
        &mut self,
        parent_id: NodeId,
        name: &str,
        child_id: NodeId,
    ) -> Result<(), FsError> {
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
            (NodeKind::Directory(_), DirectoryAction::List) => {
                permissions.read && permissions.execute
            }
            (NodeKind::Directory(_), DirectoryAction::Write) => {
                permissions.write && permissions.execute
            }
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
