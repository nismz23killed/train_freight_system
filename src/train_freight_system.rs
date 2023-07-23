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

#[derive(Debug)]
pub enum DeliveryResult {
    NoPackages,
    NoTrains,
    NotAllPackageLoaded,
    AllPackageLoaded,
}

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
        self.find_train_index_by_id(train_id)
    }

    fn find_train_index_by_id(&self, train_id: TrainId) -> Option<usize> {
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
        self.find_package_index_by_id(&package_id)
    }

    fn find_package_index_by_id(&self, package_id: &PackageId) -> Option<usize> {
        self.packages
            .iter()
            .position(|package| package.id == *package_id)
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

    fn have_undelivered_packages(&self) -> bool {
        self.packages
            .iter()
            .any(|package| {
                match &package.status {
                    package::Status::NotReady => false,
                    package::Status::DroppedAt(_, _) => true,
                    package::Status::LoadedTo(_) => true,
                    package::Status::Delivered(_) => false,
                }
            })
    }

    fn list_undelivered_packages_at_node(&self, node_id: &NodeId) -> Vec<PackageId> {
        let packages = self.packages.clone();
        packages
            .into_iter()
            .filter(|package| 
                {
                    match &package.status {
                        package::Status::DroppedAt(id,_) => {
                            id == node_id
                        },
                        _ => false,
                    }
                }
            )
            .map(|x| x.id)
            .collect()
    }

    fn get_biggest_train_in_node(&self, node_id: &NodeId) -> Option<TrainId> {
        let mut biggest_train_index: Option<usize> = None;
        let trains: Vec<&Train> = self
            .trains
            .iter()
            .filter(|train| train.status == train::Status::StoppedAt(node_id.clone()))
            .collect();
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

    fn get_travel_time_from_routes(&self, routes: &Vec<NodeId>) -> Minute {
        let mut travel_time = Minute(0);
        for i in 1..routes.iter().count() {
            let pos = self.find_node_index_by_id(&routes[i - 1]).unwrap();
            if let Some(edge) = self.nodes[pos].find_edge_with_node(&routes[i]) {
                travel_time = travel_time + edge.travel_time.clone();
            }
        }

        travel_time
    }

    fn get_all_possible_routes(
        &self,
        origin: &NodeId,
        destination: &NodeId,
        possible_paths: &mut Vec<Vec<NodeId>>,
        routes: &mut Vec<NodeId>,
    ) {
        if routes.iter().find(|&node| node == origin).is_some() {
            return;
        }

        routes.push(origin.clone());

        let pos = self.find_node_index_by_id(origin).unwrap();
        for edge in self.nodes[pos].edges.iter() {
            if routes.iter().find(|&node| node == destination).is_some() {
                possible_paths.push(routes.clone());
                return;
            }
            let mut path: Vec<NodeId> = routes.clone();
            self.get_all_possible_routes(&edge.node, destination, possible_paths, &mut path);
            if path.iter().find(|&node| node == destination).is_some() {
                possible_paths.push(path);
            }
        }
    }

    fn get_least_time_path_to_deliver_package(
        &self,
        node_index: usize,
        package_id: &PackageId,
    ) -> Vec<NodeId> {
        let package = self
            .packages
            .iter()
            .find(|package| package.id == *package_id)
            .unwrap();

        let node = &self.nodes[node_index];
        let mut possible_routes: Vec<Vec<NodeId>> = vec![];
        let mut routes: Vec<NodeId> = vec![];
        self.get_all_possible_routes(
            &node.id,
            &package.destination,
            &mut possible_routes,
            &mut routes,
        );

        let mut least_travel_time = Minute(0);
        let mut least_travel_time_route: Vec<NodeId> = vec![];
        for path in possible_routes.iter() {
            let travel_time: Minute = self.get_travel_time_from_routes(path);
            if least_travel_time == Minute(0) || travel_time < least_travel_time {
                least_travel_time = travel_time;
                least_travel_time_route = path.clone();
            }
        }

        least_travel_time_route
    }

    fn deliver_packages_in_node(&mut self, node_id: &NodeId, node_index: usize) -> DeliveryResult {
        let packages = self.list_undelivered_packages_at_node(node_id);
        if packages.is_empty() {
            return DeliveryResult::NoPackages;
        }

        let biggest_train = if let Some(biggest_train) = self.get_biggest_train_in_node(node_id) {
            biggest_train
            // load packages going to
        } else {
            return DeliveryResult::NoTrains;
        };

        let mut highest_routes: Vec<NodeId> = vec![];
        for package in packages.iter() {
            let routes = self.get_least_time_path_to_deliver_package(node_index, package);
            if routes.len() > highest_routes.len() {
                highest_routes = routes;
            }
        }

        let destination = &highest_routes[1];
        let train_pos = self.find_train_index_by_id(biggest_train.clone()).unwrap();

        let filtered_packages = self
            .get_packages_passing_to_node(destination, &packages)
            .clone();

        for package in filtered_packages.iter() {
            let pos: usize = self.find_package_index_by_id(package).unwrap();
            let package_to_load = &mut self.packages[pos];
            self.trains[train_pos].load_package(package_to_load);
        }

        let travel_time = self.nodes[node_index]
            .find_edge_with_node(destination)
            .unwrap()
            .travel_time
            .clone();
        self.trains[train_pos].deliver_to(node_id, destination, travel_time);

        if !self.list_undelivered_packages_at_node(node_id).is_empty() {
            return DeliveryResult::NotAllPackageLoaded;
        }

        DeliveryResult::AllPackageLoaded
    }

    fn get_packages_passing_to_node(
        &self,
        node_id: &NodeId,
        packages: &Vec<PackageId>,
    ) -> Vec<PackageId> {
        let node_index = self.find_node_index_by_id(node_id).unwrap();
        let mut filtered_packages: Vec<PackageId> = vec![];
        for package in packages {
            if self
                .get_least_time_path_to_deliver_package(node_index, &package)
                .iter()
                .find(|&id| id == node_id)
                .is_some()
            {
                filtered_packages.push(package.clone());
            }
        }
        filtered_packages
    }

    fn deliver_packages_in_nodes(&mut self) {
        let nodes: Vec<NodeId> = self.nodes.iter().map(|node| node.id.clone()).collect();
        for (index, node_id) in nodes.iter().enumerate() {
            loop {
                match self.deliver_packages_in_node(node_id, index) {
                    DeliveryResult::NoPackages
                    | DeliveryResult::NoTrains
                    | DeliveryResult::AllPackageLoaded => break,
                    DeliveryResult::NotAllPackageLoaded => continue,
                }
            }
        }
        // have packages not picked up and have trains not moving
    }

    fn list_packages_in_train(&self, train_id: &TrainId) -> Vec<PackageId> {
        self.packages.iter().filter(|&package| {
            match &package.status {
                package::Status::LoadedTo(id) => {
                    id == train_id
                },
                _ => false,
            }
        })
        .map(|package| package.id.clone())
        .collect()
    }

    fn list_moving_trains(&mut self) -> Vec<Train> {
        self
            .trains
            .iter()
            .filter(|&train| match train.status {
                train::Status::DeliveringTo(_, _, _) => true,
                _ => false,
            })
            .map(|train| {
                train.clone()
            })
            .collect()
    }

    fn train_arrived(&mut self) -> Minute {
        let moving_trains = self.list_moving_trains();

        let mut least_travel_time = Minute(0);
        for train in moving_trains.iter() {
            let travel_time = match &train.status {
                train::Status::DeliveringTo(_, _, travel_time) => travel_time,
                _ => &least_travel_time,
            };

            if least_travel_time == Minute(0) || travel_time < &least_travel_time {
                least_travel_time = travel_time.clone();
            }
        }

        for train in moving_trains {
            match &train.status {
                train::Status::DeliveringTo(origin, dest, travel_time) => {
                    let remaining_time = travel_time.clone() - least_travel_time.clone();
                    let train_pos = self.find_train_index_by_id(train.id.clone()).unwrap();
                    if remaining_time == Minute(0) {
                        let packages = self.list_packages_in_train(&train.id);
                        self.trains[train_pos].stopped(dest);
                        
                        for package_id in packages.iter() {
                            let package_pos = self.find_package_index_by_id(package_id).unwrap();
                            self.trains[train_pos].unload_package(&mut self.packages[package_pos], dest);
                        }
                    } else {
                        self.trains[train_pos].deliver_to(origin, dest, remaining_time);
                    }
                },
                _ => {},
            }
        }

        least_travel_time
    }

    pub fn deliver_packages(&mut self) -> Minute {
        let mut total_delivery_time = Minute(0);

        while self.have_undelivered_packages() {
            let travel_time = self.train_arrived();
            total_delivery_time = total_delivery_time + travel_time;
            self.deliver_packages_in_nodes();

            for train in &self.trains {
                let (origin, destination) = match &train.status {
                    train::Status::StoppedAt(location) => (location.0.to_owned(), "".to_string()),
                    train::Status::DeliveringTo(origin, destination, _) => (origin.0.to_owned(), destination.0.to_owned()),
                    _ => ("".into(), "".into()),
                };

                let packages_loaded: Vec<String> = self.packages.iter().filter(|package|
                    {
                        match &package.status {
                            package::Status::LoadedTo(train_id) => {
                                train_id == &train.id
                            },
                            _ => false, 
                        }
                    }
                ).map(|package| package.id.0.to_owned()).collect();

                let packages_dropped: Vec<String> = self.packages.iter().filter(|package|
                    {
                        match &package.status {
                            package::Status::Delivered(train_id) => {
                                train_id == &train.id
                            },
                            _ => false, 
                        }
                    }
                ).map(|package| package.id.0.to_owned()).collect();

                println!("W={}, T={}, N1={}, P1={:?}, N2={}, P2 ={:?}"
                , total_delivery_time.0, train.id.0, origin, packages_loaded, destination, packages_dropped);
            }
           
        }

        total_delivery_time
    }
}
