# va-novo-projeto-va-gateway--api-gateway-em-rust

## [VA] Novo Projeto: va-gateway — API Gateway em Rust

This project is an API Gateway built with Rust, leveraging the Axum web framework, SQLx for database interactions, and PostgreSQL as the database. It provides standard API Gateway features such as request routing, health checks, and basic proxying capabilities.

## Features

*   **Request Routing**: Dynamically route incoming requests to various upstream services.
*   **Health Checks**: `/health` endpoint to monitor the gateway's operational status.
*   **Proxying**: Forward requests to configured upstream services.
*   **Database Connectivity**: Integration with PostgreSQL via SQLx for potential configuration storage or logging.
*   **Configuration**: Environment variable-based configuration using `dotenvy`.
*   **Observability**: Structured logging with `tracing` and `tracing-subscriber`.
*   **CORS**: Configurable Cross-Origin Resource Sharing.

## Tech Stack

*   **Language**: Rust
*   **Web Framework**: [Axum](https://docs.rs/axum/latest/axum/)
*   **Database ORM/Client**: [SQLx](https://github.com/launchbadge/sqlx) (PostgreSQL)
*   **Asynchronous Runtime**: [Tokio](https://tokio.rs/)
*   **HTTP Client**: [reqwest](https://docs.rs/reqwest/latest/reqwest/)
*   **Logging**: [tracing](https://docs.rs/tracing/latest/tracing/)

## Setup Instructions

### Prerequisites

1.  **Rust Toolchain**: Install Rust using `rustup`:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
    Ensure you have the latest stable Rust version.

2.  **PostgreSQL**: Have a PostgreSQL instance running. You can use Docker for a quick setup:
    ```bash
    docker run --name va-gateway-postgres -e POSTGRES_USER=user -e POSTGRES_PASSWORD=password -e POSTGRES_DB=va_gateway_db -p 5432:5432 -d postgres:15
    ```

### Environment Variables

Create a `.env` file in the project root based on `.env.example`:

```ini
# .env
SERVER_PORT=8080
DATABASE_URL=postgres://user:password@localhost:5432/va_gateway_db
UPSTREAM_SERVICE_URL=http://localhost:3000 # Example: URL of a service to proxy to
```

*   `SERVER_PORT`: The port on which the gateway will listen.
*   `DATABASE_URL`: Connection string for your PostgreSQL database.
*   `UPSTREAM_SERVICE_URL`: The base URL of the service to which requests will be proxied.

### Database Migrations (Optional)

If you plan to use SQLx migrations, you'll need `sqlx-cli`:

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

Then, create a `migrations` directory and add your migration files (e.g., `V1__initial.sql`).

```sql
-- migrations/V1__initial.sql
CREATE TABLE IF NOT EXISTS example_data (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);
```

Run migrations:

```bash
DATABASE_URL=postgres://user:password@localhost:5432/va_gateway_db sqlx migrate run
```

## Building and Running

### Build

```bash
cargo build
```

For a release build:

```bash
cargo build --release
```

### Run

```bash
cargo run
```

Or, if you built a release version:

```bash
./target/release/va-novo-projeto-va-gateway--api-gateway-em-rust
```

The server will start on the configured `SERVER_PORT` (default: `8080`).

## Usage Examples

Assuming the gateway is running on `http://localhost:8080` and `UPSTREAM_SERVICE_URL` is `http://localhost:3000`.

1.  **Health Check**:
    ```bash
    curl http://localhost:8080/health
    # Expected output: OK
    ```

2.  **Proxying a request** (e.g., to `/api/users` on the upstream service):
    ```bash
    # Assuming an upstream service is running on localhost:3000/users
    curl http://localhost:8080/api/users
    ```
    This request will be forwarded to `http://localhost:3000/users`.

3.  **Database Interaction Example** (if `get_example_data` is enabled):
    ```bash
    curl http://localhost:8080/api/data
    # Expected output: Database returned: 1 (or similar, based on DB query)
    ```

## Testing

To run the tests:

```bash
cargo test
```

Note: Integration tests require a PostgreSQL database to be running and accessible via the `DATABASE_URL` (it will create a `_test` suffix database).

## CI/CD

See `.github/workflows/ci.yml` for GitHub Actions configuration, including building, testing, and linting.

## Docker

Build the Docker image:

```bash
docker build -t va-gateway .
```

Run the Docker image:

```bash
docker run -p 8080:8080 -e DATABASE_URL="postgres://user:password@host.docker.internal:5432/va_gateway_db" -e UPSTREAM_SERVICE_URL="http://host.docker.internal:3000" va-gateway
```

*   Remember to replace `host.docker.internal` with the actual IP or hostname if not running on Docker Desktop.
*   Ensure your PostgreSQL and upstream services are accessible from within the Docker container.
