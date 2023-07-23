use crate::util::{kilogram::Kilogram, minute::Minute};

use super::{
    node::NodeId,
    package::{self, Package},
};

#[derive(Debug, Default, PartialEq, Clone)]
pub enum Status {
    #[default]
    NotAvailable,
    StoppedAt(NodeId),
    DeliveringTo(NodeId, NodeId, Minute),
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TrainId(pub String);

#[derive(Debug, Default, Clone)]
pub struct Train {
    pub id: TrainId,
    pub max_capacity: Kilogram,
    pub status: Status,
    pub load_size: Kilogram,
}

impl Train {
    pub fn new(name: &str, max_capacity: Kilogram, location: NodeId) -> Self {
        Self {
            id: TrainId(name.into()),
            max_capacity,
            status: Status::StoppedAt(location),
            load_size: Kilogram(0),
        }
    }

    pub fn load_package(&mut self, package: &mut Package) {
        let available_size = self.max_capacity.clone() - self.load_size.clone();
        if package.weight <= available_size {
            self.load_size = self.load_size.clone() + package.weight.clone();
            package.status = package::Status::LoadedTo(self.id.clone());
        }
    }

    pub fn deliver_to(&mut self, origin: &NodeId, destination: &NodeId, travel_time: Minute) {
        self.status = Status::DeliveringTo(origin.clone(), destination.clone(), travel_time);
    }

    pub fn stopped(&mut self, node: &NodeId) {
        self.status = Status::StoppedAt(node.clone());
    }

    pub fn unload_package(&mut self, package: &mut Package, node: &NodeId) {
        if &package.destination == node {
            package.status = package::Status::Delivered(self.id.clone());
        } else {
            package.status = package::Status::DroppedAt(node.clone(), self.id.clone());
        }

        self.load_size = self.load_size.clone() - package.weight.clone();
    }
}
