// use std::default;

use std::path::PathBuf;


#[derive(clap::Parser,Debug)]
pub struct CLI{
    /// Enable Debug mode. Which will not exit if 
    #[arg(global = true,action = clap::ArgAction::SetTrue)]
    debug:bool,


    /// Uses a specified config file instead of `/etc/chdm.conf`
    #[arg(global = true,
    default_value_os_t = std::path::PathBuf::from("/etc/chdm.conf")
)]
    config:PathBuf
}