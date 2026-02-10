use std::sync::Arc;

pub type ProgressCallback = Arc<dyn Fn(&str) + Send + Sync>;
