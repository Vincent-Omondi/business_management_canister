use candid::{CandidType, Deserialize, Encode};
use ic_cdk::api::time;
use ic_cdk_macros::{query, update};
use serde::Serialize;
use std::collections::HashMap;
use std::cell::RefCell;

type ItemId = u64;

type Memory<T> = RefCell<HashMap<ItemId, T>>;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
struct InventoryItem {
    id: ItemId,
    name: String,
    quantity: u64,
    price: f64,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
struct SaleRecord {
    timestamp: u64,
    items: Vec<SaleItem>,
    total_amount: f64,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
struct SaleItem {
    id: ItemId,
    name: String,
    quantity: u64,
    unit_price: f64,
}

thread_local! {
    static INVENTORY: Memory<InventoryItem> = RefCell::new(HashMap::new());
    static SALES: RefCell<Vec<SaleRecord>> = RefCell::new(Vec::new());
    static NEXT_ITEM_ID: RefCell<ItemId> = RefCell::new(1);
}

#[update]
fn add_item(name: String, quantity: u64, price: f64) -> ItemId {
    NEXT_ITEM_ID.with(|next_id| {
        INVENTORY.with(|inventory| {
            let id = *next_id.borrow();
            inventory.borrow_mut().insert(
                id,
                InventoryItem {
                    id,
                    name,
                    quantity,
                    price,
                },
            );
            *next_id.borrow_mut() += 1;
            id
        })
    })
}

#[update]
fn update_item(id: ItemId, name: Option<String>, quantity: Option<u64>, price: Option<f64>) -> bool {
    INVENTORY.with(|inventory| {
        let mut inventory = inventory.borrow_mut();
        if let Some(item) = inventory.get_mut(&id) {
            if let Some(new_name) = name {
                item.name = new_name;
            }
            if let Some(new_quantity) = quantity {
                item.quantity = new_quantity;
            }
            if let Some(new_price) = price {
                item.price = new_price;
            }
            true
        } else {
            false
        }
    })
}

#[update]
fn remove_item(id: ItemId) -> bool {
    INVENTORY.with(|inventory| inventory.borrow_mut().remove(&id).is_some())
}

#[update]
fn record_sale(items: Vec<(ItemId, u64)>) -> Result<SaleRecord, String> {
    INVENTORY.with(|inventory| {
        let mut inventory = inventory.borrow_mut();
        let mut sale_items = Vec::new();
        let mut total_amount = 0.0;

        for (item_id, quantity) in items {
            if let Some(item) = inventory.get_mut(&item_id) {
                if item.quantity >= quantity {
                    item.quantity -= quantity;
                    sale_items.push(SaleItem {
                        id: item.id,
                        name: item.name.clone(),
                        quantity,
                        unit_price: item.price,
                    });
                    total_amount += item.price * quantity as f64;
                } else {
                    return Err(format!("Insufficient stock for item: {}", item.name));
                }
            } else {
                return Err(format!("Item with ID {} not found", item_id));
            }
        }

        let sale_record = SaleRecord {
            timestamp: time(),
            items: sale_items,
            total_amount,
        };

        SALES.with(|sales| sales.borrow_mut().push(sale_record.clone()));

        Ok(sale_record)
    })
}

#[query]
fn get_inventory() -> Vec<InventoryItem> {
    INVENTORY.with(|inventory| inventory.borrow().values().cloned().collect())
}

#[query]
fn get_sales() -> Vec<SaleRecord> {
    SALES.with(|sales| sales.borrow().clone())
}

#[query]
fn financial_overview() -> (f64, f64) {
    let total_sales: f64 = SALES.with(|sales| sales.borrow().iter().map(|sale| sale.total_amount).sum());
    let inventory_value: f64 = INVENTORY.with(|inventory| {
        inventory.borrow().values().map(|item| item.quantity as f64 * item.price).sum()
    });
    (total_sales, inventory_value)
}

#[query]
fn reorder_suggestions(threshold: u64) -> Vec<InventoryItem> {
    INVENTORY.with(|inventory| {
        inventory
            .borrow()
            .values()
            .filter(|item| item.quantity < threshold)
            .cloned()
            .collect()
    })
}

ic_cdk::export_candid!();
