use config::*;

#[derive(Debug)]
/// A struct that contains command line parameters,
/// It implements config::Source and can therefore be used with config-rs
struct ComandLineSource {
    args: Vec<String>,
    args_hashmap: Map<String, Value>
}

impl Source for ComandLineSource {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        return Box::new(self);
    }

    fn collect(&self) -> Result<Map<String, Value>, ConfigError> {
        todo!()
    }

    fn collect_to(&self, cache: &mut Value) -> Result<(), ConfigError> {
        todo!()
    }
}