#!/bin/bash

# Build script for safe-hash using Podman
# This script builds the container image with proper tags and labels

set -e

# Configuration
IMAGE_NAME="safe-hash"
IMAGE_TAG="latest"
REGISTRY_NAME="localhost"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Building safe-hash container image with Podman...${NC}"

# Get version from Cargo.toml if available
VERSION=$(grep -E '^version = "' crates/safe-hash/Cargo.toml | sed 's/version = "\(.*\)"/\1/' | head -1)
if [ -n "$VERSION" ]; then
    echo -e "${GREEN}Detected version: $VERSION${NC}"
    VERSION_TAG="$VERSION"
else
    echo -e "${YELLOW}Could not detect version, using 'latest'${NC}"
    VERSION_TAG="latest"
fi

# Build the image
echo -e "${BLUE}Building container image...${NC}"
podman build \
    --tag "${REGISTRY_NAME}/${IMAGE_NAME}:${IMAGE_TAG}" \
    --tag "${REGISTRY_NAME}/${IMAGE_NAME}:${VERSION_TAG}" \
    --label "org.opencontainers.image.title=safe-hash" \
    --label "org.opencontainers.image.description=Verify Safe Wallet Transactions and Messages" \
    --label "org.opencontainers.image.version=${VERSION:-unknown}" \
    --label "org.opencontainers.image.source=https://github.com/Cyfrin/safe-hash-rs" \
    --label "org.opencontainers.image.vendor=Cyfrin" \
    --build-arg BUILDKIT_INLINE_CACHE=1 \
    .

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Build completed successfully!${NC}"
    echo -e "${GREEN}Image tags:${NC}"
    echo -e "  - ${REGISTRY_NAME}/${IMAGE_NAME}:${IMAGE_TAG}"
    echo -e "  - ${REGISTRY_NAME}/${IMAGE_NAME}:${VERSION_TAG}"
    
    # Show image details
    echo -e "\n${BLUE}Image details:${NC}"
    podman images "${REGISTRY_NAME}/${IMAGE_NAME}"
    
    echo -e "\n${GREEN}To run the container, use:${NC}"
    echo -e "  podman run --rm ${REGISTRY_NAME}/${IMAGE_NAME}:${IMAGE_TAG} --help"
    echo -e "\n${GREEN}Or use the safe-hash-rs script:${NC}"
    echo -e "  ./scripts/safe-hash-rs --help"
else
    echo -e "${RED}❌ Build failed!${NC}"
    exit 1
fi