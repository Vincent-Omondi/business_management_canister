use candid::{CandidType, Deserialize, Encode};
use ic_cdk::api::{time};
use ic_cdk_macros::{query, update};
use serde::Serialize;
use std::cell::RefCell;
use std::collections::{HashMap};
use std::sync::atomic::{AtomicU64, Ordering};

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
}

static NEXT_ITEM_ID: AtomicU64 = AtomicU64::new(1);

fn generate_id() -> ItemId {
    NEXT_ITEM_ID.fetch_add(1, Ordering::Relaxed)
}

#[update]
fn add_item(name: String, quantity: u64, price: f64) -> Result<ItemId, String> {
    if name.trim().is_empty() {
        return Err("Item name cannot be empty.".to_string());
    }
    if quantity == 0 {
        return Err("Quantity must be greater than zero.".to_string());
    }
    if price <= 0.0 {
        return Err("Price must be positive.".to_string());
    }

    let id = generate_id();
    INVENTORY.with(|inventory| {
        inventory.borrow_mut().insert(
            id,
            InventoryItem {
                id,
                name,
                quantity,
                price,
            },
        );
    });
    Ok(id)
}

#[update]
fn update_item(id: ItemId, name: Option<String>, quantity: Option<u64>, price: Option<f64>) -> Result<(), String> {
    INVENTORY.with(|inventory| {
        let mut inventory = inventory.borrow_mut();
        if let Some(item) = inventory.get_mut(&id) {
            if let Some(new_name) = name {
                if new_name.trim().is_empty() {
                    return Err("Updated name cannot be empty.".to_string());
                }
                item.name = new_name;
            }
            if let Some(new_quantity) = quantity {
                if new_quantity == 0 {
                    return Err("Updated quantity must be greater than zero.".to_string());
                }
                item.quantity = new_quantity;
            }
            if let Some(new_price) = price {
                if new_price <= 0.0 {
                    return Err("Updated price must be positive.".to_string());
                }
                item.price = new_price;
            }
            Ok(())
        } else {
            Err(format!("Item with ID {} not found.", id))
        }
    })
}

#[update]
fn remove_item(id: ItemId) -> Result<(), String> {
    INVENTORY.with(|inventory| {
        if inventory.borrow_mut().remove(&id).is_some() {
            Ok(())
        } else {
            Err(format!("Item with ID {} not found.", id))
        }
    })
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
fn get_item_details(id: ItemId) -> Option<InventoryItem> {
    INVENTORY.with(|inventory| inventory.borrow().get(&id).cloned())
}

#[query]
fn search_item_by_name(name: String) -> Vec<InventoryItem> {
    let name_lower = name.to_lowercase();
    INVENTORY.with(|inventory| {
        inventory
            .borrow()
            .values()
            .filter(|item| item.name.to_lowercase().contains(&name_lower))
            .cloned()
            .collect()
    })
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

#[query]
fn get_top_selling_items(n: usize) -> Vec<(String, u64)> {
    let mut sales_count = HashMap::new();

    SALES.with(|sales| {
        for sale in sales.borrow().iter() {
            for item in &sale.items {
                *sales_count.entry(item.name.clone()).or_insert(0) += item.quantity;
            }
        }
    });

    let mut sales_vec: Vec<_> = sales_count.into_iter().collect();
    sales_vec.sort_by(|a, b| b.1.cmp(&a.1));
    sales_vec.into_iter().take(n).collect()
}

ic_cdk::export_candid!();
