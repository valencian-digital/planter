# Planter
Blazingly fast and simple data generation & seeding for MongoDB


## Installation

Use the package manager [cargo](https://pip.pypa.io/en/stable/) to install planter.
Add the following to your Cargo.toml
```toml
[dependencies]
planter = {git = "https://github.com/valencian-digital/planter", branch = "main"}# from online repo```
```
## Usage

```rust
let amount = 1000;
let now = Instant::now();
let collections: Vec<(String, planter::EntityGenerator)> = vec![
    (String::from("users"), user_generator),
    (String::from("posts"), company_generator),
    (String::from("comments"), company_generator),
    (String::from("communities"), company_generator),
    (String::from("chat"), company_generator),
];

planter::seed_data(
    collections,
    planter::Configurations::new(amount, planter::SeedMode::Disk),
);
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)
