# Contributing to athena-auth

Thank you for considering contributing to athena-auth! This document provides guidelines and instructions for contributing.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/athena-auth.git`
3. Create a feature branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Test your changes: `cargo test && cargo check`
6. Commit your changes: `git commit -am 'Add some feature'`
7. Push to the branch: `git push origin feature/your-feature-name`
8. Create a Pull Request

## Development Setup

### Prerequisites

- Rust 1.93.1 or later
- PostgreSQL (or Docker)
- Git

### Local Development

1. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```

2. Update `.env` with your local configuration

3. Start a PostgreSQL database (or use Docker Compose):
   ```bash
   docker-compose up -d postgres
   ```

4. Run database migrations:
   ```bash
   psql -d athena_auth -f schema.sql
   ```

5. Run the development server:
   ```bash
   cargo run
   ```

## Code Style

- Follow Rust standard formatting: `cargo fmt`
- Check for common mistakes: `cargo clippy`
- Ensure code compiles without warnings: `cargo check`

## Testing

Currently, the project doesn't have comprehensive tests. Contributions to add tests are welcome!

When adding new features, please include:
- Unit tests for business logic
- Integration tests for API endpoints

Run tests with:
```bash
cargo test
```

## Pull Request Guidelines

- Keep pull requests focused on a single feature or fix
- Write clear, descriptive commit messages
- Update documentation (README, comments) as needed
- Ensure all tests pass
- Follow the existing code style
- Add tests for new functionality

## Reporting Bugs

When reporting bugs, please include:

- A clear, descriptive title
- Detailed steps to reproduce the issue
- Expected behavior
- Actual behavior
- Your environment (OS, Rust version, etc.)
- Relevant logs or error messages

## Feature Requests

We welcome feature requests! Please:

- Check if the feature has already been requested
- Clearly describe the feature and its use case
- Explain why this feature would be useful
- Consider submitting a pull request to implement it

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers and help them get started
- Focus on constructive criticism
- Respect differing opinions and experiences

## Questions?

Feel free to open an issue with the "question" label if you have questions about the project.

## License

By contributing to athena-auth, you agree that your contributions will be licensed under the MIT License.
