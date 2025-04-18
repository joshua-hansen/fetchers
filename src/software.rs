use std::fmt;
use sysinfo::System;
use indoc::indoc;

pub struct Software {
    pub os: String,
    pub kernel: String,
}

impl fmt::Display for Software {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, indoc! {"
            OS: {}
            Kernel: {}"},
               self.os,
               self.kernel
        )
    }
}

impl Software {
    pub fn new() -> Self {
        let os = System::long_os_version().unwrap_or_else(|| {
            eprintln!("Error: Unable to find OS version");
            String::from("N/A")
        });
        
        let kernel = System::kernel_version().unwrap_or_else(|| {
            eprintln!("Error: Unable to find kernel version");
            String::from("N/A")
        });
        
        Self {
            os,
            kernel,
        }
    }
}
