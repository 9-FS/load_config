// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
mod error;
pub use crate::error::*;
#[allow(unused_imports)]
use figment::providers::Format;
#[allow(unused_imports)]
use std::io::Write;


/// # Summary
/// Loads config from `sources`, preferring earlier sources. If `config_file_default` is `Some`, a setting is unset, and the specified filepath does not exist yet, offers to create a default config file there. Returns loaded config of type `T` or an error.
///
/// # Arguments
/// - `T`: type of config to load, tries to populate its fields from sources
/// - `sources`: sources to load config from, prefers earlier sources
/// - `config_file_default`: default config file to create if a setting is unset, optional
///
/// # Returns
/// - successfully loaded config of type `T` or an error
///
/// # Example
/// ```
/// // create test file at test_filepath with TEST_CONTENT to test loading from file
/// const TEST_CONTENT: &str = "setting1 = true\nsetting2 = 42069";
/// let test_filepath: &std::path::Path = std::path::Path::new("./test/config.toml");
/// std::fs::create_dir_all(test_filepath.parent().unwrap()).expect(format!("Creating \"{:?}\" failed.", test_filepath.parent().unwrap()).as_str());
/// std::fs::write(test_filepath, TEST_CONTENT).expect(format!("Writing to \"{test_filepath:?}\" failed.").as_str());
///
///
/// // `./src/config.rs`
/// // collection of settings making up the configuration of the application
/// #[derive(PartialEq)] // only necessary for testing
/// #[derive(Debug, serde::Deserialize, serde::Serialize)]
/// #[allow(non_snake_case)]
/// pub struct Config
/// {
///     pub setting1: bool,
///     pub setting2: i32,
///     pub setting3: String,
/// }
///
/// impl Default for Config
/// {
///     fn default() -> Self
///     {
///         Config
///         {
///             setting1: false,
///             setting2: 0,
///             setting3: "amogusඞ".to_string(),
///         }
///     }
/// }
///
///
/// // `./src/main.rs`
/// // load config from file
/// let config: Config;
/// match load_config::load_config
/// (
///     vec!
///     [
///         load_config::Source::Env,
///         load_config::Source::File(load_config::SourceFile::Toml(test_filepath.to_str().unwrap().to_owned())),
///         load_config::Source::ConfigDefault,
///     ],
///     None,
/// )
/// {
///     Ok(o) => {config = o;} // loaded config successfully
///     Err(_) => {panic!("Loading config failed.")} // loading config failed
/// }
///
///
/// assert_eq!(config, Config{setting1: true, setting2: 42069, setting3: "amogusඞ".to_string()}); // test correctness
///
/// std::fs::remove_dir_all(test_filepath.parent().unwrap()).expect(format!("Removing {test_filepath:?} failed.").as_str()); // cleanup
/// ```
#[allow(unused_variables)]
pub fn load_config<'a, T>(sources: Vec<Source>, config_file_default: Option<SourceFile>) -> Result<T, Error>
where
    T: std::fmt::Debug + Default + serde::Deserialize<'a> + serde::Serialize,
{
    let config: T;
    let mut fig: figment::Figment = figment::Figment::new();


    for source in sources // load all sources, prefer earlier sources
    {
        match source
        {
            Source::ConfigDefault => fig = fig.join(figment::providers::Serialized::defaults(T::default())),
            Source::Env => fig = fig.join(figment::providers::Env::raw().lowercase(false)), // don't lowercase env variable names, keep unchanged
            #[cfg(feature = "config_file")]
            Source::File(source_file) => match source_file
            {
                #[cfg(feature = "json_file")]
                SourceFile::Json(filepath) => fig = fig.join(figment::providers::Json::file(filepath)),
                #[cfg(feature = "toml_file")]
                SourceFile::Toml(filepath) => fig = fig.join(figment::providers::Toml::file(filepath)),
                #[cfg(feature = "yaml_file")]
                SourceFile::Yaml(filepath) => fig = fig.join(figment::providers::Yaml::file(filepath)),
            },
        };
    }

    match fig.extract() // Figment -> T
    {
        Ok(c) => config = c, // loaded config successfully

        Err(e) => // loading config failed
        {
            #[cfg(feature = "config_file")]
            if let figment::error::Kind::MissingField(_) = e.kind // if setting unset
            {
                if let Some(s) = config_file_default // and default config file specified
                {
                    let filepath: String = match s.clone() // extract filepath where default config should be created
                    {
                        #[cfg(feature = "json_file")]
                        SourceFile::Json(filepath) => filepath,
                        #[cfg(feature = "toml_file")]
                        SourceFile::Toml(filepath) => filepath,
                        #[cfg(feature = "yaml_file")]
                        SourceFile::Yaml(filepath) => filepath,
                    };
                    if !std::path::Path::new(&filepath).exists() // and if file does not already exist, don't want to overwrite existing but faulty config file, rather give missing field error to user
                    {
                        create_default_file::<T>(&s)?; // create default config file, upon failure propagate this error over the missing field error
                        return Err(Error::CreatedDefaultFile {filepath}); // created default config file successfully
                    }
                }
            }
            return Err(e.into()); // if not because of missing field: just forward figment error
        }
    }

    return Ok(config);
}


