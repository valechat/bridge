use tokio::sync::mpsc;
use async_trait::async_trait;
use crate::{Dispatchable, EventResult};

#[async_trait]
pub trait EventHandler<E>: Send + Sync {
    fn is_alive(&self) -> bool;
    fn handle_event(&self, event: &mut E) -> EventResult;
}

pub struct MpscEventHandler<E: Dispatchable> {
    sender: mpsc::Sender<E>,
}

#[async_trait]
impl<E: Dispatchable> EventHandler<E> for MpscEventHandler<E> {
    fn is_alive(&self) -> bool {
        !self.sender.is_closed() && self.sender.capacity() != 0
    }

    fn handle_event(&self, event: &mut E) -> EventResult {
        let _ = self.sender.try_send(event.clone());
        Ok(())
    }
}

impl<E: Dispatchable> MpscEventHandler<E> {
    pub fn new_pair(buffer_size: usize) -> (Self, mpsc::Receiver<E>) {
        let (sender, receiver) = mpsc::channel(buffer_size);
        (Self { sender }, receiver)
    }
}