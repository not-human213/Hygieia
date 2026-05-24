mod hardware;
mod monitors;

fn main() {
    println!("Welcome to Progobrrr!");

    // Display hardware profile
    hardware::hardware_profile();

    // Start monitoring system resources
    monitors::start_monitoring();
}