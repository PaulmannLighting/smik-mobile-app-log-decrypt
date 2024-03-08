use anyhow::anyhow;
use std::io::{stdout, BufWriter, Write};

use clap::Parser;
use clap_stdin::FileOrStdin;
use rpassword::prompt_password;
use smik_mobile_app_log_decrypt::Platform;

#[derive(Debug, Parser)]
struct Args {
    #[arg(index = 1, name = "filename", help = "path to the encrypted log file")]
    logfile: FileOrStdin<String>,
    #[arg(long, short, help = "hexadecimal decryption key")]
    key: Option<String>,
    #[command(
        subcommand,
        name = "platform",
        help = "the type of log file to decrypt"
    )]
    log_type: Platform,
}

impl Args {
    pub fn decrypt(self) -> anyhow::Result<Vec<u8>> {
        let key = self.key()?;
        self.log_type.decrypt(&self.logfile.contents()?, &key)
    }

    #[must_use]
    fn hex_key(&self) -> Option<String> {
        self.key
            .clone()
            .or_else(|| prompt_password("Decryption key: ").ok())
    }

    fn key(&self) -> anyhow::Result<Vec<u8>> {
        Ok(hex::decode(
            self.hex_key().ok_or_else(|| anyhow!("No key provided"))?,
        )?)
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    Ok(BufWriter::new(stdout().lock()).write_all(&Args::parse().decrypt()?)?)
}
