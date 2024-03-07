<p align="center">
  <a href="https://github.com/tatupesonen/kiistor">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://github.com/tatupesonen/kiistor/assets/7150217/89625127-4602-4bff-9186-3c5b8417f305">
      <img src="https://github.com/tatupesonen/kiistor/assets/7150217/89625127-4602-4bff-9186-3c5b8417f305" alt="kiistor" />
    </picture>
  </a>
  
Small utility written in Rust designed to facilitate the loading of game keys in and out of a PostgreSQL database. It provides a simple command-line interface for managing them.
</p>

## Features
- **Addition and Retrieval:** Allows adding new game keys to the database and retrieving existing keys.
- **Easy-to-Use Interface:** Provides a user-friendly command-line interface for interacting with the utility.

## Installation
Grab the latest release for your platform from the [releases](https://github.com/tatupesonen/kiistor/releases/latest) page

## Usage

- **Get keys by title:**

  ```bash
  kiistor get --title <title> --count <count>
  ```

- **Load keys from a file:**

  ```bash
  kiistor load --file <file_path> --title <title>
  ```
  In this example, it is assumed that game keys are newline delimited (`\n`)

- **List all keys in the database:**

  ```bash
  kiistor list
  ```

- **Perform database migrations:**

  ```bash
  kiistor migrate
  ```
  You only need to run migrations if you're installing kiistor for the first time or you are upgrading to a new version of kiistor.

## Contributing

Contributions are welcome! If you have any suggestions, feature requests, or bug reports, please [create an issue](https://github.com/tatupesonen/kiistor/issues) or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**Disclaimer:** This utility is provided as-is without any warranties. Use it at your own risk.
