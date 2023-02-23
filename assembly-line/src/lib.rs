use std::ops::Div;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let total_cars_produced: f64 = speed as f64 * 221.;
    match speed {
        1..=4 => total_cars_produced,
        5..=8 => total_cars_produced * (90.).div(100.),
        9..=10 => total_cars_produced * (77.).div(100.),
        _ => 0.,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}
