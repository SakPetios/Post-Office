# Post Office

Post Office is a command-line tool designed to simplify unit testing of APIs. It aims to streamline the API testing process by allowing you to create Lua scripts for testing, which are then analyzed and compared by Post Office. This tool is written in Rust and features a TUI (Text-based User Interface) using Cursive.

## Features

1. **Pre-Executors**: Post Office supports Lua scripts that run once and don't require a result. These can be useful, for example, for authentication or setting up initial conditions.

2. **Recipes**: When you want to use pre-executors, you can define recipes to orchestrate the execution of Lua scripts and tests.

3. **Config**: Post Office provides configuration options to customize your testing environment.

## Getting Started

To get started with Post Office, follow these steps:

1. Clone the repository: `git clone https://github.com/SakPetios/post-office.git`

2. Build the tool: `cargo build`

3. Create your Lua scripts and recipes to define your API tests.

4. Run Post Office: `./post-office`

5. Analyze and compare the results of your API tests.

For detailed usage and examples, refer to the documentation in the repository.

## Example Usage

Here's a simple example of using a Lua script to test an API endpoint:

```lua
-- test.lua
local response = http.get('https://api.example.com/data')
assert(response.status_code == 200, 'HTTP request failed')
-- Add more assertions and test logic here
```

This is just a basic overview. For more advanced use cases and configuration options, please refer to the documentation.

## Contributing

We welcome contributions from the open-source community. If you'd like to contribute to Post Office, please follow our [contribution guidelines](youtube.com).
## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.


Thank you for choosing Post Office for your API testing needs. Happy testing!
