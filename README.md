# Rust-crypto-pipeline-task


---

## Features

* **Clean Architecture & Separation of Concerns:** Project structure follows clear module boundaries for maintainability and scalability.
* **Fully Tested Pipeline:** Comprehensive test coverage across the entire data processing pipeline ensures reliability and correctness.
* **Robust Error Handling:** All potential errors are explicitly handled using Rust's `Result` and `Option` types, completely avoiding `unwrap()` and `expect()` for production-grade stability.
* **Asynchronous Processing with Tokio:** Leverages the `tokio` runtime for efficient, non-blocking I/O operations and high-concurrency data processing.
* **ClickHouse Database Integration:** Demonstrates interaction with a ClickHouse instance for high-performance analytical data storage.
* **Serialization/Deserialization with Serde:** Utilizes `serde` for efficient and safe data serialization and deserialization.
* **Configurable via Environment Variables:** Database and other sensitive configurations are managed through environment variables for secure and flexible deployment.
* **Modular Project Structure:** Well-organized codebase with clear folder separation for different components (e.g., `src/models`, `src/services`, `src/db`).

---

## Technologies Used

* **Rust** (Latest Stable)
* **Tokio** (Asynchronous Runtime)
* **Serde** (Serialization/Deserialization Framework)
* **ClickHouse** (Database)
* Other crates like `anyhow`,`dotenvy`

## Local Setup


## Project structure
```
├── src/
    └── generator/
        ├── mod.rs
        ├── default_generator.rs
        └── generator_config.rs
    └── models/
        ├── mod.rs
        └── users_models.rs
    └── pipeline/
        ├── mod.rs
        └── defualt_pipeline.rs
    └── storage/
        ├── mod.rs
        └── storage.rs
    └── traits/
        ├── mod.rs
        ├── calculate_user_stats_trait.rs
        ├── storage_trait.rs
        └── transfer_trait.rs
    ├── lib.rs
    ├── main.rs
    ├── globals.rs
    ├── utils.rs
├── tests/
    └── clickhouse_tests/
        ├── mod.rs
        └── tests.rs
    └── generator_tests/
        ├── mod.rs
        └── tests.rs
    └── pipeline_tests/
        ├── mod.rs
        └── tests.rs
    ├── mod.rs
```