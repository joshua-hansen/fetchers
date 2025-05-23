mod hardware;
mod software;
mod system_information;

use std::error::Error;
use std::process;


pub fn run() -> Result<(), Box<dyn Error>> {
    // Gather System Info
    let system_information = system_information::SystemInformation::new().unwrap_or_else(|err| {
        eprintln!("Application Error: {}", err);
        process::exit(1);
    });

    // Print System Info
    println!("{}", system_information);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let result = run();
        assert!(result.is_ok());
    }
    
}