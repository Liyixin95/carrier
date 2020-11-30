use crate::config::saint_config::Config;
use std::fs::File;
use std::path::PathBuf;
use std::time::Duration;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "test", about = "about test")]
pub struct RawConfig {
    #[structopt(short = "p")]
    parallelism: u32,

    #[structopt(short = "c")]
    connection: u32,

    #[structopt(short = "t")]
    thread: Option<u32>,

    #[structopt(short = "n")]
    request_count: Option<u64>,

    #[structopt(short = "i", default_value = "1")]
    interval: u32,

    #[structopt(short = "d")]
    duration: Option<String>,

    #[structopt(short = "f", default_value = "./request.toml", parse(from_os_str))]
    command_file: PathBuf,

    host: String,
}

impl Into<Result<Config, String>> for RawConfig {
    fn into(self) -> Result<Config, String> {
        let duration = if let Some(s) = self.duration {
            Some(into_duration(s)?)
        } else {
            None
        };

        Ok(Config {
            parallelism: self.parallelism,
            connection: self.connection,
            thread: self.thread.unwrap_or_else(|| num_cpus::get() as u32),
            request_count: self.request_count,
            interval: self.interval,
            duration,
            host: self.host.parse().unwrap(),
            command_file: File::open(self.command_file).unwrap(),
        })
    }
}

fn into_duration(raw_str: String) -> Result<Duration, String> {
    let chars = raw_str.chars();

    let mut index: usize = 0;
    for c in chars {
        if not_number(c) {
            break;
        }
        index = index + 1;
    }

    if index == 0 {
        return Err("input string must start with number".to_string());
    }

    let second: u64;
    unsafe {
        let number = raw_str
            .get_unchecked(0..index)
            .parse::<u32>()
            .map_err(|err| "parse number error".to_string())?;
        let unit = raw_str.get_unchecked(index..);
        if unit.len() != 1 {
            return Err(format!("error unit : {}", unit));
        }

        let unit = parse_unit(unit.chars().next().unwrap())?;
        second = (number * unit) as u64;
    }

    Ok(Duration::from_secs(second))
}

fn not_number(c: char) -> bool {
    c > '9' || c < '0'
}

fn parse_unit(c: char) -> Result<u32, String> {
    return if c == 'm' {
        Ok(60)
    } else if c == 's' {
        Ok(1)
    } else if c == 'h' {
        Ok(3600)
    } else {
        Err(format!("no such unit: {}", c))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duration_basics() {
        assert_eq!(
            into_duration("12m".to_string()).unwrap(),
            Duration::from_secs(12 * 60)
        );
        assert_eq!(
            into_duration("12s".to_string()).unwrap(),
            Duration::from_secs(12)
        );
        assert_eq!(
            into_duration("12h".to_string()).unwrap(),
            Duration::from_secs(12 * 60 * 60)
        );
    }

    #[test]
    fn duration_error() {
        assert_eq!(
            into_duration("m".to_string()),
            Err("input string must start with number".to_string())
        );
        assert!(into_duration("m11".to_string()).is_err());
        assert!(into_duration("".to_string()).is_err());
        assert!(into_duration(" ".to_string()).is_err());
        assert!(into_duration("m ".to_string()).is_err());
        //TODO 未处理u32溢出的情况
    }

    #[test]
    fn number() {
        assert!(not_number('a'));
        assert!(!not_number('0'));
        assert!(!not_number('9'));
    }
}
