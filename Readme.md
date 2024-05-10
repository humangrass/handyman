# Handyman

A service designed to collect prices of cryptocurrency assets. Perhaps it will grow to analyze collected data.

## Features

Collecting minute candle data from:
- [Bitget](https://www.bitget.com/)
- TODO: [Jupiter](https://jup.ag/)
- TODO: something else

## Usage

Update cargo more often =3

```bash
cargo update
```

Create all config files from `*.example.*` files. Just copy-paste for example. 

After this you can start the database using `docker-compose up -d` and apply migrations using [sqlx-cli](https://crates.io/crates/sqlx-cli).

Then you can use following commands:

```bash
cargo build
```

```bash
cargo run
```

Something went wrong? Try to change params in all config files.

## How to contribute

We welcome contributions to our project! If you'd like to contribute, here's what you can do:

1. Report a problem or suggest an improvement by creating a new issue.
2. Fork the repository and create a new branch.
3. Make changes and submit a pull request.

## Connect with us

If you have questions, suggestions or problems, please contact us:

- Email: human.grass@gmail.com
- Telegram: [@human_grass](https://t.me/human_grass)

## License

This project is licensed under the [MIT License](LICENSE).
