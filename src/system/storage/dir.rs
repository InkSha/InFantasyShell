use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectoryData {
    pub children: BTreeMap<String, NodeId>,
}
