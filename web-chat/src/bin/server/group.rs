use std::sync::Arc;
use tokio::sync::broadcast::Sender;

pub struct Group {
    name: Arc<String>,
    sender: Sender<Arc<String>>
}