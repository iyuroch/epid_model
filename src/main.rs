// use std::string;


mod individual;
use individual::individual::{Individual, InfectionData};

fn main() {
    let inf_data = InfectionData::new(
        15, 0.8, 6, 2,
    );

    let new_ind = Individual::new(
        0, 0, 2, Some(7),
        100, 100, inf_data,
    );

    // individual::individual::clarinet();
    println!("Hello, world!");

    println! ("{:?}", new_ind);
}
