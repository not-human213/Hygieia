use sysinfo::System;

pub fn hardware_profile() {
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
}