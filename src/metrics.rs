use std::time::Instant;
use sysinfo::{Disks, Networks, ProcessesToUpdate, System};

#[derive(Debug, Clone, Default)]
pub struct Snapshot {
    pub os: OsInfo,
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
    pub processes: Vec<ProcessInfo>,
    pub disks: Vec<DiskInfo>,
    pub networks: Vec<NetInfo>,
}

#[derive(Debug, Clone, Default)]
pub struct OsInfo {
    pub name: String,
    pub version: String,
    pub kernel: String,
    pub host: String,
    pub arch: String,
    pub uptime_secs: u64,
}

#[derive(Debug, Clone, Default)]
pub struct CpuInfo {
    pub brand: String,
    pub usage: f32,
    pub cores: Vec<CoreInfo>,
}

#[derive(Debug, Clone)]
pub struct CoreInfo {
    pub name: String,
    pub usage: f32,
    pub frequency_mhz: u64,
}

#[derive(Debug, Clone, Default)]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub swap_total: u64,
    pub swap_used: u64,
}

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu: f32,
    pub memory: u64,
}

#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub name: String,
    pub mount: String,
    pub fs: String,
    pub total: u64,
    pub available: u64,
}

#[derive(Debug, Clone)]
pub struct NetInfo {
    pub name: String,
    pub rx_per_sec: f64,
    pub tx_per_sec: f64,
    pub total_rx: u64,
    pub total_tx: u64,
}

pub struct Collector {
    sys: System,
    disks: Disks,
    networks: Networks,
    last: Instant,
}

impl Collector {
    pub fn new() -> Self {
        Self {
            sys: System::new_all(),
            disks: Disks::new_with_refreshed_list(),
            networks: Networks::new_with_refreshed_list(),
            last: Instant::now(),
        }
    }

    pub fn collect(&mut self) -> Snapshot {
        let elapsed = self.last.elapsed().as_secs_f64().max(0.001);
        self.last = Instant::now();

        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();
        self.sys.refresh_processes(ProcessesToUpdate::All, true);
        self.disks.refresh(true);
        self.networks.refresh(true);

        let cpus = self.sys.cpus();

        let mut disks: Vec<DiskInfo> = self.disks.iter()
            .map(|disk| DiskInfo {
                name: disk.name().to_string_lossy().into_owned(),
                mount: disk.mount_point().display().to_string(),
                fs: disk.file_system().to_string_lossy().into_owned(),
                total: disk.total_space(),
                available: disk.available_space(),
            })
            .collect();
        disks.sort_by(|a,b| a.mount.cmp(&b.mount));

        let mut networks: Vec<NetInfo> = self
            .networks
            .list()
            .iter()
            .map(|(name, data)| NetInfo {
                name: name.clone(),
                rx_per_sec: data.received() as f64 / elapsed,
                tx_per_sec: data.transmitted() as f64 / elapsed,
                total_rx: data.total_received(),
                total_tx: data.total_transmitted(),
            })
            .collect();
        networks.sort_by(|a, b| a.name.cmp(&b.name));

        Snapshot {
            os: OsInfo {
                name: System::name().unwrap_or_default(),
                version: System::os_version().unwrap_or_default(),
                kernel: System::kernel_version().unwrap_or_default(),
                host: System::host_name().unwrap_or_default(),
                arch: System::cpu_arch(),
                uptime_secs: System::uptime(),
            },
            cpu: CpuInfo {
                brand: cpus.first().map(|c| c.brand().to_owned()).unwrap_or_default(),
                usage: self.sys.global_cpu_usage(),
                cores: cpus
                    .iter()
                    .map(|c| CoreInfo {
                        name: c.name().to_owned(),
                        usage: c.cpu_usage(),
                        frequency_mhz: c.frequency(),
                    })
                    .collect(),
            },
            memory: MemoryInfo {
                total: self.sys.total_memory(),
                used: self.sys.used_memory(),
                available: self.sys.available_memory(),
                swap_total: self.sys.total_swap(),
                swap_used: self.sys.used_swap(),
            },
            processes: self
                .sys
                .processes()
                .values()
                .map(|p| ProcessInfo {
                    pid: p.pid().as_u32(),
                    name: p.name().to_string_lossy().into_owned(),
                    cpu: p.cpu_usage(),
                    memory: p.memory(),
                })
                .collect(),
            disks,
            networks,
        }
    }
}
