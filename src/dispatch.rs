use std::collections::HashMap;
use crate::delivery::DeliveryOrder;
use crate::delivery::OrderStatus;
use crate::courier::Courier;
use crate::error::{AssignmentError, DeliveryError};

pub struct Dispatch {
    orders: Vec<DeliveryOrder>,
    couriers: HashMap<u32, Courier>,
}

impl Dispatch {
    pub fn new() -> Self {
        Dispatch {
            orders: vec!(),
            couriers: HashMap::new(),
        }
    }

    // Добавить новый заказ
    pub fn add_order(&mut self, order: DeliveryOrder) {
        self.orders.push(order);
    }

    // Зарегистрировать нового курьера
    pub fn register_courier(&mut self, courier: Courier) {
        self.couriers.insert(courier.id, courier);
    }

    // Найти заказ по ID (возвращает Option<&mut DeliveryOrder>)
    pub fn find_order_mut(&mut self, order_id: u32) -> Option<&mut DeliveryOrder> {
        for order in self.orders.iter_mut() {
            if order.get_order_id() == order_id {
                return Some(order);
            }
        }
        None
    }

    // Найти курьера по ID (возвращает Option<&mut Courier>)
    pub fn find_courier_mut(&mut self, courier_id: u32) -> Option<&mut Courier> {
        self.couriers.get_mut(&courier_id)
    }

    pub fn assign_order_to_courier(
        &mut self,
        order_id: u32,
        courier_id: u32,
    ) -> Result<(), AssignmentError> {
        let order_idx = self.orders.iter().position(|o| o.get_order_id() == order_id)
            .ok_or(AssignmentError::OrderNotFound)?; 

        // Проверка статуса
        if !matches!(self.orders[order_idx].status(), OrderStatus::Created) {
            return Err(AssignmentError::OrderNotCreated);
        }

        let courier = self.couriers.get_mut(&courier_id)
            .ok_or(AssignmentError::CourierNotFound)?;

        let has_fragile = self.orders[order_idx].has_fragile_cargo();
        
        if !courier.get_is_available() {
            return Err(AssignmentError::CourierUnavailable);
        }
        
        if self.orders[order_idx].destination_altitude() > courier.get_max_altitude() {
            return Err(AssignmentError::AltitudeTooHigh);
        }
        
        if has_fragile && !courier.get_can_handle_fragile() {
            return Err(AssignmentError::FragileCargoNotSupported);
        }

        // Выполняем назначение
        let order = &mut self.orders[order_idx];
        order.assign(courier.id);
        courier.assign_to_order(order.destination_altitude());
        Ok(())
    }

    pub fn start_transit(&mut self, order_id: u32) -> Result<(), DeliveryError> {
    if let Some(order) = self.find_order_mut(order_id) {
        if order.start_transit() {
            Ok(())
        } else {
            Err(DeliveryError::OrderNotAssigned)
        }
    } else {
        Err(DeliveryError::OrderNotFound)
    }
}

    // Завершает доставку заказа
    pub fn complete_delivery(&mut self, order_id: u32) -> Result<(), DeliveryError> {
        if let Some(order) = self.find_order_mut(order_id) {
            if order.complete() {
                Ok(())
            } else {
                Err(DeliveryError::OrderNotInTransit)
            }
        } else {
            Err(DeliveryError::OrderNotFound)
        }
    }

    // Сгенерировать отчёт по всем заказам
    pub fn generate_report(&self) -> String {
        let mut result = String::new();
        for order in &self.orders {
            let line = format!("Заказ {}: {}\n", order.get_order_id(), order.status_description());
            result.push_str(&line);
        }
        result
    }
}