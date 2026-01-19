mod delivery;
mod courier;
mod dispatch;
mod error;
mod traits;
mod utils;

use delivery::{DeliveryOrder};
use courier::Courier;
use dispatch::Dispatch;
use traits::Loggable;

fn main() {
    let mut dispatch = Dispatch::new();
    dispatch.register_courier(Courier::new(1, "Анна".to_string(), 5000, true));
    dispatch.add_order(DeliveryOrder::new(101, "Дима".to_string(), 5200));

    match dispatch.assign_order_to_courier(101, 1) {
        Ok(()) => {
            println!("✅ Заказ назначен!");
            match dispatch.start_transit(101) {
                Ok(()) => {
                    match dispatch.complete_delivery(101) {
                        Ok(()) => println!("📦 Доставка завершена"),
                        Err(e) => println!("❌ Ошибка доставки: {:?}", e),
                    }
                }
                Err(e) => println!("❌ Ошибка начала доставки: {:?}", e),
            }
            match dispatch.complete_delivery(101) {
                Ok(()) => println!("📦 Доставка завершена"),
                Err(e) => println!("❌ Ошибка доставки: {:?}", e),
            }
        }
        Err(e) => {
            println!("❌ Ошибка назначения: {:?}", e);
        }
    }

    println!("\n{}", dispatch.generate_report());

    // Логирование через трейт
    if let Some(order) = dispatch.find_order_mut(101) {
        println!("Лог заказа: {}", order.log_info());
    }
    
    if let Some(courier) = dispatch.find_courier_mut(1) {
        println!("Лог курьера: {}", courier.log_info());
    }
}
