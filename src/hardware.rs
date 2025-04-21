use sysinfo::System;
use std::fmt;
use indoc::indoc;

pub struct Hardware {
    pub host: String,
    pub cpu: String,
    pub cpu_cores: usize,
    pub arch: String,
    pub ram: u64,
}

impl fmt::Display for Hardware {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, indoc! {"
            Host: {}
            CPU: {}
            CPU Cores: {}
            CPU Arch: {}
            RAM: {}GB"}, 
               self.host,
               self.cpu,
               self.cpu_cores,
               self.arch,
               self.get_mem_in_gb(),
        )
    }
}

impl Hardware {
    pub async fn new(system_information: &System) -> Result<Hardware, &'static str> {
        let cpu = match system_information.cpus().first() {
            Some(cpu) => cpu.brand().to_string(),
            None => return Err("No processor found"),
        };
        Ok(Hardware {
            host: System::host_name().unwrap_or_else(|| {
                eprintln!("Unable to find system hostname");
                String::from("N/A")
            }),
            cpu,
            cpu_cores: system_information.cpus().len(),
            arch: System::cpu_arch(),
            ram: system_information.total_memory(),
        })
    }
    
    fn get_mem_in_gb(&self) -> u64 {
        self.ram / 1024 / 1024 / 1024
    }
}
