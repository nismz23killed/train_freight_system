use crate::util::kilogram::Kilogram;

use super::{node::NodeId, train::TrainId};

#[derive(Debug, Default)]
pub enum Status {
    #[default]
    NotReady,
    DroppedAt(NodeId),
    LoadedTo(TrainId),
    Delivered,
}

#[derive(Debug, Default, PartialEq)]
pub struct PackageId(pub String);

#[derive(Debug, Default)]
pub struct Package {
    pub id: PackageId,
    pub weight: Kilogram,
    pub destination: NodeId,
    pub status: Status,
}

impl Package {
    pub fn new(name: &str, weight: Kilogram, origin: NodeId, destination: NodeId) -> Self {
        let status: Status;
        if origin == destination {
            status = Status::Delivered;
        } else {
            status = Status::DroppedAt(origin);
        }

        Self {
            id: PackageId(name.into()),
            weight,
            destination,
            status,
        }
    }
}
