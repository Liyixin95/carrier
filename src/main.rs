use carrier::config::RawConfig;
use carrier::config::saint_config::Config;
use structopt::StructOpt;

fn main() -> Result<(), String>{
    let raw_config = RawConfig::from_args();
    let config: Result<Config, String> = raw_config.into();
    let config = config?;
    Ok(())

}