/// # Summary
/// Creates a default config file with `config_file_default`'s format at its contained filepath, if it does not exist yet.
///
/// # Arguments
/// - `T`: type of default config to create with `T::default()` determining the content
/// - `config_file_default`: default config file format and path to create if a setting is unset
#[cfg(feature = "config_file")]
fn create_default_file<'a, T>(config_file_default: &SourceFile) -> Result<(), CreateDefaultFileError>
where
    T: std::fmt::Debug + Default + serde::Deserialize<'a> + serde::Serialize,
{
    let mut file: std::fs::File; // file to write to
    let file_content: String; // config serialised to write to file
    let filepath: &str; // path to file to be created


    filepath = match config_file_default // extract filepath
    {
        #[cfg(feature = "json_file")]
        SourceFile::Json(filepath) => filepath,
        #[cfg(feature = "toml_file")]
        SourceFile::Toml(filepath) => filepath,
        #[cfg(feature = "yaml_file")]
        SourceFile::Yaml(filepath) => filepath,
    };


    file_content = match config_file_default
    {
        #[cfg(feature = "json_file")]
        SourceFile::Json(_) => serde_json::to_string_pretty(&T::default())?, // serialise config to json
        #[cfg(feature = "toml_file")]
        SourceFile::Toml(_) => toml::to_string_pretty(&T::default())?, // serialise config to toml
        #[cfg(feature = "yaml_file")]
        SourceFile::Yaml(_) => serde_yaml::to_string(&T::default())?, // serialise config to yaml
    };

    if let Err(e) = std::fs::create_dir_all(std::path::Path::new(filepath).parent().unwrap_or(std::path::Path::new(""))) // create all parent directories
    {
        return Err(CreateDefaultFileError::StdIo {filepath: filepath.to_owned(), source: e}); // creating parent directories failed
    }
    match std::fs::File::create_new(filepath) // create new file, fails if already exists, don't want to overwrite anything
    {
        Ok(o) => file = o, // created file successfully
        Err(e) => return Err(CreateDefaultFileError::StdIo {filepath: filepath.to_owned(), source: e}), // creating file failed
    }
    if let Err(e) = file.write_all(file_content.as_bytes()) // write serialised default config to file
    {
        return Err(CreateDefaultFileError::StdIo {filepath: filepath.to_owned(), source: e}); // writing to file failed
    }

    return Ok(());
}


/// # Summary
/// Config source. Either environment variables, a file or config default.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Source // could not use list of trait objects (Vec<Box<dyn figment::Provider>>) because figment::merge() requires a type known at compile time
{
    ConfigDefault,
    Env,
    #[cfg(feature = "config_file")]
    File(SourceFile),
}


/// # Summary
/// Supported config file source formats, contain filepath.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SourceFile
{
    #[cfg(feature = "json_file")]
    Json(String),
    #[cfg(feature = "toml_file")]
    Toml(String),
    #[cfg(feature = "yaml_file")]
    Yaml(String),
}
