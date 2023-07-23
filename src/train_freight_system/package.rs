use crate::util::kilogram::Kilogram;

use super::{node::NodeId, train::TrainId};

#[derive(Debug, Default, PartialEq, Clone)]
pub enum Status {
    #[default]
    NotReady,
    DroppedAt(NodeId, TrainId),
    LoadedTo(TrainId),
    Delivered(TrainId),
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PackageId(pub String);

#[derive(Debug, Default, Clone)]
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
            status = Status::Delivered(TrainId::default());
        } else {
            status = Status::DroppedAt(origin, TrainId::default());
        }

        Self {
            id: PackageId(name.into()),
            weight,
            destination,
            status,
        }
    }
}
