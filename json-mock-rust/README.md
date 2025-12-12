# JSON Mock Server (Rust + MongoDB)

A generic JSON storage service rewritten in Rust using MongoDB.

## Features

- **Schema-less Storage**: Store any JSON data without predefined schema
- **RESTful API**: Full CRUD operations compatible with the original Node.js version
- **MongoDB Backend**: Scalable document storage
- **CORS Enabled**: Allow cross-origin requests
- **Production Mode**: Block write operations in production environment

## Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- MongoDB 4.4+ (install locally or use Docker)

## Quick Start

1. **Start MongoDB** (using Docker):
   ```bash
   docker run -d --name mongodb -p 27017:27017 mongo:latest
   ```

2. **Configure environment**:
   ```bash
   cp .env.example .env
   # Edit .env if needed
   ```

3. **Build and run**:
   ```bash
   cargo build --release
   cargo run --release
   ```

   Server starts at `http://0.0.0.0:3000`

## API Endpoints

### Posts

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/posts/:id` | Get post by ID |
| POST | `/posts` | Create new post (auto-generates ID) |
| PUT | `/posts` | Update post (requires `id` in body) |
| DELETE | `/posts/:id` | Delete post by ID |

### Forms

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/form` | Get all forms |
| GET | `/form?id=123` | Get form by ID |
| POST | `/form/:id` | Create/update form |
| DELETE | `/form/:id` | Delete form |

### Generic Data

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/data` | Batch update root-level keys |
| GET | `/data/:name` | Get all from collection |
| GET | `/data/:name?id=123` | Get by ID from collection |
| POST | `/data/:name` | Insert into collection |
| DELETE | `/data/:name` | Delete all from collection |
| DELETE | `/data/:name?id=123` | Delete by ID from collection |

### Any Collection

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/:name` | Get collection or global value |
| GET | `/:name?id=123` | Get by ID from collection |

## Response Format

All responses follow this format:

```json
{
  "code": 200,
  "data": { ... }
}
```

Error responses:

```json
{
  "code": 500,
  "msg": "error message"
}
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `MONGODB_URI` | `mongodb://localhost:27017` | MongoDB connection string |
| `DB_NAME` | `json_mock` | Database name |
| `ENVIRONMENT` | `development` | Set to `production` to disable writes |
| `RUST_LOG` | `info` | Log level |

## Running Tests

```bash
# Unit tests
cargo test

# With logging
RUST_LOG=debug cargo test -- --nocapture
```

## Docker

### Local Build (Optimized)

```bash
# Build with BuildKit for faster builds
export DOCKER_BUILDKIT=1
docker build -t json-mock-rust .
```

### Deploy to CI Server

Use the `sync-to-ci.sh` script for deployment:

```bash
# Sync files only
./sync-to-ci.sh

# Sync + build + push (one command)
./sync-to-ci.sh n105 /tmp/json-mock-rust --deploy

# Custom registry
./sync-to-ci.sh n105 /tmp/json-mock-rust --deploy \
  --registry=docker.io \
  --image=username/json-mock-rust \
  --tag=v1.0.0
```

**Available parameters:**
- `--deploy` - Build and push Docker image after sync
- `--registry=URL` - Docker registry URL (default: `registry1.cdnline.cn:5000`)
- `--image=NAME` - Docker image name (default: `pageconfig/json-mock`)
- `--tag=TAG` - Docker image tag (default: `rust`)

### Docker Compose

```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```
