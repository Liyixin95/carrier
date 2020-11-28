use mio_study::config::RawConfig;
use mio_study::config::saint_config::Config;

fn main() -> Result<(), String>{
    let raw_config = RawConfig::from_args();
    let config: Result<Config, String> = raw_config.into();
    let config = config?;
    Ok(())

}
