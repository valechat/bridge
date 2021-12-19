use std::collections::{HashMap};
use std::sync::{Arc};
use parking_lot::{RwLock, Mutex};
use crate::{EventHandler, Dispatchable, EventResult};

type HandlerCollection<E> = Mutex<Vec<Arc<dyn EventHandler<E>>>>;

#[derive(Default)]
pub struct EventDispatcher<'a, E: Dispatchable> {
    pub handlers: RwLock<HashMap<&'a str, HandlerCollection<E>>>,
}

impl<'a, E: Dispatchable> EventDispatcher<'a, E> {
    pub fn new() -> Self {
        Self {
            handlers: RwLock::new(HashMap::new())
        }
    }

    pub fn add_handler(&self, events: Vec<&'a str>, handler: Arc<dyn EventHandler<E>>) {
        let mut all_handlers = self.handlers.write();

        for event in events {
            if let Some(handlers) = all_handlers.get(event) {
                let handlers = &mut handlers.lock();
                handlers.push(handler.clone());
                handlers.retain(|h| h.is_alive());
            } else {
                all_handlers.insert(event, Mutex::new(vec![handler.clone()]));
            }
        }
    }

    pub fn dispatch(&self, event: &mut E) -> EventResult {
        let all_handlers = self.handlers.read();
        if let Some(handlers) = all_handlers.get(event.get_name()) {
            let handlers = &mut handlers.lock();

            handlers.retain(|handler| handler.is_alive());
            for handler in handlers.iter() {
                handler.handle_event(event)?;
            }
        }

        Ok(())
    }
}