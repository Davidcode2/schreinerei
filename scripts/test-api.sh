#!/bin/bash
# API Test Script for Schreinerei Backend
# Usage: ./scripts/test-api.sh

set -e

BASE_URL="${BASE_URL:-http://localhost:3000}"
KEYCLOAK_URL="${KEYCLOAK_URL:-https://auth.jakob-lingel.dev}"
REALM="${KEYCLOAK_REALM:-schreinerei}"
CLIENT_ID="${KEYCLOAK_CLIENT_ID:-schreinerei_pwa}"
CLIENT_SECRET="${KEYCLOAK_CLIENT_SECRET:-TG776i8yxgKVuFPakoOpQZDWMHRycYkG}"

# Test user credentials (create this user in Keycloak)
TEST_USER="${TEST_USER:-test@schreinerei.test}"
TEST_PASSWORD="${TEST_PASSWORD:-test123}"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}=== Schreinerei API Test Script ===${NC}"
echo ""

# Function to get JWT token
get_token() {
    echo -e "${YELLOW}Getting JWT token from Keycloak...${NC}"
    
    TOKEN_RESPONSE=$(curl -s -X POST \
        "${KEYCLOAK_URL}/realms/${REALM}/protocol/openid-connect/token" \
        -H "Content-Type: application/x-www-form-urlencoded" \
        -d "grant_type=password" \
        -d "client_id=${CLIENT_ID}" \
        -d "client_secret=${CLIENT_SECRET}" \
        -d "username=${TEST_USER}" \
        -d "password=${TEST_PASSWORD}")
    
    ACCESS_TOKEN=$(echo "$TOKEN_RESPONSE" | jq -r '.access_token // empty')
    
    if [ -z "$ACCESS_TOKEN" ] || [ "$ACCESS_TOKEN" = "null" ]; then
        echo -e "${RED}Failed to get token${NC}"
        echo "Response: $TOKEN_RESPONSE"
        exit 1
    fi
    
    echo -e "${GREEN}✓ Token obtained${NC}"
    echo ""
}

# Function to test endpoint
test_endpoint() {
    local method=$1
    local path=$2
    local data=$3
    local expected_status=$4
    
    echo -e "${YELLOW}Testing: $method $path${NC}"
    
    if [ -n "$data" ]; then
        RESPONSE=$(curl -s -w "\n%{http_code}" -X "$method" \
            "${BASE_URL}${path}" \
            -H "Authorization: Bearer $ACCESS_TOKEN" \
            -H "Content-Type: application/json" \
            -d "$data")
    else
        RESPONSE=$(curl -s -w "\n%{http_code}" -X "$method" \
            "${BASE_URL}${path}" \
            -H "Authorization: Bearer $ACCESS_TOKEN")
    fi
    
    HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
    BODY=$(echo "$RESPONSE" | sed '$d')
    
    if [ "$HTTP_CODE" = "$expected_status" ]; then
        echo -e "${GREEN}✓ Status: $HTTP_CODE${NC}"
        if [ -n "$BODY" ] && [ "$BODY" != "null" ]; then
            echo "$BODY" | jq '.' 2>/dev/null || echo "$BODY"
        fi
    else
        echo -e "${RED}✗ Expected $expected_status, got $HTTP_CODE${NC}"
        echo "$BODY"
    fi
    echo ""
}

# Health check (no auth required)
echo -e "${YELLOW}=== Health Check ===${NC}"
curl -s "${BASE_URL}/health" | jq '.'
echo ""

# Get token for authenticated tests
get_token

echo -e "${YELLOW}=== IAM Endpoints ===${NC}"

# Get current user
test_endpoint "GET" "/api/v1/auth/me" "" "200"

# List users
test_endpoint "GET" "/api/v1/users" "" "200"

echo -e "${YELLOW}=== Inventory Endpoints ===${NC}"

# Create category
CATEGORY_RESPONSE=$(curl -s -X POST \
    "${BASE_URL}/api/v1/inventory/categories" \
    -H "Authorization: Bearer $ACCESS_TOKEN" \
    -H "Content-Type: application/json" \
    -d '{"name": "Platten", "description": "Plattenmaterial"}')
CATEGORY_ID=$(echo "$CATEGORY_RESPONSE" | jq -r '.id')
echo -e "${GREEN}✓ Created category: $CATEGORY_ID${NC}"
echo "$CATEGORY_RESPONSE" | jq '.'
echo ""

# List categories
test_endpoint "GET" "/api/v1/inventory/categories" "" "200"

# Create material
MATERIAL_RESPONSE=$(curl -s -X POST \
    "${BASE_URL}/api/v1/inventory/materials" \
    -H "Authorization: Bearer $ACCESS_TOKEN" \
    -H "Content-Type: application/json" \
    -d "{\"category_id\": \"$CATEGORY_ID\", \"name\": \"MDF 19mm\", \"description\": \"MDF Platte 19mm\", \"unit\": \"stück\", \"quantity\": 50, \"min_quantity\": 5, \"location\": \"Regal A1\"}")
MATERIAL_ID=$(echo "$MATERIAL_RESPONSE" | jq -r '.id')
echo -e "${GREEN}✓ Created material: $MATERIAL_ID${NC}"
echo "$MATERIAL_RESPONSE" | jq '.'
echo ""

# List materials
test_endpoint "GET" "/api/v1/inventory/materials" "" "200"

# Get single material
test_endpoint "GET" "/api/v1/inventory/materials/$MATERIAL_ID" "" "200"

# Withdraw material
test_endpoint "POST" "/api/v1/inventory/materials/$MATERIAL_ID/withdraw" \
    '{"quantity": 5, "notes": "Test withdrawal"}' "200"

# Check low stock (should be none)
test_endpoint "GET" "/api/v1/inventory/low-stock" "" "200"

# Generate QR code
test_endpoint "POST" "/api/v1/inventory/materials/$MATERIAL_ID/qr" "" "200"

# Get QR SVG
test_endpoint "GET" "/api/v1/inventory/materials/$MATERIAL_ID/qr/svg" "" "200"

echo -e "${YELLOW}=== Order Request Flow ===${NC}"

# Create order request
ORDER_RESPONSE=$(curl -s -X POST \
    "${BASE_URL}/api/v1/inventory/orders" \
    -H "Authorization: Bearer $ACCESS_TOKEN" \
    -H "Content-Type: application/json" \
    -d "{\"material_id\": \"$MATERIAL_ID\", \"quantity\": 20, \"reason\": \"Bestand aufgebracht\"}")
ORDER_ID=$(echo "$ORDER_RESPONSE" | jq -r '.id')
echo -e "${GREEN}✓ Created order request: $ORDER_ID${NC}"
echo "$ORDER_RESPONSE" | jq '.'
echo ""

# List order requests
test_endpoint "GET" "/api/v1/inventory/orders" "" "200"

# Approve order request
test_endpoint "POST" "/api/v1/inventory/orders/$ORDER_ID/approve" \
    '{"notes": "Approved for next delivery"}' "200"

# Fulfill order request
test_endpoint "POST" "/api/v1/inventory/orders/$ORDER_ID/fulfill" \
    '{"actual_quantity": 20, "notes": "Delivered"}' "200"

echo -e "${GREEN}=== All tests completed ===${NC}"
