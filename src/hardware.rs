use chrono::Utc;
use sysinfo::System;

use crate::models::HardwareProfile;

pub fn hardware_profile() -> HardwareProfile {
    let sys = System::new_all();

    // OS info
    println!("OS name: {:?}", System::name());
    println!("OS version: {:?}", System::os_version());
    println!("Kernel version: {:?}", System::kernel_version());
    println!("Host name: {:?}", System::host_name());

    //CPU info

    let cpus = sys.cpus();
    let brand = cpus
        .first()
        .map(|cpu| cpu.brand())
        .unwrap_or("Unknown CPU");

    let vendor = cpus
        .first()
        .map(|cpu| cpu.vendor_id())
        .unwrap_or("Unknown Vendor");

    println!("CPU: {} ({})", brand, vendor);
    println!("Logical cores: {}", cpus.len());
    println!("Physical cores: {}", System::physical_core_count().unwrap_or(0));

    // Memory info
    let total_memory = sys.total_memory() / 1024 / 1024; 
    let total_swap = sys.total_swap() / 1024 / 1024;

    println!("Memory: {} MB", total_memory);
    println!("Swap: {} MB", total_swap);


    //create profile 
    let profile = HardwareProfile {
        hostname: System::host_name().unwrap_or("Unknown".to_string()),
        os_name: System::name().unwrap_or("Unknown".to_string()),
        os_version: System::os_version().unwrap_or("Unknown".to_string()),
        kernel_version: System::kernel_version().unwrap_or("Unknown".to_string()),
        cpu_brand: brand.to_string(),
        cpu_vendor: vendor.to_string(),
        cpu_physical_cores: System::physical_core_count().unwrap_or(0),
        cpu_logical_cores: cpus.len(),
        total_memory: total_memory,
        total_swap: total_swap,
        timestamp: Utc::now(),
    };

    profile
}