// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.


#[derive(Debug, thiserror::Error)]
pub enum Error
{
    #[error(transparent)]
    CreateDefaultFile(#[from] CreateDefaultFileError),

    #[error("Loading config failed with: {0}")]
    Figment(#[from] figment::Error),
}


#[derive(Debug, thiserror::Error)]
pub enum CreateDefaultFileError
{
    #[cfg(feature = "json_file")]
    #[error("Serialising config to JSON failed with: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[cfg(feature = "yaml_file")]
    #[error("Serialising config to YAML failed with: {0}")]
    SerdeYaml(#[from] serde_yaml::Error),

    #[error("Saving default config file failed with: {0}")]
    StdIo(#[from] std::io::Error),

    #[cfg(feature = "toml_file")]
    #[error("Serialising config to TOML failed with: {0}")]
    TomlSer(#[from] toml::ser::Error),
}