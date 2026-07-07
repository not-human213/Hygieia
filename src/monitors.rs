use sysinfo::{Components, Disks, Networks, System};
use std::{thread, time::Duration};
use chrono::Utc;
use crate::models::{CpuMetrics, MemoryMetrics, NetworkMetrics};


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
        
        let now = Utc::now();
      
        let memory_metrics = MemoryMetrics {
            timestamp: now,
            used_memory: sys.used_memory() /1024/1024,
            used_swap: sys.used_swap() /1024/1024,
        };
        
     
                
        //CPU TEMP
        let comp : Vec<_>= components
        .iter()
        .filter(|component| (component.label()) == "coretemp Package id 0")
        .collect();


        let cpu_metrics = CpuMetrics {
            timestamp: now,
            temperature: comp[0].temperature(),
            usage: sys.global_cpu_usage(),
            frequency: sys.cpus()[0].frequency(),
        };


        //DISK PROB - not getting the physical diskdrive
        // println!("disks:");
        // for disk in &disks{
        //     let total_gb = disk.total_space() /1024/1024/1024;
        //     let available_gb = disk.available_space() /1024/1024/1024;
        //     println!(" {}: {} / {} GB available",
        //         disk.file_system().to_string_lossy(),
        //         disk.name().to_string_lossy(),
        //         total_gb,
        //     );
        // }

        let network_metrics: Vec<NetworkMetrics> = networks
            .iter()
            .filter(|(_,data)| data.received() > 0 || data.transmitted() > 0)
            .map(|(name, data)| NetworkMetrics {
                timestamp: now,
                interface_name: name.clone(),
                bytes_received: data.received(),
                bytes_transmitted: data.transmitted(),
            }).collect();

        println!("{:?}",cpu_metrics);
        println!("{:?}",memory_metrics);
        println!("{:?}",network_metrics);

        thread::sleep(Duration::from_secs(2));
            
    }
}
