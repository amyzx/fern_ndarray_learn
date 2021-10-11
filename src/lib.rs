pub mod file_util;
 
use std::fs;
use std::path::Path;

// setup log file
pub fn setup_logger(log_file_name: &str) -> Result<(), fern::InitError> {   
    if Path::new(log_file_name).exists() {
        fs::remove_file(log_file_name)?;
    } 

    println!("log file : {}", log_file_name);
    fern::Dispatch::new()
        .format(|out, message, _record| {
            out.finish(format_args!(
                "{}", 
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file(log_file_name)?)
        .apply()?;
    Ok(())
}
 