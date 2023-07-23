use crate::util::kilogram::Kilogram;

use super::{node::NodeId, package::Package};

#[derive(Debug, Default)]
pub enum Status {
    #[default]
    NotAvailable,
    StoppedAt(NodeId),
    DeliveringTo(NodeId, Vec<Package>),
}

#[derive(Debug, Default, PartialEq)]
pub struct TrainId(pub String);

#[derive(Debug, Default)]
pub struct Train {
    pub id: TrainId,
    pub max_capacity: Kilogram,
    pub status: Status,
}

impl Train {
    pub fn new(name: &str, max_capacity: Kilogram, location: NodeId) -> Self {
        Self {
            id: TrainId(name.into()),
            max_capacity,
            status: Status::StoppedAt(location),
        }
    }
}
