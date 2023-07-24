use crate::util::{kilogram::Kilogram, minute::Minute};

use self::{
    edge::Edge,
    error::{Error, ErrorKind, Result},
    node::{Node, NodeId},
    package::{PackageHandler, PackageId},
    train::TrainHandler,
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
    TrainPicking,
}

#[derive(Debug, Default)]
pub struct TrainFreightSystem {
    pub nodes: Vec<Node>,
    pub train_handler: TrainHandler,
    pub package_handler: PackageHandler,
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
                "Node1 and Node2 are the same".to_string(),
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

        self.train_handler
            .add_train(name, max_capacity, &self.nodes[pos].id)?;
        Ok(self)
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
        self.package_handler
            .add_package(name, weight, origin_id, destination_id)?;
        Ok(self)
    }

    fn get_travel_time_from_routes(&self, routes: &Vec<NodeId>) -> Minute {
        let mut travel_time = Minute(0);
        for i in 1..routes.len() {
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
        if routes.iter().any(|node| node == origin) {
            return;
        }

        routes.push(origin.clone());

        let pos = self.find_node_index_by_id(origin).unwrap();
        for edge in self.nodes[pos].edges.iter() {
            if routes.iter().any(|node| node == destination) {
                possible_paths.push(routes.clone());
                return;
            }
            let mut path: Vec<NodeId> = routes.clone();
            self.get_all_possible_routes(&edge.node, destination, possible_paths, &mut path);
            if path.iter().any(|node| node == destination) {
                possible_paths.push(path);
            }
        }
    }

    fn get_least_time_path_to_deliver_package(
        &self,
        node_index: usize,
        package_id: &PackageId,
    ) -> Vec<NodeId> {
        let package = self.package_handler.get_package(package_id).unwrap();

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

    fn list_all_undelivered_packages_least_possible_routes(&mut self) -> Vec<Vec<NodeId>> {
        let packages = self.package_handler.list_undelivered_packages();
        let mut packages_routes: Vec<Vec<NodeId>> = vec![];
        for package_id in &packages {
            let package = self.package_handler.get_package(package_id).unwrap();
            let node_id = package.get_location().unwrap();
            let node_index = self.find_node_index_by_id(node_id).unwrap();
            let routes = self.get_least_time_path_to_deliver_package(node_index, package_id);
            packages_routes.push(routes);
        }
        packages_routes
    }

    fn deliver_packages_in_node(&mut self, node_id: &NodeId, node_index: usize) -> DeliveryResult {
        let packages = self
            .package_handler
            .list_undelivered_packages_at_node(node_id);
        if packages.is_empty() {
            return DeliveryResult::NoPackages;
        }

        let biggest_train = if let Some(biggest_train) = self
            .train_handler
            .find_largest_capacity_train_in_node(node_id)
        {
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
        let filtered_packages = self.get_packages_passing_to_node(destination, &packages);
        // Before loading packages, check if there are packages closer to
        // this route that can still fit in this train.
        // if there are then move train to that direction
        let all_routes = self.list_all_undelivered_packages_least_possible_routes();
        for routes in &all_routes {
            let diff: Vec<&NodeId> = routes.iter().filter(|node_id| !highest_routes.contains(node_id)).rev().collect();
            for check_node_id in diff.clone() {
                let time1 = self.get_travel_time_from_routes(&vec![check_node_id.clone(), node_id.clone()]);
                let time2 = self.get_travel_time_from_routes(&highest_routes);
                if time1 < time2 {
                    let packages = self.package_handler.list_undelivered_packages_at_node(check_node_id);
                    for package_id in &packages {
                        let train = self.train_handler.get_train(&biggest_train).unwrap();
                        let this_package = self.package_handler.get_package(package_id).unwrap();
                        if train.can_accomodate_package(this_package) {
                            let time =  self.get_travel_time_from_routes(&vec![diff[0].clone(), node_id.clone()]);
                            self.train_handler.move_to_node(&biggest_train, node_id, diff[0], time);
                            return DeliveryResult::TrainPicking;
                        }
                    }
                }
            }
        }

        for package_id in filtered_packages.iter() {
            let package = self.package_handler.get_package_mut(package_id).unwrap();
            self.train_handler.load_package(&biggest_train, package);
        }

        let travel_time = self.nodes[node_index]
            .find_edge_with_node(destination)
            .unwrap()
            .travel_time
            .clone();
        self.train_handler
            .move_to_node(&biggest_train, node_id, destination, travel_time);

        if !self
            .package_handler
            .list_undelivered_packages_at_node(node_id)
            .is_empty()
        {
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
                .get_least_time_path_to_deliver_package(node_index, package)
                .iter()
                .any(|id| id == node_id)
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
                    DeliveryResult::NotAllPackageLoaded
                    | DeliveryResult::TrainPicking => continue,
                }
            }
        }
        // have packages not picked up and have trains not moving
        let dropped_packages = self.package_handler.list_undelivered_packages();
        for package_id in &dropped_packages {
            let package = self.package_handler.get_package(package_id).unwrap();
            let node_id = package.get_location().unwrap();
            let node_index = self.find_node_index_by_id(node_id).unwrap();
            let routes = self.get_least_time_path_to_deliver_package(node_index, package_id);

            for i in 1..routes.len() {
                for train_id in &self.train_handler.list_stopped_trains_at_node(&routes[i]){
                    let this_route = vec![routes[i-1].clone(), routes[i].clone()];
                    let travel_time = self.get_travel_time_from_routes(&this_route);
                    self.train_handler.move_to_node(train_id, &routes[i], &routes[i-1], travel_time);
                    break;
                }
            }
        }
    }

    fn train_arrived(&mut self) -> Minute {
        let least_travel_time = self
            .train_handler
            .get_moving_train_lowest_travel_time()
            .unwrap_or(Minute(0));
        self.train_handler.time_elapsed(&least_travel_time);

        self.train_handler
            .unload_packages_in_trains_that_stopped(&mut self.package_handler.packages);

        least_travel_time.clone()
    }

    pub fn deliver_packages(&mut self) -> Minute {
        let mut total_delivery_time = Minute(0);

        while self.package_handler.have_undelivered_packages() {
            let travel_time = self.train_arrived();
            total_delivery_time = total_delivery_time + travel_time;
            self.deliver_packages_in_nodes();

            for train in &self.train_handler.trains {
                let (origin, destination) = match &train.status {
                    train::Status::StoppedAt(location) => (location.0.to_owned(), "".to_string()),
                    train::Status::DeliveringTo(origin, destination, _) => {
                        (origin.0.to_owned(), destination.0.to_owned())
                    }
                    _ => ("".into(), "".into()),
                };

                println!(
                    "W={}, T={}, N1={}, P1={:?}, N2={}, P2 ={:?}",
                    total_delivery_time.0,
                    train.id.0,
                    origin,
                    self.package_handler
                        .list_package_names_in_transit(&train.id),
                    destination,
                    self.package_handler.list_package_names_delivered(&train.id)
                );
            }
        }

        total_delivery_time
    }
}
