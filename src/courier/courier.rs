use crate::traits::{Identifiable, Loggable};

pub struct Courier {
    pub id: u32,
    name: String,
    current_altitude: u16, // где сейчас находится (0 = базовый лагерь)
    max_altitude: u16,     // максимальная высота, на которую может подняться
    is_available: bool,    // свободен ли для нового заказа
    can_handle_fragile: bool,
}

impl Identifiable for Courier {
    fn id(&self) -> u32 {
        self.id
    }
}

impl Loggable for Courier {
    fn log_info(&self) -> String {
        format!(
            "COURIER[{}] {} (max_alt: {}, available: {})",
            self.id(),
            self.get_name(),
            self.get_max_altitude(),
            self.get_is_available()
        )
    }
}

impl Courier {
    // Создаёт курьера: начинает с базового лагеря (0 м), доступен
    pub fn new(id: u32, name: String, max_altitude: u16, can_handle_fragile: bool) -> Self {
        Courier { id, name, current_altitude: 0, max_altitude, is_available: true, can_handle_fragile }
    }

    pub fn can_deliver_to(&self, altitude: u16, order_has_fragile: bool) -> bool {
        self.is_available 
            && altitude <= self.max_altitude 
            && (!order_has_fragile || self.can_handle_fragile)
    }

    // Назначает курьера на заказ (меняет состояние)
    pub fn assign_to_order(&mut self, order_altitude: u16) -> bool {
        // Мы уже знаем, что can_deliver_to == true
        self.current_altitude = order_altitude;
        self.is_available = false;
        true
    }

    // Завершает доставку: возвращается в базовый лагерь (0 м), становится доступен
    pub fn complete_delivery(&mut self) {
        self.current_altitude = 0;
        self.is_available = true;
    }

    pub fn get_is_available(&self) -> bool {
        self.is_available
    }

    pub fn get_max_altitude(&self) -> u16 {
        self.max_altitude
    }

    pub fn get_can_handle_fragile(&self) -> bool {
        self.can_handle_fragile
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}
