#[derive(Debug)]
pub enum AssignmentError {
    OrderNotCreated,
    CourierNotFound,
    CourierUnavailable,
    AltitudeTooHigh,
    FragileCargoNotSupported,
    OrderNotFound,
}

#[derive(Debug)]
pub enum DeliveryError {
    OrderNotFound,
    OrderNotInTransit,
    OrderNotAssigned,
}