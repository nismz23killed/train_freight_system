use serde::{Deserialize, Serialize};

use serde_derive::Deserialize;
use train_freight_system::{
    train_freight_system::error::Result,
    train_freight_system::TrainFreightSystem,
    util::{kilogram::Kilogram, minute::Minute},
};

#[derive(Deserialize, Debug)]
struct NodeInput {
    name: String,
}

fn show_options() {
    println!("Select options below");
    println!("[N] Node input [ ex: N,A where A=name]");
    println!("[E] Edge input [ ex: E,E1,A,B,30 where E1=name, A=node1, B=node2, 30=travel time]");
    println!("[T] Train input [ ex: T,Q1,6,B where Q1=name, 6=Capacity, B=node location]");
    println!("[P] Package input [ ex: P,K1,5,A,C where K1=name 5=Weight, A=node origin, B=node destination]");
    println!("[X] deliver packages");
    println!("[C] Clear data");
    println!("[_]Any invalid keys will show the options");
}

fn main() {
    let stdin = std::io::stdin();
    let mut system = TrainFreightSystem::default();
    show_options();
    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Unable to read input");
        input = input.to_uppercase();
        match &input[..1] {
            "N" => {
                let node: Vec<&str> = input.split(',').map(|str| str.trim()).collect();
                if node.len() == 2 && !node[1].is_empty() {
                    system
                        .add_node(node[1])
                        .unwrap_or_else(|err| println!("{:?}", err));
                } else {
                    println!("Invalid node entry")
                }
            }
            "E" => {
                let edge: Vec<&str> = input.split(',').map(|str| str.trim()).collect();
                if edge.len() == 5 && !edge[1].is_empty() {
                    let time = if let Some(time) = edge[4].clone().parse::<u32>().ok() {
                        time
                    } else {
                        println!("Invalid travel time");
                        continue;
                    };
                    system
                        .add_edge(edge[1], edge[2], edge[3], Minute(time))
                        .unwrap_or_else(|err| println!("{:?}", err));
                } else {
                    println!("Invalid edge entry");
                }
            }
            "T" => {
                let train: Vec<&str> = input.split(',').map(|str| str.trim()).collect();
                if train.len() == 4 && !train[1].is_empty() {
                    let capacity = if let Some(capacity) = train[2].clone().parse::<u32>().ok() {
                        capacity
                    } else {
                        println!("Invalid capacity");
                        continue;
                    };
                    system
                        .add_train(train[1], Kilogram(capacity), train[3])
                        .unwrap_or_else(|err| println!("{:?}", err));
                } else {
                    println!("Invalid train entry");
                }
            }
            "P" => {
                let package: Vec<&str> = input.split(',').map(|str| str.trim()).collect();
                if package.len() == 5 && !package[1].is_empty() {
                    let weight = if let Some(weight) = package[2].clone().parse::<u32>().ok() {
                        weight
                    } else {
                        println!("Invalid weight");
                        continue;
                    };
                    system
                        .add_package(package[1], Kilogram(weight), package[3], package[4])
                        .unwrap_or_else(|err| println!("{:?}", err));
                } else {
                    println!("Invalid package entry");
                }
            }
            "X" => {
                let total_delivery_time = system.deliver_packages();
                println!("completed delivery in: {:?}", total_delivery_time);
            }
            "C" => {
                system = TrainFreightSystem::default();
                println!("Cleared");
            }
            _ => {
                show_options();
            }
        }
    }

    // let system = system
    //     .node("A")?
    //     .node("B")?
    //     .node("C")?
    //     .node("D")?
    //     .edge("E1", "A", "B", Minute(30))?
    //     .edge("E2", "B", "C", Minute(10))?
    //     .edge("E3", "C", "D", Minute(20))?
    //     .package("K1", Kilogram(5), "C", "A")?
    //     .package("K2", Kilogram(4), "D", "A")?
    //     .train("Q1", Kilogram(10), "B")?;

    // let total_delivery_time = system.deliver_packages();
}
