use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeConfigError {
    #[error("convertion from toml to string failed")]
    TomlToStringError(#[from] toml::ser::Error),

    #[error("write to file {1} failed: {0}")]
    WriteToFileFailed(#[source] std::io::Error, String),

    #[error("read from file {1} failed: {0}")]
    ReadFromFileFailed(#[source] std::io::Error, String),

    #[error("convertion from string to toml failed")]
    StringToTomlError(#[from] toml::de::Error)
}