use crate::error::VoidBuilderError;
use config::*;
use std::collections::VecDeque;

#[derive(Debug)]
/// A struct that contains command line parameters,
/// It implements config::Source and can therefore be used with config-rs
struct ComandLineSource {
    args: Map<String, Value>,
}

impl ComandLineSource {
    /// Creates a new CommandLineSource from a vector of args
    fn new(mut args: VecDeque<String>) -> Result<Self, VoidBuilderError> {
        // Create an empty map to contain the args
        let mut args_map: Map<String, Value> = Map::new();

        // Remove first arg (the executable name)
        args.pop_front();

        // Check if --config is specified
        match args.iter().position(|x| x == "--config") {
            Some(index) => match args.get(index + 1) {
                Some(arg) => {
                    if args.contains(&"--".to_string()) {
                        return Err(VoidBuilderError::new(
                            "Incorrect parameters: --config specified with no value".to_string(),
                        ));
                    }

                    if !std::path::Path::new(arg).exists() {
                        return Err(VoidBuilderError::new(
                            "Config Error: Config file not found".to_string(),
                        ));
                    }

                    args_map.insert(
                        "config".to_string(),
                        Value::new(None, ValueKind::String(arg.to_string())),
                    );
                }
                None => {
                    return Err(VoidBuilderError::new(
                        "Incorrect parameters: --config specified with no value".to_string(),
                    ))
                }
            },
            None => (),
        }

        return Ok(ComandLineSource { args: args_map });
    }
}

impl Source for ComandLineSource {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        todo!()
    }

    fn collect(&self) -> Result<Map<String, Value>, ConfigError> {
        todo!()
    }

    fn collect_to(&self, cache: &mut Value) -> Result<(), ConfigError> {
        todo!()
    }
}
