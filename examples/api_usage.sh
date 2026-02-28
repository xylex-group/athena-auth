#!/bin/bash

# Example API usage script for athena-auth

API_URL="${API_URL:-http://localhost:3000}"

echo "=== Athena Auth API Example Usage ==="
echo ""

# Register a new user
echo "1. Registering a new user..."
REGISTER_RESPONSE=$(curl -s -X POST "$API_URL/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "securepassword123"
  }')
echo "Response: $REGISTER_RESPONSE"
echo ""

# Login
echo "2. Logging in..."
LOGIN_RESPONSE=$(curl -s -X POST "$API_URL/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "securepassword123"
  }')
echo "Response: $LOGIN_RESPONSE"
echo ""

# Extract access token
ACCESS_TOKEN=$(echo "$LOGIN_RESPONSE" | grep -o '"access_token":"[^"]*' | cut -d'"' -f4)

if [ -z "$ACCESS_TOKEN" ]; then
  echo "Failed to get access token. Exiting."
  exit 1
fi

echo "Access Token: $ACCESS_TOKEN"
echo ""

# Verify token
echo "3. Verifying token..."
VERIFY_RESPONSE=$(curl -s -X GET "$API_URL/auth/verify" \
  -H "Authorization: Bearer $ACCESS_TOKEN")
echo "Response: $VERIFY_RESPONSE"
echo ""

# Get current user info
echo "4. Getting current user info..."
ME_RESPONSE=$(curl -s -X GET "$API_URL/auth/me" \
  -H "Authorization: Bearer $ACCESS_TOKEN")
echo "Response: $ME_RESPONSE"
echo ""

# Create an API key
echo "5. Creating an API key..."
API_KEY_RESPONSE=$(curl -s -X POST "$API_URL/auth/api-keys" \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "My Test API Key"
  }')
echo "Response: $API_KEY_RESPONSE"
echo ""

# Extract API key ID
API_KEY_ID=$(echo "$API_KEY_RESPONSE" | grep -o '"id":"[^"]*' | cut -d'"' -f4)

# List API keys
echo "6. Listing API keys..."
LIST_KEYS_RESPONSE=$(curl -s -X GET "$API_URL/auth/api-keys" \
  -H "Authorization: Bearer $ACCESS_TOKEN")
echo "Response: $LIST_KEYS_RESPONSE"
echo ""

# Revoke API key
if [ -n "$API_KEY_ID" ]; then
  echo "7. Revoking API key..."
  REVOKE_RESPONSE=$(curl -s -X DELETE "$API_URL/auth/api-keys/$API_KEY_ID" \
    -H "Authorization: Bearer $ACCESS_TOKEN" \
    -w "\nHTTP Status: %{http_code}")
  echo "Response: $REVOKE_RESPONSE"
  echo ""
fi

echo "=== Example Usage Complete ==="
