use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
// pub enum DiningHall {
//     Lothian,
//     Glasgow,
// }

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    // pub dining_hall: DiningHall,
    pub url: String
}
