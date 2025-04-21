use crate::hardware::Hardware;
use crate::software::Software;
use sysinfo::System;
use indoc::indoc;
use std::process;
use std::fmt;
use std::fmt::Formatter;

pub struct SystemInformation {
    pub hardware: Hardware,
    pub software: Software,
}

impl fmt::Display for SystemInformation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, indoc! {"
            Hardware:
            {}
            \nSoftware:
            {}"},
               self.hardware,
               self.software,
        )
    }
}

impl SystemInformation {
    #[tokio::main]
    pub async fn new() -> Result<SystemInformation, &'static str>{
        let system_info = System::new_all();
        let hardware = Hardware::new(&system_info).await.unwrap_or_else(|err| {
            eprint!("Application Error: {}", err);
            process::exit(1)
        });

        let software = Software::new().await;

        Ok(SystemInformation {
            hardware,
            software,
        })
    }
}
