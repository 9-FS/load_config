// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
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
#[allow(unused_variables)]
pub fn load_config<'a, T>(sources: Vec<Source>, config_file_default: Option<SourceFile>) -> Result<T, figment::Error>
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
            Source::Env => fig = fig.join(figment::providers::Env::raw()),
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
        Ok(c) => // loaded config successfully
        {
            config = c;
            log::debug!("Loaded {config:?}.");
        }
        Err(e) => // loading config failed
        {
            log::error!("Loading config failed with: {e}");

            #[cfg(feature = "config_file")]
            if let figment::error::Kind::MissingField(_) = e.kind // if setting unset
            {
                if let Some(s) = config_file_default // and default config file specified
                {
                    offer_default_file_creation::<T>(s); //offer to create default config file
                }
            }

            return Err(e);
        }
    }

    return Ok(config);
}


/// # Summary
/// Offers to create a default config file with `config_file_default`'s format and its contained filepath if it does not exist yet. If accepted, writes default config of type `T` to file.
///
/// # Arguments
/// - `config_file_default`: default config file format and path to create if a setting is unset
#[cfg(feature = "config_file")]
fn offer_default_file_creation<'a, T>(config_file_default: SourceFile) -> ()
where
    T: std::fmt::Debug + Default + serde::Deserialize<'a> + serde::Serialize,
{
    let mut file: std::fs::File; // file to write to
    let file_content: String; // config serialised to write to file
    let filepath: String; // path to file to be created


    filepath = match &config_file_default // extract filepath
    {
        #[cfg(feature = "json_file")]
        SourceFile::Json(filepath) => filepath.clone(),
        #[cfg(feature = "toml_file")]
        SourceFile::Toml(filepath) => filepath.clone(),
        #[cfg(feature = "yaml_file")]
        SourceFile::Yaml(filepath) => filepath.clone(),
    };
    if std::path::Path::new(filepath.as_str()).exists() {return;} // if file already exists: don't want to overwrite anything


    loop
    {
        log::info!("Would you like to create a default config file at \"{filepath}\"? (y/n)");
        let mut input: String = String::new();
        _ = std::io::stdin().read_line(&mut input); // read input
        match input.trim()
        {
            "y" => break, // offer accepted, create default config file
            "n" => return, // offer denied, don't do anything
            _ => {} // input invalid, ask again
        }
    }

    match config_file_default
    {
        #[cfg(feature = "json_file")]
        SourceFile::Json(_) =>
        {
            match serde_json::to_string_pretty(&T::default()) // serialise config to json
            {
                Ok(o) => {file_content = o;}
                Err(e) =>
                {
                    log::error!("Serialising \"{:?}\" to json failed with: {}", &T::default(), e);
                    return;
                }
            };
        }
        #[cfg(feature = "toml_file")]
        SourceFile::Toml(_) =>
        {
            match toml::to_string_pretty(&T::default()) // serialise config to toml
            {
                Ok(o) => {file_content = o;}
                Err(e) =>
                {
                    log::error!("Serialising \"{:?}\" to toml failed with: {}", &T::default(), e);
                    return;
                }
            };
        }
        #[cfg(feature = "yaml_file")]
        SourceFile::Yaml(_) =>
        {
            match serde_yaml::to_string(&T::default()) // serialise config to yaml
            {
                Ok(o) => {file_content = o;}
                Err(e) =>
                {
                    log::error!("Serialising \"{:?}\" to yaml failed with: {}", &T::default(), e);
                    return;
                }
            };
        }
    };

    match std::fs::File::create_new(filepath.clone()) // create new file, fails if already exists, don't want to overwrite anything
    {
        Ok(o) => {file = o;}
        Err(e) =>
        {
            log::error!("Creating default config file at \"{filepath}\" failed with: {e}");
            return;
        }
    };

    match file.write_all(file_content.as_bytes()) // write string to file
    {
        Ok(_) => log::info!("Created default config file at \"{filepath}\"."),
        Err(e) => log::error!("Writing config to \"{filepath}\" failed with: {e}"),
    };

    return;
}


/// # Summary
/// Config source. Either environment variables, a file or config default.
pub enum Source // could not use list of trait objects (Vec<Box<dyn figment::Provider>>) because figment::merge() requires a type known at compile time
{
    ConfigDefault,
    Env,
    #[cfg(feature = "config_file")]
    File(SourceFile),
}


/// # Summary
/// Supported config file source formats, contain filepath.
pub enum SourceFile
{
    #[cfg(feature = "json_file")]
    Json(String),
    #[cfg(feature = "toml_file")]
    Toml(String),
    #[cfg(feature = "yaml_file")]
    Yaml(String),
}
