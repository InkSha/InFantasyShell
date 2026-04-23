use crate::system::{System, error::FsError, storage::node::NodeId};

pub struct ShellState {
    pub system: System,
    pub cwd: NodeId,
    pub home: NodeId,
    pub actor: String,
}

impl ShellState {
    pub fn new(actor: &str) -> Result<Self, FsError> {
        let mut system = System::default();

        system.storage.new_mvp_world()?;

        let cwd = system
            .storage
            .resolve_node_unchecked(system.storage.root_id(), "/home/player")?;

        Ok(Self {
            system,
            cwd,
            home: cwd,
            actor: actor.to_string(),
        })
    }

    pub fn render_prompt(&self, shell_name: &str) -> String {
        let cwd_path = self
            .system
            .storage
            .absolute_path(self.cwd)
            .unwrap_or_else(|_| "/".to_string());
        format!("{}@{}:{}$ ", self.actor, shell_name, cwd_path)
    }
}
