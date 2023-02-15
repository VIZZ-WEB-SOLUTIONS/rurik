use nix::sys::inotify::{AddWatchFlags, InitFlags, Inotify};
use std::fs;
use std::process::Stdio;
use tokio::process::Command;
use tokio::time::{sleep, Duration};
use toml::Table;

struct Cron {
    name: String,
    command: String,
    sleep_time: u64,
}

impl Cron {
    pub fn new(name: String, command: String, sleep_time: u64) -> Self {
        Self {
            name,
            command,
            sleep_time,
        }
    }

    async fn start_cron(&self) {
        loop {
            sleep(Duration::from_millis(self.sleep_time)).await;
            self.execute_command();
        }
    }

    fn execute_command(&self) {
        println!("{} => ", self.name);
        let command = self.command.clone();

        tokio::spawn(async {
            let _ = Command::new("sh")
                .arg("-c")
                .arg(command)
                .stdout(Stdio::inherit())
                .status().await;
        });
    }
}

#[tokio::main]
async fn main() {
    let cron_home: &'static str = env!("MICRO_CRON_HOME", "MICRO_CRON_HOME not set");
    let cron_main: &'static str = env!("MICRO_CRON_MAIN", "MICRO_CRON_MAIN not set");

    tokio::spawn(async move {
        let main_toml = fs::read_to_string(cron_home.to_owned() + "/" + cron_main)
            .expect("Could not read the mian.toml file");

        let parsed_toml = main_toml.parse::<Table>().unwrap();
        let crons = parsed_toml["crons"].as_array().unwrap();
        for cron in crons {
            let name = cron["name"].as_str().unwrap().to_string();
            let command = cron["command"].as_str().unwrap().to_string();
            let sleep_time = cron["sleep"].as_integer().unwrap() as u64;

            let x = Cron::new(name, command, sleep_time);

            tokio::spawn(async move {
                x.start_cron().await;
            });
        }
    });

    let instace = Inotify::init(InitFlags::empty()).unwrap();

    let _wd = instace
        .add_watch(cron_home, AddWatchFlags::IN_CLOSE_WRITE)
        .unwrap();

    loop {
        let events = instace.read_events().unwrap();

        let name = match events[0].name.clone() {
            Some(n) => n,
            None => String::new().into(),
        };

        if name == cron_main {
            println!("Name is {:?}", name);
        };
    };
}
