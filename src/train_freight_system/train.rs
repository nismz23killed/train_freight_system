use crate::util::{kilogram::Kilogram, minute::Minute};

use super::{
    error::{Error, ErrorKind, Result},
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
    pub fn new(name: &str, max_capacity: Kilogram) -> Self {
        Self {
            id: TrainId(name.into()),
            max_capacity,
            ..Default::default()
        }
    }

    pub fn set_location(&mut self, location: NodeId) {
        self.status = Status::StoppedAt(location);
    }

    pub fn get_location(&mut self) -> Option<NodeId> {
        match &self.status {
            Status::StoppedAt(node) => Some(node.clone()),
            _ => None,
        }
    }

    pub fn load_package(&mut self, package: &mut Package) {
        let available_size = self.max_capacity.clone() - self.load_size.clone();
        if package.weight <= available_size {
            self.load_size = self.load_size.clone() + package.weight.clone();
            package.status = package::Status::LoadedTo(self.id.clone());
        }
    }

    pub fn move_to(&mut self, origin: &NodeId, destination: &NodeId, travel_time: Minute) {
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

#[derive(Debug, Default)]
pub struct TrainHandler {
    pub trains: Vec<Train>,
}

impl TrainHandler {
    pub fn add_train(
        &mut self,
        name: &str,
        max_capacity: Kilogram,
        location: &NodeId,
    ) -> Result<()> {
        let mut train = Train::new(name, max_capacity);
        train.set_location(location.clone());

        if self.find_train_index_by_name(name).is_some() {
            return Err(Error::new(
                ErrorKind::AddTrainError,
                format!("Train '{name}' already existing"),
            )
            .into());
        }
        self.trains.push(train);
        Ok(())
    }

    fn find_train_index_by_name(&self, train_name: &str) -> Option<usize> {
        let train_id = TrainId(train_name.into());
        self.find_train_index_by_id(&train_id)
    }

    fn find_train_index_by_id(&self, train_id: &TrainId) -> Option<usize> {
        self.trains.iter().position(|train| train.id == *train_id)
    }

    fn list_trains_stopped_at_node(&self, node_id: &NodeId) -> Vec<&Train> {
        self.trains
            .iter()
            .filter(|train| train.status == Status::StoppedAt(node_id.clone()))
            .collect()
    }

    pub fn find_largest_capacity_train_in_node(&mut self, node_id: &NodeId) -> Option<TrainId> {
        let trains = self.list_trains_stopped_at_node(node_id);
        let mut biggest_train_index: Option<usize> = None;
        for (index, train) in trains.iter().enumerate() {
            if biggest_train_index.is_none() {
                biggest_train_index = Some(index);
                continue;
            }
            let biggest_train = &self.trains[biggest_train_index.unwrap()];
            if biggest_train.max_capacity < train.max_capacity {
                biggest_train_index = Some(index);
            }
        }
        biggest_train_index.map(|index| self.trains[index].id.clone())
    }

    pub fn get_moving_train_lowest_travel_time(&self) -> Option<Minute> {
        self.trains.iter().filter_map(|train| {
            match &train.status {
                Status::DeliveringTo(_, _, travel_time) => {
                    Some(travel_time.clone())
                },
                _ => None,
            }
        }).min()
    }

    pub fn time_elapsed(&mut self, duration: &Minute) {
        for train in &mut self.trains {
            match &train.status.clone() {
                Status::DeliveringTo(origin, destination, travel_time) => {
                    let remaining_time = travel_time.clone() - duration.clone();
                    if remaining_time == Minute(0) {
                        train.stopped(destination);
                    } else {
                        train.move_to(origin, destination, remaining_time);
                    }
                },
                _ => {},
            }
        }
    }

    fn list_stopped_trains(&self) -> Vec<TrainId> {
        self.trains.iter().filter(|train| {
            match &train.status {
                Status::StoppedAt(_) => true,
                _ => false,
            }
       })
       .map(|train| train.id.clone())
       .collect()
    }

    pub fn unload_packages_in_trains_that_stopped(&mut self, packages: &mut Vec<Package>) {
        for train_id in &self.list_stopped_trains() {
            for package in packages.iter_mut() {
                if package.is_package_loaded_in_train(train_id) {
                    let pos = self.find_train_index_by_id(train_id).unwrap();
                    let node_id = &self.trains[pos].get_location().unwrap();
                    self.trains[pos].unload_package(package, node_id);
                }
            }
        }
    }

    pub fn load_package(&mut self, train_id: &TrainId, package: &mut Package) {
        let pos = self.find_train_index_by_id(train_id).unwrap();
        self.trains[pos].load_package(package);
    }

    pub fn move_to_node(&mut self, train_id: &TrainId, origin: &NodeId, destination: &NodeId, travel_time: Minute) {
        let pos = self.find_train_index_by_id(train_id).unwrap();
        self.trains[pos].move_to(origin, destination, travel_time);
    }
}
