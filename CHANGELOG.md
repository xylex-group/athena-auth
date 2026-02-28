# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-02-28

### Added
- Initial release of athena-auth
- User registration endpoint (`POST /auth/register`)
- User login endpoint with JWT tokens (`POST /auth/login`)
- Token verification endpoint (`GET /auth/verify`)
- Get current user endpoint (`GET /auth/me`)
- API key creation endpoint (`POST /auth/api-keys`)
- API key listing endpoint (`GET /auth/api-keys`)
- API key revocation endpoint (`DELETE /auth/api-keys/:id`)
- Password hashing using bcrypt
- JWT token generation and validation with 24-hour expiration
- Database integration using athena_rs v0.82.2
- Support for multiple database backends (PostgreSQL, Supabase, ScyllaDB)
- CORS middleware
- Tracing/logging middleware
- Docker and Docker Compose support
- PostgreSQL database schema with proper indexes
- Example API usage script
- Comprehensive README with API documentation
- Environment variable configuration via .env file
- Security documentation (SECURITY.md)
- Contributing guidelines (CONTRIBUTING.md)

### Security
- All passwords hashed with bcrypt before storage
- All API keys hashed with bcrypt before storage
- JWT_SECRET environment variable required (no default fallback)
- Secure JWT token generation with expiration
- Protected routes require valid JWT authentication

[0.1.0]: https://github.com/xylex-group/athena-auth/releases/tag/v0.1.0
