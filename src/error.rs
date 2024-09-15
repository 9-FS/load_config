// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.


#[derive(Debug, thiserror::Error)]
pub enum Error
{
    #[error(transparent)]
    CreateDefaultFile(#[from] CreateDefaultFileError), // loading config failed, creating default file failed

    #[error("Created default config file at \"{filepath}\".")]
    CreatedDefaultFile {filepath: String}, // loading config failed, default file created successfully

    #[error("Loading config failed with: {0}")]
    Figment(#[from] figment::Error), // loading config failed, nothing else could be done
}


#[derive(Debug, thiserror::Error)]
pub enum CreateDefaultFileError
{
    #[cfg(feature = "json_file")]
    #[error("Loading config failed. Serialising default config to JSON failed with: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[cfg(feature = "yaml_file")]
    #[error("Loading config failed. Serialising default config to YAML failed with: {0}")]
    SerdeYaml(#[from] serde_yaml::Error),

    #[error("Loading config failed. Saving default config file at \"{filepath}\" failed with: {source}")]
    StdIo {filepath: String, source: std::io::Error},

    #[cfg(feature = "toml_file")]
    #[error("Loading config failed. Serialising default config to TOML failed with: {0}")]
    TomlSer(#[from] toml::ser::Error),
}