use sysinfo::{Components, Disks, Networks, System};

fn main() {
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

        println!("Memory: {} MB / {} MB", sys.used_memory() /1024/1024, sys.total_memory() /1024/1024);

        println!("CPU usage: {}%", sys.global_cpu_usage());
        println!("CPU frequency: {} MHz", sys.cpus()[0].frequency());
        let (temperature_sum, temperature_count) = components
            .iter()
            .filter_map(|component| component.temperature())
            .fold((0.0_f32, 0_usize), |(sum, count), temperature| {
                (sum + temperature, count + 1)
            });

        if temperature_count > 0 {
            println!(
                "CPU temperature: {:.1}°C",
                temperature_sum / temperature_count as f32
            );
        } else {
            println!("CPU temperature: unavailable");
        }

    }
}
