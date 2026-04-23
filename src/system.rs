pub mod bit;
pub mod cpu;
pub mod error;
pub mod memory;
pub mod network;
pub mod permission;
pub mod storage;

use crate::system::{error::FsError, storage::node::NodeId};

pub struct System {
    cwd: Option<NodeId>,
    pub home: Option<NodeId>,
    pub actor: Option<String>,

    pub cpu: cpu::CPU,
    pub memory: memory::Memory,
    pub storage: storage::Storage,
    pub network: network::Network,
}

impl System {
    pub fn default() -> Result<Self, FsError> {
        Self::new(
            cpu::CPU::default(),
            memory::Memory::defualt(),
            storage::Storage::default(),
            network::Network::default(),
        )
    }

    pub fn new(
        cpu: cpu::CPU,
        memory: memory::Memory,
        storage: storage::Storage,
        network: network::Network,
    ) -> Result<Self, FsError> {
        Ok(Self {
            cpu,
            memory,
            storage,
            network,
            cwd: None,
            home: None,
            actor: None,
        })
    }

    pub fn login(&mut self /*, actor: &str */) {
        let actor = "player";
        self.actor = Some(actor.to_string());

        match self.storage.new_mvp_world() {
            Ok(_) => {}
            Err(e) => {
                panic!("Failed to initialize storage: {}", e);
            }
        }

        match self
            .storage
            .resolve_node_unchecked(self.storage.root_id(), "/home/player")
        {
            Ok(home_id) => {
                self.home = Some(home_id);
                self.cwd = Some(home_id);
            }
            Err(_) => {
                self.home = None;
                self.cwd = None;
            }
        }
    }

    pub fn change_cwd(&mut self, new_cwd: NodeId) {
        self.cwd = Some(new_cwd);
    }

    pub fn get_cwd(&self) -> NodeId {
        match self.cwd {
            Some(cwd) => cwd,
            None => self.storage.root_id(),
        }
    }

    pub fn get_actor(&self) -> String {
        match &self.actor {
            Some(actor) => actor.clone(),
            None => String::from("root"),
        }
    }

    pub fn render_prompt<T: ToString>(&self, shell_name: T) -> String {
        let cwd_path = self
            .storage
            .absolute_path(self.get_cwd())
            .unwrap_or_else(|_| "/".to_string());
        format!(
            "{}@{}:{}$ ",
            self.get_actor(),
            shell_name.to_string(),
            cwd_path
        )
    }
}
