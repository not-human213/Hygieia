use std::{thread, time::Duration};
use sysinfo::{Components, Disks, Networks, System};

fn main() {
    let mut sys = System::new_all();
    let mut networks = Networks::new_with_refreshed_list();
    let mut disks = Disks::new_with_refreshed_list();
    let mut components = Components::new_with_refreshed_list();

    loop{
        sys.refresh_all();
        networks.refresh(true);
        disks.refresh(true);
        components.refresh(true);

        println!("Memory: {} MB / {} MB", sys.used_memory() /1024/1024, sys.total_memory() /1024/1024);
    }
}
