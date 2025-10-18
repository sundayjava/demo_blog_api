#!/bin/bash

# Create config directory if it doesn't exist
mkdir -p config

echo "Creating config/default.toml..."
cat > config/default.toml << 'EOF'
environment = "development"

[server]
host = "127.0.0.1"
port = 8080

[database]
url = "postgres://postgres:postgres@localhost:5432/blog_api_dev"
max_connections = 5

[jwt]
secret = "dev_secret_key_change_in_production"
expiration_hours = 24
EOF

echo "Creating config/production.toml..."
cat > config/production.toml << 'EOF'
environment = "production"

[server]
host = "0.0.0.0"
port = 8080

[database]
max_connections = 20
EOF

echo "✅ Configuration files created successfully!"
echo ""
echo "Files created:"
ls -lh config/

echo ""
echo "Content of config/default.toml:"
cat config/default.toml

echo ""
echo "Content of config/production.toml:"
cat config/production.toml