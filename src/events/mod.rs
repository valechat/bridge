pub mod handler;
pub mod event;
pub mod dispatcher;

pub use handler::*;
pub use event::*;

/*
The event system is intentionally fully synchronous.
This way we can avoid excessive cloning and async locking.
Many event handlers don't need to be asynchronous so it's not worth the overhead.
Event Handlers that need to be async can spawn an async task and clone the necessary event values.
*/