# Podman Setup for safe-hash-rs

This document provides comprehensive instructions for running safe-hash-rs using Podman containers for enhanced security.

## Prerequisites

- Podman installed on your system
- Basic understanding of container concepts

## Quick Start

1. **Build the container:**
   ```bash
   ./scripts/podman-build.sh
   ```

2. **Run safe-hash:**
   ```bash
   ./scripts/safe-hash-rs help
   ```

## Detailed Usage

### Building the Container

The build script creates a multi-stage container with:
- Rust build environment for compilation
- Minimal Debian runtime environment
- Security hardening (non-root user, minimal dependencies)

```bash
# Build with automatic version detection
./scripts/podman-build.sh

# Or build manually
podman build -t localhost/safe-hash:latest .
```

### Running Commands

#### Basic Usage

```bash
# Show help (use 'help' subcommand to see safe-hash help)
./scripts/safe-hash-rs help

# Verify a transaction
./scripts/safe-hash-rs tx \
  --chain ethereum \
  --nonce 63 \
  --safe-address 0x1c694Fc3006D81ff4a56F97E1b99529066a23725 \
  --safe-version 1.4.1
```

#### Using Local Files

Mount a directory to access local files:

```bash
./scripts/safe-hash-rs -v ./test msg \
  --chain sepolia \
  --safe-address 0x657ff0D4eC65D82b2bC1247b0a558bcd2f80A0f1 \
  --input-file /app/input/test_message.txt \
  --safe-version 1.4.1
```

#### Interactive Mode

For debugging or exploring:

```bash
./scripts/safe-hash-rs -i
```

### Manual Podman Commands

For advanced users who prefer direct control:

```bash
# Basic run
podman run --rm \
  --cap-drop ALL \
  --read-only \
  --tmpfs /tmp \
  --network host \
  localhost/safe-hash:latest --help

# With volume mount
podman run --rm \
  --cap-drop ALL \
  --read-only \
  --tmpfs /tmp \
  --network host \
  -v ./test:/app/input:ro,Z \
  localhost/safe-hash:latest msg \
  --chain sepolia \
  --safe-address 0x657ff0D4eC65D82b2bC1247b0a558bcd2f80A0f1 \
  --input-file /app/input/test_message.txt \
  --safe-version 1.4.1
```

## Security Features

### Container Hardening

- **Non-root execution**: Container runs as unprivileged user (uid 1000)
- **Read-only filesystem**: Container filesystem mounted read-only
- **Capability dropping**: All Linux capabilities dropped (`--cap-drop ALL`)
- **Minimal base**: Uses Debian slim for reduced attack surface
- **Temporary filesystem**: `/tmp` mounted as tmpfs for runtime files

### Network Isolation

By default, the container uses host networking for API access. For enhanced security:

```bash
# No network access (offline mode only)
podman run --rm --network none localhost/safe-hash:latest tx --offline [OPTIONS]

# Custom network
podman network create safe-hash-net
podman run --rm --network safe-hash-net localhost/safe-hash:latest [OPTIONS]
```

### Resource Limits

Add resource constraints for additional security:

```bash
podman run --rm \
  --memory 512m \
  --cpus 1 \
  --pids-limit 100 \
  localhost/safe-hash:latest [OPTIONS]
```

## Troubleshooting

### Common Issues

1. **Permission denied on volume mounts:**
   ```bash
   # Ensure the directory is readable
   chmod +r ./test/*
   
   # Or use SELinux relabeling
   podman run --rm -v ./test:/app/input:ro,Z localhost/safe-hash:latest [OPTIONS]
   ```

2. **Network connectivity issues:**
   ```bash
   # Use host networking
   podman run --rm --network host localhost/safe-hash:latest [OPTIONS]
   ```

3. **Image not found:**
   ```bash
   # Rebuild the image
   ./scripts/podman-build.sh
   ```

### Debugging

Run in interactive mode to debug issues:

```bash
./scripts/safe-hash-rs -i
# Inside container:
safe-hash --help
```

## Best Practices

1. **Always use the latest image**: Rebuild regularly for security updates
2. **Use read-only mounts**: Mount input directories as read-only (`:ro`)
3. **Limit resources**: Set memory and CPU limits for production use
4. **Network isolation**: Use `--offline` mode when possible
5. **Regular updates**: Keep base images updated

## Docker Compatibility

The same setup works with Docker:

```bash
# Build with Docker
docker build -t safe-hash:latest .

# Run with Docker
docker run --rm --cap-drop ALL --read-only --tmpfs /tmp --network host safe-hash:latest --help
```

## Integration Examples

### CI/CD Pipeline

```yaml
# Example GitHub Actions step
- name: Verify Safe Transaction
  run: |
    podman build -t safe-hash:latest .
    podman run --rm --network host safe-hash:latest tx \
      --chain ethereum \
      --nonce ${{ env.NONCE }} \
      --safe-address ${{ env.SAFE_ADDRESS }} \
      --safe-version 1.4.1
```

### Automated Verification Script

```bash
#!/bin/bash
# verify-transaction.sh

# Build if image doesn't exist
if ! podman image exists localhost/safe-hash:latest; then
    ./scripts/podman-build.sh
fi

# Run verification
./scripts/safe-hash-rs tx \
  --chain "${CHAIN:-ethereum}" \
  --nonce "${NONCE}" \
  --safe-address "${SAFE_ADDRESS}" \
  --safe-version "${SAFE_VERSION:-1.4.1}"
```

This container setup provides a secure, isolated environment for running safe-hash while maintaining all the functionality of the native binary.