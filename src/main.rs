use train_freight_system::{
    train_freight_system::error::Result,
    train_freight_system::TrainFreightSystem,
    util::{kilogram::Kilogram, minute::Minute},
};

#[tokio::main]
async fn main() -> Result<()> {
    let mut binding = TrainFreightSystem::default();
    let system = binding
        .node("A")?
        .node("B")?
        .node("C")?
        .edge("E1", "A", "B", Minute(30))?
        .edge("E2", "B", "C", Minute(10))?
        .package("K1", Kilogram(5), "A", "C")?
        .train("Q1", Kilogram(6), "B")?;

    let total_delivery_time = system.deliver_packages();

    println!("total journey time to complete the delivery: {:?}", total_delivery_time);
    Ok(())
}
