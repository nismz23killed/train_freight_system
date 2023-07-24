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
        .node("D")?
        .edge("E1", "A", "B", Minute(30))?
        .edge("E2", "B", "C", Minute(10))?
        .edge("E3", "C", "D", Minute(20))?
        .package("K1", Kilogram(5), "C", "A")?
        .package("K2", Kilogram(4), "D", "A")?
        .train("Q1", Kilogram(10), "B")?;

    let total_delivery_time = system.deliver_packages();

    println!("completed delivery in: {:?}", total_delivery_time);
    Ok(())
}
