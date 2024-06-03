use std::collections::VecDeque;
use config::*;

#[derive(Debug)]
/// A struct that contains command line parameters,
/// It implements config::Source and can therefore be used with config-rs
struct ComandLineSource {
    args: Map<String, Value>
}

impl ComandLineSource {
    /// Creates a new CommandLineSource from a vector of args
    fn new(mut args: VecDeque<String>) -> Self {
        // Create an empty map to contain the args
        let mut args_map: Map<String, Value> = Map::new();

        // Remove first arg (the executable name)
        args.pop_front();

        for index in 0..args.len() {
            let arg = args.get(index);
            if arg == "--config" {
                index
                args_map.insert(arg, args)
            }
        }

        return ComandLineSource {
            args: args_map
        }
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