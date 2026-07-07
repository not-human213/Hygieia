use sysinfo::{Components, Disks, Networks, System};
use std::{thread, time::Duration};
use chrono::Utc;
use crate::models::{CpuMetrics};


pub fn start_monitoring() {
    let mut sys = System::new_all();
    let mut networks = Networks::new_with_refreshed_list();
    let mut disks = Disks::new_with_refreshed_list();
    let mut components = Components::new_with_refreshed_list();


    println!("System name {:?}",System::name());
    println!("CPUs {}", sys.cpus().len());
    loop{
        sys.refresh_all();
        networks.refresh(true);
        disks.refresh(true);
        components.refresh(true);

        println!("Memory: {} MB / {} MB", 
                sys.used_memory() /1024/1024,
                sys.total_memory() /1024/1024);

        println!("CPU usage: {}%",
                sys.global_cpu_usage());


        println!("CPU frequency: {} MHz", 
                sys.cpus()[0].frequency());

                
        //CPU TEMP
        let comp : Vec<_>= components
        .iter()
        .filter(|component| (component.label()) == "coretemp Package id 0")
        .collect();
        println!("{:?}", comp[0].temperature());
        
        
        let now = Utc::now();


        let cpu_metrics = CpuMetrics {
            captured_at: now,
            temperature: comp[0].temperature(),
            usage: sys.global_cpu_usage(),
            frequency: sys.cpus()[0].frequency(),
        }









        println!("disks:");
        for disk in &disks{
            let total_gb = disk.total_space() /1024/1024/1024;
            let available_gb = disk.available_space() /1024/1024/1024;
            println!(" {}: {} / {} GB available",
                disk.file_system().to_string_lossy(),
                disk.name().to_string_lossy(),
                total_gb,
            );
        }

        println!("{}",std::any::type_name_of_val(&networks));
        for (interface_name, data) in &networks{
            println!(" [{}] received: {} bytes, transmitted: {} bytes",
                interface_name,
                data.received(),
                data.transmitted()
            );
        }



        thread::sleep(Duration::from_secs(2));
            
    }
}
