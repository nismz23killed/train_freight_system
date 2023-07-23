use crate::util::minute::Minute;

use super::node::NodeId;

#[derive(Debug, PartialEq)]
pub struct EdgeId(pub String);

#[derive(Debug)]
pub struct Edge {
    pub id: EdgeId,
    pub node: NodeId,
    pub travel_time: Minute,
}

impl Edge {
    pub fn new(name: &str, node: NodeId, travel_time: Minute) -> Self {
        Self {
            id: EdgeId(name.into()),
            node,
            travel_time,
        }
    }
}
