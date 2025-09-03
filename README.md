# Actix Web Boilerplate

This project is a template for building RESTful APIs in Rust using [Actix Web](https://actix.rs/). It includes best practices, useful middlewares, and a modular structure to make development and maintenance easier.

## Features

- Modular structure: controllers, services, routes
- Correlation ID middleware for request tracing
- Configurable logging with correlation integration
- Ready-to-use OpenAPI/Swagger documentation (using [utoipa](https://docs.rs/utoipa))
- Example endpoints for monitoring and QR codes

## Structure

```
src/
   controllers/      # Business logic per resource
   routes/           # Route and scope definitions
   libraries/        # Middlewares, utilities, constants
   services/         # Reusable services
   docs.rs           # OpenAPI/Swagger documentation definition
   config.rs         # Loads environment variables and app config
   main.rs           # Entry point
```

## Usage

1. Install Rust: https://rustup.rs/
2. Clone the repository:
   ```sh
   git clone https://github.com/your-username/actix-web-boilerplate-template.git
   cd actix-web-boilerplate-template
   ```
3. Install dependencies and run:
   ```sh
   cargo run
   ```
4. Access the API at `http://localhost:8080/api`
5. Swagger documentation at `http://localhost:8080/swagger-ui/`

## OpenAPI/Swagger Documentation with utoipa

This project uses [utoipa](https://docs.rs/utoipa) to automatically generate OpenAPI documentation for all endpoints.

### How does it work?

- Routes and handlers are annotated with macros like `#[utoipa::path(...)]` to describe endpoints, parameters, and responses.
- The file `src/docs.rs` defines the main documentation structure and data schemas.
- The documentation is automatically exposed at `/swagger-ui/` when running the project.


### Example: Adding and Documenting a New Endpoint

Suppose you want to add a new endpoint to get the API version:

1. Create the handler in `src/controllers/version.rs`:

```rust
use actix_web::{get, HttpResponse, Responder};

#[utoipa::path(
   get,
   path = "/version",
   context_path = "/api",
   description = "Get API version.",
   responses(
      (status = 200, description = "Current API version", body = String)
   )
)]
#[get("/version")]
pub async fn get_version() -> impl Responder {
   HttpResponse::Ok().body("1.0.0")
}
```

2. Register the handler in your router (e.g., in `src/routes/version.rs`).

3. Add the handler to the documentation in `src/docs.rs`:

```rust
#[derive(OpenApi)]
#[openapi(
   // ...existing code...
   paths(
      // ...existing endpoints...
      crate::controllers::version::get_version,
   ),
   // ...existing code...
)]
pub struct ApiDoc;
```

4. Run the project and visit `/swagger-ui/` to see your new endpoint documented.

## Environment Variables
Note: For each environment variable you want to use, you must add a corresponding field to the `Config` struct in `src/config.rs`. Then, read the variable with `env::var` and assign it to the field in the constructor. This ensures the variable is read and parsed with the correct type.

For example, to use a variable called `API_KEY`, add it to the struct and assign it in `Config::new()`:

```rust
pub struct Config {
   pub port: u16,
   pub log_level: String,
   pub api_key: String, // Add your variable here
}

impl Config {
   pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
      let _ = dotenvy::dotenv();

      // ...existing code...

      let api_key = env::var("API_KEY")
         .expect("API_KEY must be set");

      Ok(Config {
         port: application_port,
         log_level,
         api_key,
      })
   }
}
```

You can set environment variables using a `.env` file in the project root. Example:

```env
PORT=8080
LOG_LEVEL=info
```

These variables are loaded automatically at runtime in `src/config.rs`. You do not need to load them manually; the configuration module handles this when the application starts.

Example usage in `main.rs`:

```rust
use crate::config::Config;

fn main() {
   let cfg = Config::new().expect("failed to load configuration from environment");
   println!("Running on port: {} with log level: {}", cfg.port, cfg.log_level);
   // ...rest of your setup
}
```
