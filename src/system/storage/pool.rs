use crate::system::storage::node::{Node, NodeId};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::sync::{Arc, Weak};

static GLOBAL_POOL: Lazy<DashMap<NodeId, Weak<Node>>> = Lazy::new(|| DashMap::new());

pub fn insert(node_id: NodeId, node: Arc<Node>) {
    GLOBAL_POOL.insert(node_id, Arc::downgrade(&node));
}

pub fn get(node_id: &NodeId) -> Option<Arc<Node>> {
    GLOBAL_POOL.get(node_id).and_then(|weak| weak.upgrade())
}

pub fn remove(node_id: &NodeId) -> Option<Arc<Node>> {
    GLOBAL_POOL
        .remove(node_id)
        .and_then(|(_, weak)| weak.upgrade())
}

pub fn contains(node_id: &NodeId) -> bool {
    GLOBAL_POOL.contains_key(node_id)
}
