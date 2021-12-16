use crate::Event;

pub trait EventHandler<E> {
    fn handle_event(&self, event: &Event<E>);
}