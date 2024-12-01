# Business Management Canister

The **Business Management Canister** is a backend application for managing business operations using the Internet Computer Protocol (ICP). This canister provides functionalities for inventory management, sales tracking, and financial overviews, implemented in Rust.

---

## Features

1. **Inventory Management**
   - Add, update, and remove inventory items.
   - Monitor stock levels and receive reorder suggestions.

2. **Sales Tracking**
   - Record sales transactions.
   - Track historical sales data.

3. **Financial Overview**
   - Calculate total sales revenue.
   - Evaluate the value of current inventory.

---

## Project Structure

```
business_management_canister/
├── Cargo.toml                # Configuration for the Rust canister.
├── dfx.json                  # DFX project configuration.
├── src/
│   ├── icp_rust_boilerplate_backend/
│       ├── Cargo.toml        # Rust package configuration for the backend.
│       ├── icp_rust_boilerplate_backend.did # Candid interface.
│       ├── src/lib.rs        # Main application logic.
├── package.json              # Scripts for deployment and candid generation.
```

---

## Installation and Setup

### Prerequisites

- **Rust Compiler** (1.64 or higher)
  ```bash
  curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
  source "$HOME/.cargo/env"
  ```

- **WebAssembly Target for Rust**
  ```bash
  rustup target add wasm32-unknown-unknown
  ```

- **Candid Extractor**
  ```bash
  cargo install candid-extractor
  ```

- **DFX SDK**
  ```bash
  DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
  echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
  source ~/.bashrc
  ```

### Local Deployment

1. Start the Internet Computer local replica:
   ```bash
   dfx start --background
   ```

2. Deploy the canister:
   ```bash
   dfx deploy
   ```

### Generating Candid File

To automatically regenerate the candid file after changes:
1. Add the script located [here](https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh) to the root of your project.
2. Update line 16 of the script with your canister name.
3. Use the following command:
   ```bash
   ./did.sh && dfx generate
   ```

   Alternatively, you can use the following in `package.json`:
   ```json
   {
     "scripts": {
       "generate": "./did.sh && dfx generate",
       "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
     }
   }
   ```
   Run `npm run generate` or `npm run gen-deploy` as needed.

---

## Code Overview

### Key Functions

- **Inventory Operations**
  - `add_item(name, quantity, price)`
  - `update_item(id, name, quantity, price)`
  - `remove_item(id)`

- **Sales Management**
  - `record_sale(items)` - Logs sales transactions.
  - `get_sales()` - Retrieves sales history.

- **Queries**
  - `get_inventory()` - Fetches inventory details.
  - `financial_overview()` - Provides sales revenue and inventory value.
  - `reorder_suggestions(threshold)` - Suggests items to reorder.

---

## Contributing

Contributions are welcome! Please submit a pull request or create an issue for any feature requests or bug reports.

---

## License

This project is licensed under the MIT License. 
---

## Resources

- [Dfinity SDK Documentation](https://sdk.dfinity.org/docs/)
- [Candid Language Reference](https://internetcomputer.org/docs/current/developer-docs/quickstart/candid-intro/)
- [Rust Programming Language](https://www.rust-lang.org/)