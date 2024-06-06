use crate::model::client_model::{ClientBackupCommand, ClientCommand, ClientSubcommand};
use guardian_backup_domain::model::backup::backup::{Backup, BackupId};
use guardian_backup_domain::model::backup::schedule::Schedule;
use guardian_backup_domain::model::backup::schedule_rule::ScheduleRule;
use guardian_backup_domain::model::device_identifier::DeviceIdentifier;
use guardian_backup_domain::model::duration::Duration;
use guardian_backup_domain::model::timestamp::Timestamp;
use regex::Regex;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

pub trait ClientService {
    type Error: Error;

    async fn handle_command(&mut self, command: ClientCommand) -> Result<(), Self::Error>;
}

pub struct MainClientService {}

impl MainClientService {
    pub fn new() -> Self {
        todo!()
    }
}

impl ClientService for MainClientService {
    type Error = DurationErrors;

    async fn handle_command(&mut self, command: ClientCommand) -> Result<(), Self::Error> {
        match command.subcommand {
            ClientSubcommand::Server { .. } => {
                unimplemented!()
            }
            ClientSubcommand::Backup(inner) => match inner {
                ClientBackupCommand::Auto { .. } => {
                    todo!()
                }
                ClientBackupCommand::Create {
                    backup_root,
                    retention_period,
                    name,
                } => {
                    create_backup(backup_root.unwrap(), retention_period, Box::from(name)).await;
                    Ok(())
                }
                ClientBackupCommand::Restore { .. } => {
                    todo!()
                }
                ClientBackupCommand::List {} => {
                    todo!()
                }
            },
        }
    }
}

async fn create_backup(backup_root: PathBuf, retention_period: Option<String>, name: Box<str>) {
    let mut schedule = Schedule::new(Vec::new());

    let mut ret_period = 2600000;
    match retention_period {
        None => {}
        Some(_) => {
            let seconds_unwrapped = &*retention_period.unwrap();
            if let Ok(duration_seconds) = duration_to_seconds(seconds_unwrapped) {
                ret_period = duration_seconds * 1000
            }
        }
    }

    schedule.add_rule(ScheduleRule::new(
        Duration::Limited {
            milliseconds: ret_period as u64,
        },
        Duration::Infinite,
        Timestamp::now(),
    ));
    Backup::new(
        BackupId::from_str(name.as_ref()).unwrap(),
        DeviceIdentifier::default(),
        schedule,
        Box::from(backup_root),
        Vec::new(),
    );
}

fn duration_to_seconds(input: &str) -> Result<u32, DurationErrors> {
    let input_str = input;
    let regex = Regex::new(r"(\d+d|\d+h|\d+m)").unwrap();
    println!("{}", input_str);

    let mut duration_in_sec = 0;

    if !regex.is_match(input_str) {
        return Err(DurationErrors::NoMatches);
    }

    for timepart_capture in regex.captures_iter(input_str) {
        let time_piece = timepart_capture.get(0).unwrap().as_str();
        println!("{:?}", time_piece);
        let (time_amount_str, unit) = time_piece.split_at(time_piece.len() - 1);
        let time_amount = time_amount_str.parse::<u32>().unwrap();
        match unit {
            "d" => {
                duration_in_sec += 24 * 60 * 60 * time_amount;
            }
            "h" => {
                duration_in_sec += 60 * 60 * time_amount;
            }
            "m" => {
                duration_in_sec += 60 * time_amount;
            }
            _ => {
                panic!("should be unreachable, check duration regex")
            }
        }
    }
    Ok(duration_in_sec)
}

#[derive(Debug)]
pub enum DurationErrors {
    NoMatches,
}

impl Display for DurationErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DurationErrors::NoMatches => write!(f, "No matches in the valid format found!"),
        }
    }
}

impl Error for DurationErrors {}

#[derive(Debug)]
pub enum CreateErrors {
    InvalidRoot,
}

impl Display for CreateErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateErrors::InvalidRoot => write!(f, "The provided root path was invalid"),
        }
    }
}

impl Error for CreateErrors {}
