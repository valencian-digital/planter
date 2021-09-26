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
fn user_generator(_history: &planter::GeneratedData) -> bson::Document {
    return doc! {
        "_id": bson::Bson::ObjectId(bson::oid::ObjectId::new()),
        "name": "Jane Doe"
    };
}

let amount = 1000;
let now = Instant::now();
let collections: Vec<(String, planter::EntityGenerator)> = vec![
    (String::from("users"), user_generator),
];

planter::seed_data(
    collections,
    planter::Configurations::new(amount, planter::SeedMode::Disk),
);
```

Then to import that data into your running MongoDB database, execute the following command:
```bash
mongorestore --db=test data/
```
Look at the [mongorestore](https://docs.mongodb.com/database-tools/mongorestore/#std-label-mongorestore-examples) documentation in order to see all of options

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)
