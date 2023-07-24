use crate::util::kilogram::Kilogram;

use super::{error::Error, error::ErrorKind, error::Result, node::NodeId, train::TrainId};

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
        let status = if origin == destination {
            Status::Delivered(TrainId::default())
        } else {
            Status::DroppedAt(origin, TrainId::default())
        };

        Self {
            id: PackageId(name.into()),
            weight,
            destination,
            status,
        }
    }

    pub fn is_package_loaded_in_train(&self, train_id: &TrainId) -> bool {
        match &self.status {
            Status::LoadedTo(train) => train == train_id,
            _ => false,
        }
    }

    pub fn get_carrier(&self) -> Option<&TrainId> {
        match &self.status {
            Status::LoadedTo(train) => Some(train),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
pub struct PackageHandler {
    pub packages: Vec<Package>,
}

impl PackageHandler {
    pub fn find_package_index_by_name(&self, name: &str) -> Option<usize> {
        let package_id = PackageId(name.into());
        self.find_package_index_by_id(&package_id)
    }

    pub fn find_package_index_by_id(&self, package_id: &PackageId) -> Option<usize> {
        self.packages
            .iter()
            .position(|package| package.id == *package_id)
    }

    pub fn add_package(
        &mut self,
        name: &str,
        weight: Kilogram,
        origin: NodeId,
        destination: NodeId,
    ) -> Result<()> {
        if self.find_package_index_by_name(name).is_some() {
            return Err(Error::new(
                ErrorKind::AddPackageError,
                format!("Package '{name}' already existed"),
            )
            .into());
        }
        self.packages
            .push(Package::new(name, weight, origin, destination));
        Ok(())
    }

    pub fn have_undelivered_packages(&self) -> bool {
        self.packages.iter().any(|package| match &package.status {
            Status::NotReady => false,
            Status::DroppedAt(_, _) => true,
            Status::LoadedTo(_) => true,
            Status::Delivered(_) => false,
        })
    }

    pub fn list_undelivered_packages_at_node(&self, node_id: &NodeId) -> Vec<PackageId> {
        let packages = self.packages.clone();
        packages
            .into_iter()
            .filter(|package| match &package.status {
                Status::DroppedAt(id, _) => id == node_id,
                _ => false,
            })
            .map(|x| x.id)
            .collect()
    }

    pub fn get_package_mut(&mut self, package_id: &PackageId) -> Option<&mut Package> {
        self.find_package_index_by_id(package_id)
            .map(|pos| &mut self.packages[pos])
    }

    pub fn get_package(&self, package_id: &PackageId) -> Option<&Package> {
        self.find_package_index_by_id(package_id)
            .map(|pos| &self.packages[pos])
    }

    pub fn list_package_names_in_transit(&self, train_id: &TrainId) -> Vec<String> {
        self.packages
            .iter()
            .filter(|package| match &package.status {
                Status::LoadedTo(id) => id == train_id,
                _ => false,
            })
            .map(|package| package.id.0.to_owned())
            .collect()
    }

    pub fn list_package_names_delivered(&self, train_id: &TrainId) -> Vec<String> {
        self.packages
            .iter()
            .filter(|package| match &package.status {
                Status::Delivered(id) => id == train_id,
                _ => false,
            })
            .map(|package| package.id.0.to_owned())
            .collect()
    }
}
