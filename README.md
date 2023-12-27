# Namada Account Generator

This Rust program utilizes the Namada SDK to generate new accounts.

## Usage

Run this to generate accounts and display their details:

```bash
cargo run
```

## Configuration

Configure the program by modifying the relevant parameters in the `main` function, such as the RPC server URL and the number of accounts to generate.

```rust
// Setup client
let http_client = HttpClient::new("https://rpc.luminara.icu/").unwrap();

// ...

// Generate accounts
let accounts = gen_accounts(&mut namada, 1).await;
```

## Contributing

Feel free to contribute to the project by opening issues, providing feedback, or submitting pull requests. Contributions are welcome and appreciated.

## License

This project is licensed under the [MIT License](LICENSE).
