use std::collections::{HashMap};
use std::sync::RwLock;
use crate::{Event, EventHandler};

pub struct EventDispatcher<E> {
    pub handlers: RwLock<HashMap<String, Vec<Box<dyn EventHandler<E>>>>>
}

impl<E> EventDispatcher<E> {
    pub fn add_handler(&self, event: &str, handler: Box<dyn EventHandler<E>>) {
        let mut all_handlers = self.handlers.write().unwrap();
        if let Some(handlers) = all_handlers.get_mut(event) {
            handlers.push(handler);
        } else {
            all_handlers.insert(event.to_string(), vec![handler]);
        }
    }

    pub fn dispatch(&self, event: Event<E>) {
        let all_handlers = self.handlers.read().unwrap();
        if let Some(handlers) = all_handlers.get(event.name) {
            for handler in handlers {
                handler.handle_event(&event)
            }
        }
    }
}