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
    pub brand: String,
    pub vendor: String,
    pub physical_cores: usize,
    pub logical_cores: usize,
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
    pub event_id: i64;
    pub level: String;
    pub source: String;
    pub message: String;
    pub timestamp: DateTime<Utc>;
}

// Memory buffer

pub struct MetricsBuffer {
    pub cpu: Vec<CpuMetrics>,
    pub memory: Vec<MemoryMetrics>,
    pub disk: Vec<DiskMetrics>,
    pub network: Vec<NetworkMetrics>,
    pub events: Vec<SystemEvent>,
    pub flush_interval: u64,
}