# Actix Web Boilerplate

This project is a template for building RESTful APIs in Rust using [Actix Web](https://actix.rs/). It includes best practices, useful middlewares, and a modular structure to make development and maintenance easier.

## Features

- Modular structure: controllers, services, routes
- Correlation ID middleware for request tracing
- Configurable logging with correlation integration
- Ready-to-use OpenAPI/Swagger documentation
- Example endpoints for monitoring and QR codes

## Structure

```
src/
  controllers/      # Business logic per resource
  routes/           # Route and scope definitions
  libraries/        # Middlewares, utilities, constants
  services/         # Reusable services
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

## Environment Variables

- `PORT`: Listening port (default 8080)
- `LOG_LEVEL`: Log level (default info)
