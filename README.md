# Blog API - Production-Grade REST API in Rust

A production-ready blog API built with Rust, featuring user authentication, posts, comments, and likes functionality.

## Features

- ✅ User authentication (JWT-based)
- ✅ Role-based access control (Admin, Moderator, User)
- ✅ CRUD operations for users, posts, and comments
- ✅ Post likes/unlikes
- ✅ Nested comments support
- ✅ Soft deletes for posts and comments
- ✅ Pagination and filtering
- ✅ Input validation
- ✅ Comprehensive error handling
- ✅ Structured logging with tracing
- ✅ Database migrations
- ✅ Docker support
- ✅ CI/CD with GitHub Actions
- ✅ Integration and unit tests

## Tech Stack

- **Framework**: Actix-web 4.x
- **Database**: PostgreSQL with SQLx
- **Authentication**: JWT + Argon2 password hashing
- **Validation**: Validator
- **Logging**: Tracing + Tracing-subscriber
- **Testing**: Actix-rt

## Prerequisites

- Rust 1.75+ ([Install Rust](https://rustup.rs/))
- PostgreSQL 15+ ([Install PostgreSQL](https://www.postgresql.org/download/))
- Docker & Docker Compose (optional, for containerized setup)

## Getting Started

### 1. Clone the Repository

```bash
git clone <repository-url>
cd blog-api
```

### 2. Set Up Environment Variables

```bash
cp .env.example .env
```

Edit `.env` and configure your database credentials and JWT secret:

```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/blog_api_dev
TEST_DATABASE_URL=postgres://postgres:postgres@localhost:5432/blog_api_test
APP__JWT__SECRET=your-super-secret-key-change-this
```

### 3. Set Up Database

#### Option A: Using Docker Compose (Recommended for Development)

```bash
# Start PostgreSQL containers
docker-compose -f docker-compose.dev.yml up -d

# Wait for databases to be ready
sleep 5
```

#### Option B: Manual PostgreSQL Setup

```bash
# Create databases
createdb blog_api_dev
createdb blog_api_test
```

### 4. Install SQLx CLI

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

### 5. Run Migrations

```bash
sqlx migrate run
```

### 6. Build and Run

```bash
# Development mode
cargo run

# Production mode
cargo build --release
./target/release/blog-api
```

The API will be available at `http://localhost:8080`

## Running with Docker

### Development

```bash
# Start only PostgreSQL
docker-compose -f docker-compose.dev.yml up -d

# Run app locally
cargo run
```

### Production

```bash
# Build and start all services
docker-compose up --build -d

# View logs
docker-compose logs -f app

# Stop services
docker-compose down
```

## API Endpoints

### Authentication

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| POST | `/api/auth/register` | Register new user | No |
| POST | `/api/auth/login` | Login user | No |

### Users

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| GET | `/api/users` | List all users | Yes (Admin/Moderator) |
| GET | `/api/users/me` | Get current user | Yes |
| GET | `/api/users/:id` | Get user by ID | Yes |
| PUT | `/api/users/me` | Update current user | Yes |
| PUT | `/api/users/:id` | Update user | Yes (Admin) |
| DELETE | `/api/users/:id` | Delete user | Yes (Owner/Admin) |

### Posts

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| GET | `/api/public/posts` | List posts (public) | No |
| GET | `/api/public/posts/:id` | Get post (public) | No |
| GET | `/api/posts` | List posts | Yes |
| POST | `/api/posts` | Create post | Yes |
| GET | `/api/posts/:id` | Get post | Yes |
| PUT | `/api/posts/:id` | Update post | Yes (Owner/Moderator/Admin) |
| DELETE | `/api/posts/:id` | Delete post | Yes (Owner/Admin) |
| POST | `/api/posts/:id/like` | Like post | Yes |
| DELETE | `/api/posts/:id/unlike` | Unlike post | Yes |

### Comments

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| GET | `/api/posts/:post_id/comments` | List comments | Yes |
| POST | `/api/posts/:post_id/comments` | Create comment | Yes |
| GET | `/api/posts/:post_id/comments/:id` | Get comment | Yes |
| PUT | `/api/posts/:post_id/comments/:id` | Update comment | Yes (Owner/Moderator/Admin) |
| DELETE | `/api/posts/:post_id/comments/:id` | Delete comment | Yes (Owner/Admin) |

### Health Check

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| GET | `/health` | API health check | No |

## Example API Usage

### Register a New User

```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "johndoe",
    "email": "john@example.com",
    "password": "SecurePass123"
  }'
```

**Response:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "johndoe",
    "email": "john@example.com",
    "bio": null,
    "avatar_url": null,
    "role": "user",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

### Login

```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john@example.com",
    "password": "SecurePass123"
  }'
```

### Create a Post

```bash
curl -X POST http://localhost:8080/api/posts \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{
    "title": "My First Blog Post",
    "content": "This is the content of my first blog post...",
    "published": true,
    "tags": ["rust", "web-development"]
  }'
```

### List Posts with Filters

```bash
# All posts
curl http://localhost:8080/api/public/posts

# Filter by tag
curl "http://localhost:8080/api/public/posts?tag=rust"

# Search posts
curl "http://localhost:8080/api/public/posts?search=blog"

# Pagination
curl "http://localhost:8080/api/public/posts?page=2&limit=10"

# Sort by likes
curl "http://localhost:8080/api/public/posts?sort_by=likes_count&order=desc"
```

### Create a Comment

```bash
curl -X POST http://localhost:8080/api/posts/{post_id}/comments \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{
    "content": "Great post! Thanks for sharing."
  }'
```

### Reply to a Comment

```bash
curl -X POST http://localhost:8080/api/posts/{post_id}/comments \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{
    "content": "Thanks for your comment!",
    "parent_id": "parent-comment-uuid"
  }'
```

### Like a Post

```bash
curl -X POST http://localhost:8080/api/posts/{post_id}/like \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

## Testing

### Run All Tests

```bash
cargo test
```

### Run Specific Test Suite

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test '*'

# Specific test
cargo test test_user_registration_success
```

### Run Tests with Coverage

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --verbose --all-features --workspace --timeout 120
```

## Project Structure

```
blog-api/
├── src/
│   ├── main.rs                 # Application entry point
│   ├── config/
│   │   └── mod.rs             # Configuration management
│   ├── models/
│   │   ├── mod.rs
│   │   ├── user.rs            # User models and DTOs
│   │   ├── post.rs            # Post models and DTOs
│   │   ├── comment.rs         # Comment models and DTOs
│   │   └── common.rs          # Common models (pagination, etc.)
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── auth.rs            # Authentication handlers
│   │   ├── user.rs            # User handlers
│   │   ├── post.rs            # Post handlers
│   │   └── comment.rs         # Comment handlers
│   ├── services/
│   │   ├── mod.rs
│   │   ├── password.rs        # Password hashing service
│   │   └── jwt.rs             # JWT service
│   ├── middleware/
│   │   ├── mod.rs
│   │   └── auth.rs            # Authentication middleware
│   ├── db/
│   │   ├── mod.rs             # Database connection
│   │   └── repository/
│   │       ├── mod.rs
│   │       ├── user.rs        # User database operations
│   │       ├── post.rs        # Post database operations
│   │       └── comment.rs     # Comment database operations
│   ├── error.rs               # Custom error types
│   └── routes.rs              # Route configuration
├── migrations/                 # Database migrations
│   ├── 20240101000001_create_users_table.sql
│   ├── 20240101000002_create_posts_table.sql
│   ├── 20240101000003_create_comments_table.sql
│   └── 20240101000004_create_likes_table.sql
├── tests/
│   ├── common/
│   │   └── mod.rs             # Test utilities
│   └── integration/
│       └── auth_tests.rs      # Integration tests
├── config/
│   ├── default.toml
│   ├── development.toml
│   └── production.toml
├── .github/
│   └── workflows/
│       └── ci.yml             # CI/CD pipeline
├── Cargo.toml
├── Dockerfile
├── docker-compose.yml
├── docker-compose.dev.yml
├── .env.example
├── .gitignore
└── README.md
```

## Database Schema

### Users Table
```sql
- id: UUID (PK)
- username: VARCHAR(50) UNIQUE
- email: VARCHAR(255) UNIQUE
- password_hash: VARCHAR(255)
- bio: TEXT
- avatar_url: VARCHAR(500)
- role: VARCHAR(20)
- created_at: TIMESTAMP
- updated_at: TIMESTAMP
```

### Posts Table
```sql
- id: UUID (PK)
- user_id: UUID (FK -> users.id)
- title: VARCHAR(200)
- content: TEXT
- slug: VARCHAR(250) UNIQUE
- published: BOOLEAN
- tags: TEXT[]
- likes_count: INTEGER
- deleted_at: TIMESTAMP
- created_at: TIMESTAMP
- updated_at: TIMESTAMP
```

### Comments Table
```sql
- id: UUID (PK)
- post_id: UUID (FK -> posts.id)
- user_id: UUID (FK -> users.id)
- content: TEXT
- parent_id: UUID (FK -> comments.id, nullable)
- deleted_at: TIMESTAMP
- created_at: TIMESTAMP
- updated_at: TIMESTAMP
```

### Likes Table
```sql
- id: UUID (PK)
- post_id: UUID (FK -> posts.id)
- user_id: UUID (FK -> users.id)
- created_at: TIMESTAMP
- UNIQUE(post_id, user_id)
```

## Configuration

The application uses a layered configuration system:

1. **Default configuration**: `config/default.toml`
2. **Environment-specific**: `config/{environment}.toml`
3. **Environment variables**: Prefixed with `APP__`

Priority: Environment variables > Environment-specific config > Default config

### Environment Variables

```bash
# Server
APP__SERVER__HOST=0.0.0.0
APP__SERVER__PORT=8080

# Database
APP__DATABASE__URL=postgres://user:pass@localhost/dbname
APP__DATABASE__MAX_CONNECTIONS=20

# JWT
APP__JWT__SECRET=your-secret-key
APP__JWT__EXPIRATION_HOURS=24

# Environment
RUN_MODE=production
```

## Security Features

- **Password Hashing**: Argon2 algorithm for secure password storage
- **JWT Authentication**: Token-based authentication with expiration
- **Role-Based Access Control**: Admin, Moderator, and User roles
- **Input Validation**: Comprehensive validation on all inputs
- **SQL Injection Prevention**: Parameterized queries via SQLx
- **CORS Configuration**: Configurable CORS policies
- **Security Headers**: Automatic security headers in production

## Performance Optimizations

- **Connection Pooling**: Efficient database connection management
- **Async/Await**: Non-blocking I/O throughout the application
- **Database Indexes**: Optimized queries with proper indexing
- **Denormalized Counters**: Like counts stored for quick access
- **Pagination**: Efficient data retrieval with offset/limit
- **Soft Deletes**: Quick deletion without cascading operations

## Development Workflow

### Creating Database Migrations

```bash
# Create a new migration
sqlx migrate add migration_name

# Edit the generated file in migrations/
# Then run migrations
sqlx migrate run

# Revert last migration (if needed)
sqlx migrate revert
```

### Code Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

### Linting

```bash
# Run clippy
cargo clippy

# Fix clippy warnings
cargo clippy --fix
```

### Pre-commit Checks

```bash
# Run all checks before committing
cargo fmt -- --check && \
cargo clippy -- -D warnings && \
cargo test
```

## Troubleshooting

### Database Connection Issues

```bash
# Check if PostgreSQL is running
pg_isadmin

# Test connection
psql -U postgres -d blog_api_dev

# Reset database
dropdb blog_api_dev
createdb blog_api_dev
sqlx migrate run
```

### Migration Errors

```bash
# Check migration status
sqlx migrate info

# Force migration
sqlx migrate run --ignore-missing

# Start fresh
sqlx database drop
sqlx database create
sqlx migrate run
```

### Docker Issues

```bash
# Clean up containers
docker-compose down -v

# Rebuild from scratch
docker-compose build --no-cache

# View logs
docker-compose logs -f app
```

## Production Deployment

### Environment Setup

1. Set strong JWT secret
2. Configure production database
3. Enable HTTPS
4. Set up monitoring and logging
5. Configure backup strategy

### Deployment Checklist

- [ ] Update `APP__JWT__SECRET` with a strong random key
- [ ] Set `RUN_MODE=production`
- [ ] Configure production database credentials
- [ ] Enable HTTPS/TLS
- [ ] Set up reverse proxy (nginx/traefik)
- [ ] Configure monitoring (Prometheus/Grafana)
- [ ] Set up log aggregation
- [ ] Enable automated backups
- [ ] Set resource limits (CPU/memory)
- [ ] Configure firewall rules

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Standards

- Follow Rust naming conventions
- Write tests for new features
- Update documentation
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes
- Add appropriate tracing/logging

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For issues, questions, or contributions, please open an issue on GitHub.

## Roadmap

- [ ] File upload functionality
- [ ] Email notifications
- [ ] Rate limiting
- [ ] Caching layer (Redis)
- [ ] Full-text search (Elasticsearch)
- [ ] GraphQL API
- [ ] WebSocket support for real-time features
- [ ] Admin dashboard
- [ ] API documentation (Swagger/OpenAPI)

---

**Built with ❤️ using Rust and Actix-web**