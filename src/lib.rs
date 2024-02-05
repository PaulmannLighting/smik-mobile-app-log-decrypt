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
            Self::Ios => {
                let encrypted = ios_log_decrypt::EncryptedLog::new(ciphertext.into());
                let blocks: Vec<anyhow::Result<Vec<u8>>> = encrypted.decrypt(key.into()).collect();
                let mut bytes = Vec::with_capacity(
                    blocks
                        .iter()
                        .map(|result| result.as_ref().map(Vec::len).unwrap_or(0))
                        .sum(),
                );

                for block in blocks {
                    bytes.extend(block?);
                }

                Ok(bytes)
            }
        }
    }
}
