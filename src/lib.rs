use clap::{builder::PossibleValue, ValueEnum};

#[derive(Clone, Debug, Eq, PartialEq)]
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
                let mut bytes = Vec::new();
                for block in
                    ios_log_decrypt::EncryptedLog::new(ciphertext.into()).decrypt(key.into())
                {
                    bytes.extend(block?);
                }
                Ok(bytes)
            }
        }
    }
}

impl ValueEnum for LogType {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Android, Self::Ios]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Android => PossibleValue::new("android"),
            Self::Ios => PossibleValue::new("ios"),
        })
    }
}
