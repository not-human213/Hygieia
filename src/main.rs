mod hardware;
mod monitors;
mod models;

fn main() {
    println!("Welcome to Progobrrr!");

    // Display hardware profile
    // let profile = hardware::hardware_profile();
    // println!("Hardware Profile: {:?}", profile);

    // Start monitoring system resources
    monitors::start_monitoring();
}