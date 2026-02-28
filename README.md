# athena-auth

A Rust-based authentication API built with Axum and athena_rs for database operations.

## Features

- **User Authentication**: Register, login, and JWT-based authentication
- **API Key Management**: Create, list, and revoke API keys
- **Secure Password Handling**: Bcrypt password hashing
- **JWT Tokens**: JSON Web Token authentication with configurable expiration
- **Database Abstraction**: Uses athena_rs for database operations (supports PostgreSQL, Supabase, ScyllaDB)

## API Endpoints

### Public Routes

- `POST /auth/register` - Register a new user
  ```json
  {
    "email": "user@example.com",
    "password": "secure_password"
  }
  ```

- `POST /auth/login` - Login and receive JWT token
  ```json
  {
    "email": "user@example.com",
    "password": "secure_password"
  }
  ```
  Response:
  ```json
  {
    "access_token": "eyJ...",
    "token_type": "Bearer",
    "expires_in": 86400
  }
  ```

### Protected Routes (Require Bearer Token)

- `GET /auth/verify` - Verify JWT token validity
  - Headers: `Authorization: Bearer <token>`

- `GET /auth/me` - Get current user information
  - Headers: `Authorization: Bearer <token>`

- `POST /auth/api-keys` - Create a new API key
  ```json
  {
    "name": "My API Key",
    "expires_at": "2026-12-31T23:59:59Z" // optional
  }
  ```

- `GET /auth/api-keys` - List all API keys for the current user
  - Headers: `Authorization: Bearer <token>`

- `DELETE /auth/api-keys/:id` - Revoke an API key
  - Headers: `Authorization: Bearer <token>`

## Setup

### Prerequisites

- Rust (1.93.1 or later)
- PostgreSQL database (or Supabase/ScyllaDB via athena_rs)

### About athena_rs

This project uses the [athena_rs](https://crates.io/crates/athena_rs) crate (v0.82.2), which is a database gateway API that supports multiple backends:
- **Native PostgreSQL** (via SQLx)
- **Supabase** 
- **ScyllaDB**
- **Neon**

The `DATABASE_URL` should point to your database connection string, and the `DATABASE_KEY` is used for authentication (empty for local PostgreSQL).

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/xylex-group/athena-auth.git
   cd athena-auth
   ```

2. Copy the example environment file and configure:
   ```bash
   cp .env.example .env
   ```

3. Edit `.env` with your configuration:
   ```env
   DATABASE_URL=postgresql://localhost/athena_auth
   DATABASE_KEY=your_database_key_here
   JWT_SECRET=your_jwt_secret_key_here_change_in_production
   PORT=3000
   ```

4. Set up the database schema:
   ```bash
   psql -d athena_auth -f schema.sql
   ```

5. Build and run:
   ```bash
   cargo run
   ```

The server will start on `http://0.0.0.0:3000` (or the port specified in `.env`).

## Database Schema

The application uses two main tables:

- **users**: Stores user accounts with email and hashed passwords
- **api_keys**: Stores API keys associated with users (Note: The problem statement mentioned storing API keys in a "public" or "api_key" table - we use `api_keys` as the standard naming convention)

See `schema.sql` for the complete schema definition.

## Docker Deployment

### Using Docker Compose

The easiest way to run the application with a PostgreSQL database:

```bash
docker-compose up --build
```

This will:
- Start a PostgreSQL database
- Automatically create the required tables using `schema.sql`
- Start the athena-auth API on port 3000

### Building Docker Image Manually

```bash
docker build -t athena-auth .
docker run -p 3000:3000 \
  -e DATABASE_URL=postgresql://localhost/athena_auth \
  -e DATABASE_KEY="" \
  -e JWT_SECRET=your_secret_key \
  athena-auth
```

## Example Usage

An example usage script is provided in `examples/api_usage.sh`:

```bash
chmod +x examples/api_usage.sh
./examples/api_usage.sh
```

This script demonstrates:
1. User registration
2. User login
3. Token verification
4. Getting user information
5. Creating an API key
6. Listing API keys
7. Revoking an API key

## Development

Build the project:
```bash
cargo build
```

Run in development mode:
```bash
cargo run
```

Run with release optimizations:
```bash
cargo build --release
./target/release/athena-auth
```

## Security Notes

- Always use a strong, random `JWT_SECRET` in production
- Store database credentials securely (use environment variables)
- API keys are hashed before storage using bcrypt
- Passwords are hashed using bcrypt with default cost factor
- JWT tokens expire after 24 hours by default

## Technology Stack

- **Framework**: Axum 0.8
- **Database**: athena_rs 0.82.2 (multi-database gateway)
- **Authentication**: JWT (jsonwebtoken), bcrypt
- **Async Runtime**: Tokio
- **Serialization**: Serde

## License

MIT License - see LICENSE file for details
