# PG Connector 🐘

[![Crates.io](https://img.shields.io/crates/v/pg_connector.svg)](https://crates.io/crates/pg_connector)
[![Documentation](https://docs.rs/pg_connector/badge.svg)](https://docs.rs/pg_connector)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/PTechSoftware/pg_connector#license)

**A high-performance, developer-friendly PostgreSQL connector for Rust.** This crate provides a streamlined API for managing database connections with built-in pooling and dedicated support for read/write cluster split architectures.

Built on top of [`tokio-postgres`](https://github.com/sfackler/rust-postgres) and [`bb8`](https://github.com/djc/bb8), it ensures your application scales efficiently while keeping your code clean.

---

## ✨ Features

- 🏎️ **High Performance**: Asynchronous I/O powered by `tokio` and `tokio-postgres`.
- 🔋 **Robust Pooling**: Intelligent connection management using `bb8`.
- ⚖️ **Read/Write Splitting**: Native support for clusters with separate read and write endpoints.
- 🚰 **Streamlined API**: One unified trait (`SqlExpose`) for all your SQL operations.
- 🛠️ **Transaction Support**: Full async implementation for PostgreSQL transactions.
- 🛡️ **Errors Made Easy**: Specific, actionable error types for connection and pooling issues.

---

## 🚀 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pg_connector = "1.0.0"
tokio = { version = "1.40", features = ["full"] }
bb8-postgres = "0.9" # Optional: if you need custom TLS or types
```

---

## 💡 Quick Start

```rust
use pg_connector::{PgConnector, SqlExpose, ClusterDefine, PoolManager};
use bb8_postgres::tokio_postgres::NoTls;
use bb8::QueueStrategy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Setup Pool Managers for Write (Primary) and Read (Replica)
    let manager_write = PoolManager::new(
        "primary.db.ptech.software", 
        5432, 
        "postgres", 
        "mypassword", 
        Some("mydb"), 
        0, 
        5, 
        NoTls
    );

    let manager_read = PoolManager::new(
        "replica.db.ptech.software", 
        5432, 
        "postgres", 
        "mypassword", 
        Some("mydb"), 
        0, 
        5, 
        NoTls
    );

    // 2. Initialize Read and Write Pools
    let pool_write = manager_write.get_pool(5, 20, true, 5000, None, QueueStrategy::Fifo).await?;
    let pool_read = manager_read.get_pool(10, 50, true, 5000, None, QueueStrategy::Fifo).await?;

    // 3. Create the Connector
    let connector = PgConnector::new(pool_write, pool_read);

    // 4. Run Queries (Reads from Replica, Writes to Primary)
    let rows = connector.query(
        ClusterDefine::READ, 
        "SELECT id, name FROM users WHERE active = $1", 
        &[&true]
    ).await?;

    for row in rows {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        println!("User {}: {}", id, name);
    }

    Ok(())
}
```

---

## 🛠️ Advanced Usage

### Transactions

`pg_connector` makes transactions simple. The `SqlExpose` trait is also implemented for `Transaction`, allowing you to use the same methods inside a transaction block.

```rust
let mut conn = connector.get_write().await?;
let transaction = conn.transaction().await?;

// Use the same trait methods!
transaction.execute(ClusterDefine::WRITE, "UPDATE accounts SET balance = balance - 100 WHERE id = 1", &[]).await?;
transaction.execute(ClusterDefine::WRITE, "UPDATE accounts SET balance = balance + 100 WHERE id = 2", &[]).await?;

transaction.commit().await?;
```

### Pre-prepared Statements

For repeated queries, use `prepare` or `prepare_typed` for maximum performance.

```rust
let stmt = connector.prepare(ClusterDefine::READ, "SELECT * FROM large_table WHERE val = $1").await?;
let results = connector.query(ClusterDefine::READ, &stmt.query_string(), &[&42]).await?;
```

---

## 📄 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

---

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

---

## 📝 Changelog

### v1.0.1 (2026-03-22)

#### Add feature

- Add feature to get detailed error information from Postgres.

<div align="center">
  <sub>Built with ❤️ by PTechSoftware</sub>
</div>
