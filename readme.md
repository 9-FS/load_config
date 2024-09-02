# load_config
## Introduction

This loader provides a simple way to load configurations from different sources hierachially using `figment` and `serde`. It supports the following sources:
- `T::default()`
- environment variables
- JSON files
- TOML files
- YAML files

In case of conflicting settings, the source order defines the precedence. Earlier sources are preferred over later ones. The loader can also offer to generate a default config file in case a setting could not be loaded.

## Installation

1. Paste the following `Cargo.toml` entry into your `Cargo.toml` beneath `[dependencies]`:
    ```TOML
    load_config = { git = "https://github.com/9-FS/load_config", tag = "", features = []}
    ```
1. Write the desired version number into the `tag` field.
    > [!NOTE]
    > Cargo does not support automatic versioning for GitHub dependencies. Manual updates are required in the `Cargo.toml` file using `tag`.
1. Enable the features for the desired config file formats in the `Cargo.toml` file. The following features are available:
    - `json_file`
    - `toml_file`, can notably also be used for `.env` files
    - `yaml_file`

Example:

```TOML
load_config = { git = "https://github.com/9-FS/load_config", tag = "1.0.0", features = ["toml_file"] }
```

## Usage

1. Define a struct that represents the config containing the desired settings. It must implement `Debug`, `Default`, `serde::Deserialize`, and `serde::Serialize`.

    Template:

    ```Rust
    /// # Summary
    /// Collection of settings making up the configuration of the application.
    #[derive(Debug, serde::Deserialize, serde::Serialize)]
    #[allow(non_snake_case)]
    pub struct Config
    {
        // add settings
    }

    impl Default for Config
    {
        fn default() -> Self
        {
            Config
            {
                // add default setting values
            }
        }
    }
    ```
1. Execute `load_config` with the desired config struct, sources, and optionally a default source file in case a setting could not be loaded and generation of a default config file is chosen.

    Template:

    ```Rust
    let mut config: Config;


    match load_config::load_config
    (
        vec!
        [
            load_config::Source::Env,
            // load_config::Source::File(load_config::SourceFile::Toml("./config/.env".to_string())),
            // load_config::Source::File(load_config::SourceFile::Yaml("./config/config.yaml".to_string())),
            // load_config::Source::File(load_config::SourceFile::Json("./config/config.json".to_string())),
            // load_config::Source::ConfigDefault,
        ],
        None
        // Some(load_config::SourceFile::Toml("./config/.env".to_string()))
    )
    {
        Ok(o) => {config = o;} // loaded config successfully
        Err(_) => {return std::process::ExitCode::FAILURE;} // loading config failed
    }
    ```

    > [!NOTE]
    > Enabling `load_config::Source::ConfigDefault` fills up missing settings with default values, so creating a default config file is never triggered.

### Example

`./Cargo.toml`

```TOML
load_config = { git = "https://github.com/9-FS/load_config", tag = "1.0.0", features = ["yaml_file"] }
```

`./src/config.rs`

```Rust
/// # Summary
/// Collection of settings making up the configuration of the application.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[allow(non_snake_case)]
pub struct Config
{
    setting1: String,
    setting2: i32,
}

impl Default for Config
{
    fn default() -> Self
    {
        Config
        {
            setting1: "amogusඞ".to_string(),
            setting2: 42069,
        }
    }
}
```

`./src/main.rs`

```Rust
let mut config: Config;


match load_config::load_config
(
    vec!
    [
        load_config::Source::Env,
        load_config::Source::File(load_config::SourceFile::Yaml("./config/config.yaml".to_string())),
    ],
    Some(load_config::SourceFile::Yaml("./config/config.yaml".to_string()))
)
{
    Ok(o) => {config = o;} // loaded config successfully
    Err(_) => {return std::process::ExitCode::FAILURE;} // loading config failed
}
```