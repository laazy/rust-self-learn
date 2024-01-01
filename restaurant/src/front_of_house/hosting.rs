// use super::serving;
use crate::front_of_house::serving::serving::take_order;

pub fn add_to_waitlist() {
    seat_at_table();
    // serving::take_order();
    take_order();
}

fn seat_at_table() {}
