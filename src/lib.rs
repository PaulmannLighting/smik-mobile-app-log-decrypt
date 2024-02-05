use clap::Subcommand;

#[derive(Clone, Debug, Eq, PartialEq, Subcommand)]
#[command(
    author,
    version,
    about,
    long_about = "Mobile platform",
    subcommand_help_heading = "Platforms",
    subcommand_value_name = "platform"
)]
pub enum Platform {
    Android,
    Ios,
}

impl Platform {
    /// Decrypts the respective log file.
    ///
    /// # Errors
    /// Returns an [`anyhow::Error`] if decryption fails.
    pub fn decrypt(&self, ciphertext: &str, key: &[u8]) -> anyhow::Result<Vec<u8>> {
        match self {
            Self::Android => android_log_decrypt::decrypt(ciphertext, key),
            Self::Ios => ios_log_decrypt::decrypt(ciphertext, key),
        }
    }
}
