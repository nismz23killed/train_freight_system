use crate::util::{kilogram::Kilogram, minute::Minute};

use self::{
    edge::Edge,
    error::{Error, ErrorKind, Result},
    node::{Node, NodeId},
    package::{Package, PackageId},
    train::{Train, TrainId},
};

pub(crate) mod edge;
pub mod error;
pub(crate) mod node;
pub(crate) mod package;
pub(crate) mod train;

#[derive(Debug, Default)]
pub struct TrainFreightSystem {
    pub nodes: Vec<Node>,
    pub trains: Vec<Train>,
    pub packages: Vec<Package>,
}

impl TrainFreightSystem {
    pub fn node(&mut self, name: &str) -> Result<&mut Self> {
        if self.find_node_index_by_name(name).is_some() {
            return Err(Error::new(
                ErrorKind::AddStationError,
                format!("Station '{name}' already existing"),
            )
            .into());
        }

        self.nodes.push(Node::new(name));
        Ok(self)
    }

    fn find_node_index_by_name(&self, node_name: &str) -> Option<usize> {
        self.find_node_index_by_id(&NodeId(node_name.into()))
    }

    fn find_node_index_by_id(&self, node_id: &NodeId) -> Option<usize> {
        self.nodes.iter().position(|node| node.id == *node_id)
    }

    pub fn edge(
        &mut self,
        name: &str,
        node_1: &str,
        node_2: &str,
        travel_time: Minute,
    ) -> Result<&mut Self> {
        let node_1_pos = self.find_node_index_by_name(node_1).ok_or_else(|| {
            Error::new(
                ErrorKind::AddEdgeError,
                format!("Node1 '{node_1}' non-existent"),
            )
        })?;

        let node_2_pos = self.find_node_index_by_name(node_2).ok_or_else(|| {
            Error::new(
                ErrorKind::AddEdgeError,
                format!("Node2 '{node_2}' non-existent"),
            )
        })?;

        if node_1_pos == node_2_pos {
            return Err(Error::new(
                ErrorKind::AddEdgeError,
                format!("Node1 and Node2 are the same"),
            )
            .into());
        }

        // Push edges on both sides
        self.nodes[node_1_pos].add_edge(Edge::new(
            name,
            NodeId(node_2.into()),
            travel_time.clone(),
        ))?;
        self.nodes[node_2_pos].add_edge(Edge::new(name, NodeId(node_1.into()), travel_time))?;
        Ok(self)
    }

    fn find_train_index_by_name(&self, train_name: &str) -> Option<usize> {
        let train_id = TrainId(train_name.into());
        self.trains.iter().position(|train| train.id == train_id)
    }

    pub fn train(
        &mut self,
        name: &str,
        max_capacity: Kilogram,
        location: &str,
    ) -> Result<&mut Self> {
        let pos = self.find_node_index_by_name(location).ok_or_else(|| {
            Error::new(
                ErrorKind::AddTrainError,
                format!("Node '{location} doesn't exist"),
            )
        })?;

        let train = Train::new(name, max_capacity, self.nodes[pos].id.clone());
        if self.find_train_index_by_name(name).is_some() {
            return Err(Error::new(
                ErrorKind::AddTrainError,
                format!("Train '{name}' already existing"),
            )
            .into());
        }
        self.trains.push(train);
        Ok(self)
    }

    fn find_package_index_by_name(&self, name: &str) -> Option<usize> {
        let package_id = PackageId(name.into());
        self.packages
            .iter()
            .position(|package| package.id == package_id)
    }

    pub fn package(
        &mut self,
        name: &str,
        weight: Kilogram,
        origin: &str,
        destination: &str,
    ) -> Result<&mut Self> {
        let origin_pos = self.find_node_index_by_name(origin).ok_or_else(|| {
            Error::new(
                ErrorKind::AddPackageError,
                format!("Node '{origin}' doesn't exist"),
            )
        })?;

        let destination_pos = self.find_node_index_by_name(destination).ok_or_else(|| {
            Error::new(
                ErrorKind::AddPackageError,
                format!("Node '{destination}' doesn't exist"),
            )
        })?;

        let origin_id = self.nodes[origin_pos].id.clone();
        let destination_id = self.nodes[destination_pos].id.clone();

        if self.find_package_index_by_name(name).is_some() {
            return Err(Error::new(
                ErrorKind::AddPackageError,
                format!("Package '{name}' already existed"),
            )
            .into());
        }
        self.packages
            .push(Package::new(name, weight, origin_id, destination_id));
        Ok(self)
    }

    pub fn deliver_packages(&mut self) -> Minute {
        Minute(0)
    }
}
