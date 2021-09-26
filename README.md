# Planter
Blazingly fast and simple data generation & seeding for MongoDB


## Installation

Use the package manager [pip](https://pip.pypa.io/en/stable/) to install foobar.
Add the following to your Cargo.toml
```toml
[dependencies]
mongoseed = {git = "https://github.com/valencian-digital/mongoseed", branch = "main"}# from online repo```
```
## Usage

```python
import foobar

# returns 'words'
foobar.pluralize('word')

# returns 'geese'
foobar.pluralize('goose')

# returns 'phenomenon'
foobar.singularize('phenomena')
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)
