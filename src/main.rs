use clap::Parser;
use clap_stdin::FileOrStdin;
use log::error;
use rpassword::prompt_password;
use smik_mobile_app_log_decrypt::LogType;
use std::io::{stdout, BufWriter, Write};
use std::process::exit;

#[derive(Debug, Parser)]
struct Args {
    #[arg(index = 1, name = "filename", help = "path to the encrypted log file")]
    logfile: FileOrStdin<String>,
    #[arg(long, short, help = "hexadecimal decryption key")]
    key: Option<String>,
    #[command(subcommand, help = "the type of log file to decrypt")]
    log_type: LogType,
}

impl Args {
    pub fn decrypt(self) -> anyhow::Result<Vec<u8>> {
        let key = self.key();
        self.log_type.decrypt(
            &self.logfile.contents().unwrap_or_else(|error| {
                error!("{error}");
                exit(3)
            }),
            &key,
        )
    }

    #[must_use]
    fn hex_key(&self) -> String {
        self.key.clone().unwrap_or_else(|| {
            prompt_password("Decryption key: ").unwrap_or_else(|error| {
                error!("{error}");
                exit(1)
            })
        })
    }

    fn key(&self) -> Vec<u8> {
        hex::decode(self.hex_key()).unwrap_or_else(|error| {
            error!("{error}");
            exit(2);
        })
    }
}

fn main() {
    env_logger::init();

    BufWriter::new(stdout().lock())
        .write_all(&Args::parse().decrypt().unwrap_or_else(|error| {
            error!("{error}");
            exit(4);
        }))
        .unwrap_or_else(|error| {
            error!("{error}");
            exit(5);
        });
}
