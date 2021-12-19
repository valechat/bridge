pub type EventResult = Result<(), String>;

pub trait Dispatchable: Send + Sync + Clone {
    fn get_name(&self) -> &'static str;
}