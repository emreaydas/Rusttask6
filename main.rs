#![allow(unused_imports)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use structopt::StructOpt;

#[derive(Debug, Serialize, Deserialize)]
struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Sale {
    product_name: String,
    quantity_sold: u32,
    sale_price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Purchase {
    product_name: String,
    quantity_purchased: u32,
    purchase_price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Inventory {
    products: HashMap<String, Product>,
    sales: Vec<Sale>,
    purchases: Vec<Purchase>,
}

use std::io::{self, Write};

fn add_product(inventory: &mut Inventory, product: Product) {
    inventory.products.insert(product.name.clone(), product);
}

fn edit_product(inventory: &mut Inventory, product_name: &str, new_product: Product) {
    if let Some(product) = inventory.products.get_mut(product_name) {
        product.name = new_product.name;
        product.description = new_product.description;
        product.price = new_product.price;
        product.quantity = new_product.quantity;
    }
}

fn delete_product(inventory: &mut Inventory, product_name: &str) {
    inventory.products.remove(product_name);
}

fn display_inventory(inventory: &Inventory) {
    println!("Inventory:");
    for (_, product) in &inventory.products {
        println!(
            "Name: {}, Description: {}, Price: {:.2}, Quantity: {}",
            product.name, product.description, product.price, product.quantity
        );
    }
}

fn record_sale(inventory: &mut Inventory, sale: Sale) {
    if let Some(product) = inventory.products.get_mut(&sale.product_name) {
        if product.quantity >= sale.quantity_sold {
            product.quantity -= sale.quantity_sold;
            inventory.sales.push(sale);
        } else {
            println!("Error: Insufficient stock for sale.");
        }
    } else {
        println!("Error: Product not found in inventory.");
    }
}

fn calculate_total_sales(inventory: &Inventory) -> f64 {
    inventory
        .sales
        .iter()
        .map(|sale| sale.quantity_sold as f64 * sale.sale_price)
        .sum()
}

fn calculate_total_profit(inventory: &Inventory) -> f64 {
    let total_revenue: f64 = calculate_total_sales(inventory);
    let total_cost: f64 = inventory
        .sales
        .iter()
        .map(|sale| {
            inventory
                .products
                .get(&sale.product_name)
                .map(|product| sale.quantity_sold as f64 * product.price)
                .unwrap_or(0.0)
        })
        .sum();
    total_revenue - total_cost
}

fn record_purchase(inventory: &mut Inventory, purchase: Purchase) {
    if let Some(product) = inventory.products.get_mut(&purchase.product_name) {
        product.quantity += purchase.quantity_purchased;
        inventory.purchases.push(purchase);
    } else {
        println!("Error: Product not found in inventory.");
    }
}

fn calculate_total_purchases(inventory: &Inventory) -> f64 {
    inventory
        .purchases
        .iter()
        .map(|purchase| purchase.quantity_purchased as f64 * purchase.purchase_price)
        .sum()
}

fn generate_inventory_report(inventory: &Inventory) {
    println!("Inventory Report:");
    for (_, product) in &inventory.products {
        println!(
            "Name: {}, Description: {}, Price: {:.2}, Quantity: {}",
            product.name, product.description, product.price, product.quantity
        );
    }
}

fn generate_sales_report(inventory: &Inventory) {
    println!("Sales Report:");
    for sale in &inventory.sales {
        println!(
            "Product: {}, Quantity Sold: {}, Sale Price: {:.2}",
            sale.product_name, sale.quantity_sold, sale.sale_price
        );
    }
    println!("Total Sales: {:.2}", calculate_total_sales(inventory));
    println!("Total Profit: {:.2}", calculate_total_profit(inventory));
}

fn generate_purchase_report(inventory: &Inventory) {
    println!("Purchase Report:");
    for purchase in &inventory.purchases {
        println!(
            "Product: {}, Quantity Purchased: {}, Purchase Price: {:.2}",
            purchase.product_name, purchase.quantity_purchased, purchase.purchase_price
        );
    }
    println!("Total Purchases: {:.2}", calculate_total_purchases(inventory));
}

#[derive(StructOpt)]
enum Command {
    #[structopt(name = "add")]
    Add {
        #[structopt(subcommand)]
        entity: EntityType,
    },
    #[structopt(name = "edit")]
    Edit {
        #[structopt(subcommand)]
        entity: EntityType,
    },
    #[structopt(name = "delete")]
    Delete {
        #[structopt(subcommand)]
        entity: EntityType,
    },
    #[structopt(name = "display")]
    Display {
        #[structopt(subcommand)]
        entity: EntityType,
    },
    #[structopt(name = "sale")]
    Sale {
        #[structopt(short = "n", long = "name")]
        product_name: String,
        #[structopt(short = "q", long = "quantity")]
        quantity_sold: u32,
        #[structopt(short = "p", long = "price")]
        sale_price: f64,
    },
    #[structopt(name = "purchase")]
    Purchase {
        #[structopt(short = "n", long = "name")]
        product_name: String,
        #[structopt(short = "q", long = "quantity")]
        quantity_purchased: u32,
        #[structopt(short = "p", long = "price")]
        purchase_price: f64,
    },
    #[structopt(name = "inventory")]
    Inventory,
    #[structopt(name = "sales")]
    Sales,
    #[structopt(name = "purchases")]
    Purchases,
}

#[derive(StructOpt)]
enum EntityType {
    #[structopt(name = "product")]
    Product {
        #[structopt(short = "n", long = "name")]
        name: String,
        #[structopt(short = "d", long = "description")]
        description: String,
        #[structopt(short = "p", long = "price")]
        price: f64,
        #[structopt(short = "q", long = "quantity")]
        quantity: u32,
    },
}


fn main() {
    let mut inventory = Inventory {
        products: HashMap::new(),
        sales: Vec::new(),
        purchases: Vec::new(),
    };

    match Command::from_args() {
        Command::Add { entity } => match entity {
            EntityType::Product {
                name,
                description,
                price,
                quantity,
            } => {
                let product = Product {
                    name,
                    description,
                    price,
                    quantity,
                };
                add_product(&mut inventory, product);
            }
        },
        Command::Edit { entity } => match entity {
            EntityType::Product {
                name,
                description,
                price,
                quantity,
            } => {
                let new_product = Product {
                    name: name.clone(),
                    description,
                    price,
                    quantity,
                };
                edit_product(&mut inventory, &name, new_product);
            }
        },
        Command::Delete { entity } => match entity {
            EntityType::Product { name, .. } => {
                delete_product(&mut inventory, &name);
            }
        },
        Command::Display { entity } => match entity {
            EntityType::Product { .. } => {
                display_inventory(&inventory);
            }
        },
        Command::Sale {
            product_name,
            quantity_sold,
            sale_price,
        } => {
            let sale = Sale {
                product_name,
                quantity_sold,
                sale_price,
            };
            record_sale(&mut inventory, sale);
        }
        Command::Purchase {
            product_name,
            quantity_purchased,
            purchase_price,
        } => {
            let purchase = Purchase {
                product_name,
                quantity_purchased,
                purchase_price,
            };
            record_purchase(&mut inventory, purchase);
        }
        Command::Inventory => generate_inventory_report(&inventory),
        Command::Sales => generate_sales_report(&inventory),
        Command::Purchases => generate_purchase_report(&inventory),
    }
}
