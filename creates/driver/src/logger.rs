pub use tracing::{debug, error, info, warn};
use tracing_subscriber;

pub fn setup() {
    tracing_subscriber::fmt::init();
}
