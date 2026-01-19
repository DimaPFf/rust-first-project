pub trait Identifiable {
    fn id(&self) -> u32;
}

pub trait Loggable {
    fn log_info(&self) -> String;
}