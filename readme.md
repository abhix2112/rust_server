# Rust Web Server

This is a simple Rust web server built using the **Axum** framework. It provides two routes:

- `GET /` → Returns a welcome message.
- `GET /password` → Generates a random alphanumeric password.

## Features

- Built with **Axum** (lightweight and async-friendly)
- Uses **Tokio** as the async runtime
- Generates random passwords using the **rand** crate
- Returns JSON responses using **serde**

## Installation

Ensure you have **Rust** installed. If not, install it using:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then, clone this repository and navigate to the project directory:

```sh
git clone https://github.com/your-username/rust-web-server.git
cd rust-web-server
```

## Dependencies

The project uses the following dependencies:

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
rand = "0.8"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

You can install them using:

```sh
cargo check
```

## Running the Server

Run the server using:

```sh
cargo run
```

The server will start at:

```
http://127.0.0.1:3000
```

## API Endpoints

### 1. Root Route

**Endpoint:** `GET /`

```sh
curl http://127.0.0.1:3000/
```

**Response:**

```
Welcome to Rust Web Server!
```

### 2. Password Generator

**Endpoint:** `GET /password`

```sh
curl http://127.0.0.1:3000/password
```

**Response (JSON):**

```json
{
  "password": "G8jKpZxA3QwY"
}
```

## Next Steps

- Add database support (SQLite/PostgreSQL)
- Deploy using Docker
- Implement authentication

## License

This project is licensed under the MIT License.

---

Feel free to contribute by submitting a pull request!
