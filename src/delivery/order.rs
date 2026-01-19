use crate::traits::{Identifiable, Loggable};

pub enum OrderStatus {
    Created,
    Assigned  { courier_id: u32 },
    InTransit,
    Delivered,
    Failed {reason: String},
}

pub struct Cargo {
    item_type: String,   // "Электроника", "Продукты", "Снаряжение"
    weight_kg: f32,      // вес в килограммах
    is_fragile: bool,    // требует осторожной транспортировки
}

// Заказ на доставку
pub struct DeliveryOrder {
    id: u32,
    customer_name: String,
    destination_altitude: u16, // высота доставки (в метрах)
    status: OrderStatus,
    cargo: Option<Cargo>,
}

impl Identifiable for DeliveryOrder {
    fn id(&self) -> u32 {
        self.id
    }
}

impl Loggable for DeliveryOrder {
    fn log_info(&self) -> String {
        format!("ORDER[{}] {}", self.id(), self.status_description())
    }
}

impl DeliveryOrder {
    // Создаёт новый заказ со статусом `Created`
    pub fn new(id: u32, customer_name: String, destination_altitude: u16) -> Self {
        DeliveryOrder {
            id,
            customer_name,
            destination_altitude,
            status: OrderStatus::Created,
            cargo: None,
        }
    }

    // Назначает курьера: работает ТОЛЬКО если статус == Created
    pub fn assign(&mut self, courier_id: u32) -> bool {
        if matches!(self.status, OrderStatus::Created) {
            self.status = OrderStatus::Assigned{ courier_id };
            true
        } else {
            false
        }
    }

    // Начинает доставку: работает ТОЛЬКО если статус == Assigned
    pub fn start_transit(&mut self) -> bool {
        if matches!(self.status, OrderStatus::Assigned { .. }) {
            self.status = OrderStatus::InTransit;
            true
        } else {
            false
        }
    }

    // Завершает доставку: работает ТОЛЬКО если статус == InTransit
    pub fn complete(&mut self) -> bool {
        if matches!(self.status, OrderStatus::InTransit) {
            self.status = OrderStatus::Delivered;
            true
        } else {
            false
        }
    }

    // НОВЫЙ метод: добавить груз к заказу (можно только в статусе Created)
    pub fn add_cargo(&mut self, cargo: Cargo) -> bool {
        if matches!(self.status, OrderStatus::Created) {
            self.cargo = Some(cargo);
            true
        } else {
            false
        }
    }

    // Проверить, есть ли хрупкий груз
    pub fn has_fragile_cargo(&self) -> bool {
        match &self.cargo {
            Some(cargo) => cargo.is_fragile,
            None => false,
        }
    }

    // Помечает как проваленный (можно вызывать всегда)
    pub fn fail(&mut self, reason: String) {
        self.status = OrderStatus::Failed { reason };
    }

    // Получить ID курьера (если назначен)
    pub fn get_courier_id(&self) -> Option<u32> {
        match &self.status {
        OrderStatus::Assigned { courier_id } => Some(*courier_id),
        _ => None,
    }
    }

    // Напечатать статус красиво
    pub fn status_description(&self) -> String {
        match &self.status {
            OrderStatus::Created => "Создан".to_string(),
            OrderStatus::Assigned { courier_id } => format!("Назначен курьеру {}", courier_id),
            OrderStatus::InTransit => "В пути".to_string(),
            OrderStatus::Delivered => "Доставлен".to_string(),
            OrderStatus::Failed { reason } => format!("Провален: {}", reason),
        }
    }

    pub fn status(&self) -> &OrderStatus {
        &self.status
    }

    pub fn destination_altitude(&self) -> u16 {
        self.destination_altitude
    }
}