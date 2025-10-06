# Docker Usage Guide

## Building the Image

```bash
# Build the image
docker build -t agsi:latest .

# The image will be ~50MB (minimal Debian + Rust binary)
```

## Running Commands

### Validate Files

```bash
docker run --rm -v $(pwd)/data:/data agsi:latest validate /data/model.agsi.json
```

### Create Documents

```bash
docker run --rm -v $(pwd)/data:/data agsi:latest create document --id DOC001 --output /data/doc.json
```

### Run MCP Server

```bash
docker run --rm -i agsi:latest mcp
# Or with docker-compose
docker-compose up agsi-mcp
```

### Extract Materials

```bash
docker run --rm -v $(pwd)/data:/data agsi:latest extract /data/model.json --output /data/materials.json
```

## Docker Compose

Use `docker-compose.yml` for multi-service setup:

```bash
# Start all services
docker-compose up

# Run specific service
docker-compose run agsi-cli validate /data/model.json

# Run MCP server
docker-compose up agsi-mcp
```

## Container Size

- Builder stage: ~2GB (includes Rust toolchain)
- Runtime stage: ~50MB (minimal Debian + binary)
- Final image: ~50MB

## Tips

- Mount your data directory: `-v $(pwd)/data:/data`
- Use `--rm` to auto-remove containers
- Interactive mode for MCP: `-i` flag
- Detached mode: `-d` flag

## Example Workflow

```bash
# Build image
docker build -t agsi:latest .

# Create project directory
mkdir my-project
cd my-project

# Create document
docker run --rm -v $(pwd):/data agsi:latest create document --id PROJ001 --output /data/project.json

# Validate
docker run --rm -v $(pwd):/data agsi:latest validate /data/project.json

# View stats
docker run --rm -v $(pwd):/data agsi:latest stats /data/project.json
```
