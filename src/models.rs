use chrono::{DateTime, Utc};
use serde::Serialize;

// maps to 'hardware_profile' table in the database

#[derive(Debug, Serialize)]
pub struct HardwareProfile{
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub cpu_brand: String,
    pub cpu_vendor: String,
    pub cpu_physical_cores: usize,
    pub cpu_logical_cores: usize,
    pub total_memory: u64,
    pub total_swap: u64,
    pub timestamp: DateTime<Utc>,
}

// maps to 'disk_hardware' table in the database

#[derive(Debug, Serialize)]
pub struct DiskHardware{
    pub name: String,
    pub mount_point: String,
    pub file_system: String,
    pub total_space: u64,
    pub available_space: u64,
}

// maps to 'network_hardware' table in the database

#[derive(Debug, Serialize)]
pub struct NetworkHardware{
    pub interface_name: String,
}

// live monitoring data structures

// maps to 'cpu_metrics' table in the database

#[derive(Debug, Serialize)]
pub struct CpuMetrics{
    pub captured_at : DateTime<Utc>,
    pub temperature : f32,
    pub usage : f32,
    pub frequency: u64,
}

// maps to 'memory_metrics' table in the database

#[derive(Debug, Serialize)]
pub struct MemoryMetrics{
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
    pub timestamp: DateTime<Utc>,
}

// maps to 'disk_metrics' table in the database

#[derive(Debug, Serialize)]
pub struct DiskMetrics{
    pub name: String,
    pub mount_point: String,
    pub file_system: String,
    pub total_space: u64,
    pub available_space: u64,
    pub timestamp: DateTime<Utc>,
}

// maps to 'network_metrics' table in the database

#[derive(Debug, Serialize)]
pub struct NetworkMetrics{
    pub interface_name: String,
    pub bytes_received: u64,
    pub bytes_transmitted: u64,
    pub timestamp: DateTime<Utc>,
}

//System events collected at flush

#[derive(Debug, Serialize)]
pub struct SystemEvent{
    pub event_id: i64,
    pub level: String,
    pub source: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

// Memory buffer

pub struct MetricsBuffer {
    pub cpu: Vec<CpuMetrics>,
    pub memory: Vec<MemoryMetrics>,
    pub disk: Vec<DiskMetrics>,
    pub network: Vec<NetworkMetrics>,
    pub events: Vec<SystemEvent>,
    pub _flush_interval: u64,
}

impl MetricsBuffer {
    pub fn new(flush_interval: u64) -> Self {
        Self {
            cpu: Vec::new(),
            memory: Vec::new(),
            disk: Vec::new(),
            network: Vec::new(),
            events: Vec::new(),
            _flush_interval: flush_interval,
        }
    }
    pub fn append_tick(
        &mut self,
        cpu: CpuMetrics,
        memory: MemoryMetrics,
        disks: Vec<DiskMetrics>,
        networks: Vec<NetworkMetrics>,
    ){
        self.cpu.push(cpu);
        self.memory.push(memory);
        self.disk.extend(disks);
        self.network.extend(networks);
    }
    pub fn append_event(&mut self, events: Vec<SystemEvent>){
        self.events.extend(events);
    }

    pub fn flush(&self)
    {
        // YEETTT TO THE DATABASE
    }

    pub fn clear(&mut self)
    {
        self.cpu.clear();
        self.memory.clear();
        self.disk.clear();
        self.network.clear();
        self.events.clear();
    }
}