use clap::Subcommand;

#[derive(Clone, Debug, Eq, PartialEq, Subcommand)]
pub enum LogType {
    Android,
    Ios,
}

impl LogType {
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
