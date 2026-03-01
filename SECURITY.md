# Security Policy

## Security Features

This project implements several security best practices:

### Authentication & Authorization

- **Password Hashing**: All passwords are hashed using bcrypt with the default cost factor (12)
- **JWT Tokens**: JSON Web Tokens are used for stateless authentication with 24-hour expiration
- **API Key Hashing**: API keys are hashed using bcrypt before storage, only the plaintext key is returned once at creation
- **Authorization Middleware**: Protected routes require valid JWT tokens in the Authorization header

### Environment Variables

The following environment variables **MUST** be set for secure operation:

- `JWT_SECRET`: A strong, random secret key for JWT signing and verification. The application will **fail to start** if this is not set.
- `DATABASE_URL`: Connection string to your database
- `DATABASE_KEY`: Authentication key for the database (if required)

### Security Considerations

1. **JWT_SECRET**: 
   - Must be a strong, random string (at least 32 characters recommended)
   - Should be different for each environment (dev, staging, production)
   - Should never be committed to version control
   - Should be rotated periodically

2. **Database Credentials**:
   - Keep database credentials secure
   - Use environment variables, never hardcode in source
   - For production, use secrets management systems (AWS Secrets Manager, HashiCorp Vault, etc.)

3. **Password Policy**:
   - The API does not enforce password complexity requirements - this should be implemented client-side or as middleware
   - Consider implementing rate limiting on login attempts to prevent brute force attacks

4. **API Keys**:
   - API keys are only shown once at creation time
   - Store the plaintext key securely - it cannot be retrieved later
   - Regularly rotate API keys
   - Implement expiration times for API keys when creating them

5. **CORS**:
   - Currently configured to allow all origins (`Any`)
   - For production, configure specific allowed origins in the CORS layer

### Reporting Security Issues

If you discover a security vulnerability, please report it by:
1. **DO NOT** open a public GitHub issue
2. Email the maintainers directly at security@xylex-group.com
3. Include detailed information about the vulnerability and steps to reproduce

## Recommended Production Configuration

```bash
# Strong JWT secret (generate with: openssl rand -base64 32)
JWT_SECRET=<your-secure-random-string>

# Database connection with TLS
DATABASE_URL=postgresql://user:password@host:5432/db?sslmode=require

# Bind to localhost if behind a reverse proxy
PORT=3000

# Enable additional security headers in your reverse proxy (nginx, Apache, etc.)
```

## Security Checklist for Deployment

- [ ] Set a strong, unique `JWT_SECRET`
- [ ] Use TLS/SSL for database connections
- [ ] Enable HTTPS for the API endpoint
- [ ] Configure CORS to allow only trusted origins
- [ ] Implement rate limiting (e.g., using nginx or a service like Cloudflare)
- [ ] Set up monitoring and logging for security events
- [ ] Regularly update dependencies (`cargo update`)
- [ ] Use a secrets management system for production
- [ ] Implement password complexity requirements
- [ ] Add account lockout after failed login attempts
- [ ] Consider implementing 2FA/MFA for additional security
