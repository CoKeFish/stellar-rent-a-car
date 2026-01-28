# Rent-a-Car dApp - Stellar Soroban Smart Contract

[🇪🇸 Versión en Español](README.es.md)

![Stellar](https://img.shields.io/badge/Stellar-FFD700?style=for-the-badge&logo=stellar&logoColor=000000)
![Soroban](https://img.shields.io/badge/Soroban-FFD700?style=for-the-badge&logo=stellar&logoColor=000000)
![Rust](https://img.shields.io/badge/Rust-8B4513?style=for-the-badge&logo=rust&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)
![React](https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB)

> Decentralized application (dApp) for vehicle rental built on the Stellar network using Soroban smart contracts. This project implements a complete car rental management system with administrator, owner, and renter roles.

![Application preview](images/Banner.png)

---

## Table of Contents

- [Project Description](#project-description)
- [Latest Features](#latest-features)
- [Main Features](#main-features)
- [Requirements](#requirements)
- [Installation](#installation)
- [Usage](#usage)
- [Contract Architecture](#contract-architecture)
- [Implemented Functionalities](#implemented-functionalities)
- [Project Structure](#project-structure)
- [Testing](#testing)
- [Technologies Used](#technologies-used)

---

## Project Description

This application enables decentralized vehicle rental management using Stellar blockchain technology.

Users can:

| Role | Functionalities |
|-----|----------------|
| **Administrators** | Configure commissions, manage vehicles, and withdraw earnings |
| **Owners** | Add vehicles to the catalog, receive rental payments |
| **Renters** | Browse available vehicles, rent and return cars |

---

## Latest Features

### User Experience Improvements

<details>
<summary><strong>Toast notification system</strong></summary>

Replaced alerts with non-blocking toast notifications for better visual feedback. All operations display clear success or error messages without interrupting the user flow.
![Application preview](images/toast.png)
</details>

<details>
<summary><strong>Robust error handling</strong></summary>

Complete Stellar error mapping system with descriptive messages in Spanish. The system automatically detects the error type and presents clear messages to the user.

</details>

<details>
<summary><strong>Transaction validation</strong></summary>

All operations validate transaction success before updating the application state. This prevents inconsistencies and ensures the interface reflects the actual blockchain state.

</details>

<details>
<summary><strong>Prevention of duplicate executions</strong></summary>

Protection against double-clicks and simultaneous executions with state flags. Buttons are automatically disabled during operations to prevent duplicate transactions.

</details>

<details>
<summary><strong>Intuitive numeric fields</strong></summary>

You can now completely clear fields and type from scratch without issues. Fields use string values internally, allowing smooth and natural editing.

</details>

<details>
<summary><strong>Smart buttons</strong></summary>

Automatically disabled during operations and when no funds are available. Display clear loading states (e.g., "Renting...", "Deleting...") for immediate feedback.

</details>

### Security and Reliability Improvements

- **Real-time validation**: Buttons are disabled when values are invalid
- **Network error handling**: Clear messages when transactions fail
- **Synchronized state**: Application state updates only after confirming successful transactions

---

## Recommendation for Development and Testing

> **IMPORTANT**: For convenience when testing the application, **I recommend creating 3 different accounts in Freighter** and keeping them connected simultaneously.

**Recommended wallets:**

- **Administrator Wallet**: To configure commissions and manage vehicles
- **Owner Wallet**: To add vehicles and receive rental payments
- **Renter Wallet**: To rent and return vehicles

---

## Main Features

### Administrator Commission

The Administrator can configure a fixed monetary commission for each rental. This commission is automatically added to the deposit paid by the renter, ensuring platform revenue.

**Functionalities:**

- *Commission configuration by Administrator*
- *Automatic commission on each rental*
- *Withdrawal of accumulated commissions at any time*
- *Query available commission for withdrawal*

![Set Commission](images/set-commission.png)

---

### Deposit + Commission

When renting a vehicle, the configured commission is automatically added to the total deposit. The Owner receives 100% of the rental amount (without commission deduction), while the Administrator accumulates the configured commission.

**Features:**

- *Automatic calculation: `Total Deposit = Rental Amount + Commission`*
- *Owner receives full rental amount*
- *Commission accumulates in Administrator's account*

![Rental with Commission](images/rental-with-commission.png)

---

### Administrator Fund Withdrawal

The Administrator can query and withdraw accumulated commissions at any time through an intuitive interface.

**Functionalities:**

- *Real-time display of available commission*
- *Modal for commission withdrawal*
- *Available funds validation*
- *Button disabled when no funds available*

![Commission Withdrawal](images/withdraw-commission.png)

---

### Car Returns

Renters can return vehicles they have rented, changing the vehicle status from "Rented" to "Available".

**Functionalities:**

- *"Return" button visible for renters on rented vehicles*
- *Automatic vehicle status change*
- *Real-time catalog update*

![Return Car](images/return-car.png)

---

### Restricted Owner Withdrawals

Owners can only withdraw their funds when the vehicle has been returned ("Available" status). The withdrawal button is disabled if:

- The vehicle is rented ("Rented" status)
- No funds available for withdrawal

**Functionalities:**

- *Contract validation: only allows withdrawal if car is available*
- *"Withdraw" button visible only when funds are available*
- *Modal to specify withdrawal amount*
- *Real-time display of available funds*

![Owner Withdrawal](images/withdraw-owner.png)

---

## Requirements

Before installing and running the project, make sure you have installed:

| Tool | Description | Link |
|-------------|-------------|--------|
| **Rust** | Programming language (latest stable version) | [Install Rust](https://www.rust-lang.org/tools/install) |
| **Cargo** | Rust package manager (included with Rust) | - |
| **Rust Target for Soroban** | Target needed to compile contracts | [Soroban Guide](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup) |
| **Node.js** | JavaScript runtime (v22 or higher) | [Install Node.js](https://nodejs.org/en/download/package-manager) |
| **npm** | Node.js package manager (included with Node.js) | - |
| **Stellar CLI** | Stellar command-line tool | [Stellar CLI](https://github.com/stellar/stellar-core) |
| **Scaffold Stellar CLI Plugin** | Plugin for dApp development | [Scaffold Stellar](https://github.com/AhaLabs/scaffold-stellar) |

> **Important note**: All these tools are necessary to develop and deploy the project. Make sure you have them installed before continuing.

---

## Installation

### 1. Clone the repository

```bash
git clone <your-repository>
cd stellar0dApp
```

---

### 2. Configure environment variables

```bash
cp .env.example .env
```

**Important:** Edit the `.env` file with your network and contract configurations before continuing.

---

### 3. Install frontend dependencies

```bash
npm install
```

---

### 4. Install contract dependencies

```bash
npm run install:contracts
```

---

### 5. Compile the contract

```bash
cd contracts/rent-a-car
cargo build --target wasm32-unknown-unknown --release
```

> **Note**: This step may take several minutes the first time it runs.

---

## Usage

### Development mode

To run the project in development mode:

```bash
npm run dev
```

**What starts:**

- *Vite development server*
- *Scaffold Stellar watcher to rebuild contract clients*

Open your browser at the URL shown in the console (usually `http://localhost:5173`).

---

### Build for production

```bash
npm run build
```

---

### Production preview

```bash
npm run preview
```

---

## Contract Architecture

### Data Structures

The contract uses the following main structures:

#### Car (Vehicle)

```rust
pub struct Car {
    pub car_status: CarStatus,
    pub available_to_withdraw: i128,
}
```

**Important fields:**

- `car_status`: Current vehicle status
- `available_to_withdraw`: Amount available for owner withdrawal

---

#### Rental

```rust
pub struct Rental {
    pub total_days_to_rent: u32,
    pub amount: i128,
}
```

**Important fields:**

- `total_days_to_rent`: Total rental days
- `amount`: Rental amount (in stroops)

---

#### CarStatus (Vehicle Status)

| Status | Description |
|--------|-------------|
| `Available` | Available for rent |
| `Rented` | Currently rented |
| `Maintenance` | Under maintenance |

---

### Contract Functions

#### Public functions (anyone can call)

These functions don't require authentication and can be called by any user:

| Function | Description |
|---------|-------------|
| `get_car_status(owner)` | Gets vehicle status |
| `get_admin_available_to_withdraw()` | Gets Admin available commission |
| `get_owner_available_to_withdraw(owner)` | Gets Owner available funds |

---

#### Administrator functions

Require administrator authentication:

| Function | Description |
|---------|-------------|
| `set_admin_commission(commission)` | Configures Administrator commission |
| `withdraw_admin_commission(amount)` | Withdraws accumulated commissions |
| `remove_car(owner)` | Removes vehicle from catalog |

---

#### Owner functions

Require owner authentication:

| Function | Description |
|---------|-------------|
| `add_car(owner, price_per_day)` | Adds vehicle to catalog |
| `payout_owner(owner, amount)` | Withdraws funds (only if car is available) |

---

#### Renter functions

Require renter authentication:

| Function | Description |
|---------|-------------|
| `rental(renter, owner, total_days_to_rent, amount)` | Rents a vehicle |
| `return_car(renter, owner)` | Returns a rented vehicle |

---

## Implemented Functionalities

The project includes all main functionalities for complete vehicle rental management:

* **Vehicle management**: Add and remove vehicles from catalog, real-time status queries
* **Rental system**: Daily rental with automatic price calculation and availability validation
* **Commission system**: Commission configuration, automatic accumulation, and Administrator earnings withdrawal
* **Vehicle returns**: Renters can return vehicles, automatically changing their status
* **Fund management**: Restricted withdrawals for Owners (only when car is available), real-time fund display
* **User interface**: Role-based dashboard, intuitive modals, form validation, robust error handling, and toast notifications

---

## Project Structure

```
stellar0dApp/
├── contracts/
│   └── rent-a-car/
│       ├── src/
│       │   ├── contract.rs              # Main contract implementation
│       │   ├── interfaces/
│       │   │   └── contract.rs          # Contract interface
│       │   ├── storage/
│       │   │   ├── admin.rs              # Admin storage functions
│       │   │   ├── car.rs                # Car storage functions
│       │   │   ├── rental.rs             # Rental storage functions
│       │   │   └── structs/
│       │   │       ├── car.rs            # Car data structure
│       │   │       └── rental.rs         # Rental data structure
│       │   ├── methods/
│       │   │   ├── admin/                # Administrator methods
│       │   │   ├── owner/                # Owner methods
│       │   │   ├── renter/               # Renter methods
│       │   │   └── public/               # Public methods (queries)
│       │   ├── events/                   # Event definitions
│       │   └── tests/                    # Contract unit tests
│       └── Cargo.toml
├── src/
│   ├── components/
│   │   ├── CarList.tsx                   # Vehicle list
│   │   ├── CreateCarForm.tsx             # Form to add vehicles
│   │   ├── RentCarModal.tsx              # Rental modal
│   │   ├── SetCommissionModal.tsx        # Commission configuration modal
│   │   ├── WithdrawCommissionModal.tsx   # Commission withdrawal modal
│   │   └── WithdrawOwnerModal.tsx        # Owner withdrawal modal
│   ├── pages/
│   │   ├── Dashboard.tsx                 # Main dashboard
│   │   ├── RoleSelection.tsx             # Role selection
│   │   └── ConnectWallet.tsx              # Wallet connection
│   ├── services/
│   │   ├── stellar.service.ts            # Service to interact with Stellar
│   │   └── wallet.service.ts             # Wallet handling service
│   ├── providers/
│   │   └── StellarAccountProvider.tsx    # Account context provider
│   └── interfaces/                        # TypeScript definitions
├── package.json
├── environments.toml
└── README.md
```

---

## Testing

The project includes a complete unit test suite for the smart contract.

### Run contract tests

```bash
cd contracts/rent-a-car
cargo test --lib
```

> **Tip**: You can run specific tests using `cargo test --lib <test_name>`

---

### Implemented Tests

#### Administration tests

*Administrative functions*

- `test_set_admin_commission_successfully`
- `test_withdraw_admin_commission_successfully`
- `test_get_admin_available_to_withdraw_after_rental`

---

#### Vehicle tests

*Vehicle catalog management*

- `test_add_car_successfully`
- `test_remove_car_deletes_from_storage`
- `test_get_car_status_returns_available`

---

#### Rental tests

*Rental and return process*

- `test_rental_car_successfully`
- `test_rental_with_admin_commission`
- `test_return_car_successfully`

---

#### Withdrawal tests

*Fund withdrawal validation*

- `test_payout_owner_successfully`
- `test_payout_owner_when_car_is_rented_fails`
- `test_get_owner_available_to_withdraw_car_available_with_funds`

---

#### Authentication tests

*Security and permissions*

- Tests to verify only authorized users can execute specific functions

---

## Technologies Used

**Backend (Smart Contract):**
- *Rust* with *Soroban SDK* for smart contracts on Stellar
- *Stellar XDR* for data serialization

**Frontend:**
- *React 19* with *TypeScript* for user interface
- *Vite* as build tool and dev server
- *Stellar SDK* and *Stellar Wallets Kit* for wallet integration (Freighter)

**Development tools:**
- *Scaffold Stellar* as base framework
- *Cargo* and *npm* for dependency management

**Network:**
- *Stellar Testnet* for testing and development

---

## Author

**Rodion Romanovich**

---

## Acknowledgments

- **Núcleo** - For the opportunity to participate in the bootcamp
- **Stellar Community** - For the documentation and tools, and for being such a fun technology
- **Javeblockchain** - Because I didn't think I'd meet such wonderful people

---
