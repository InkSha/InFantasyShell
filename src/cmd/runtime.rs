use crate::vfs::{FsError, NodeId, Vfs};

pub struct ShellState {
    pub vfs: Vfs,
    pub cwd: NodeId,
    pub home: NodeId,
    pub actor: String,
}

impl ShellState {
    pub fn new(actor: &str) -> Result<Self, FsError> {
        let vfs = Vfs::new_mvp_world()?;
        let cwd = vfs.resolve_node_unchecked(vfs.root_id(), "/home/player")?;

        Ok(Self {
            vfs,
            cwd,
            home: cwd,
            actor: actor.to_string(),
        })
    }

    pub fn render_prompt(&self, shell_name: &str) -> String {
        let cwd_path = self
            .vfs
            .absolute_path(self.cwd)
            .unwrap_or_else(|_| "/".to_string());
        format!("{}@{}:{}$ ", self.actor, shell_name, cwd_path)
    }
}
