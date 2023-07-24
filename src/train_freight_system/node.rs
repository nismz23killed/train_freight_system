use super::{
    edge::{Edge, EdgeId},
    error::{Error, ErrorKind, Result},
};

#[derive(Debug, Default, PartialEq, Clone, Hash, Eq)]
pub struct NodeId(pub String);

#[derive(Debug, Default)]
pub struct Node {
    pub id: NodeId,
    pub edges: Vec<Edge>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            id: NodeId(name.into()),
            ..Default::default()
        }
    }

    fn find_edge_index_by_id(&self, id: &EdgeId) -> Option<usize> {
        self.edges.iter().position(|edge| edge.id == *id)
    }

    pub fn find_edge_with_node(&self, node_id: &NodeId) -> Option<&Edge> {
        self.edges.iter().find(|&edge| edge.node == *node_id)
    }

    pub fn add_edge(&mut self, edge: Edge) -> Result<()> {
        if self.find_edge_index_by_id(&edge.id).is_some() {
            return Err(Error::new(
                ErrorKind::AddEdgeError,
                format!("Edge '{:?}' already exising", edge.id),
            )
            .into());
        }
        self.edges.push(edge);
        Ok(())
    }
}
