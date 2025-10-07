#!/bin/bash

# Run script for safe-hash using Podman
# This script runs the container with proper volume mounts and security settings

set -e

# Configuration
IMAGE_NAME="safe-hash"
IMAGE_TAG="latest"
REGISTRY_NAME="localhost"
CONTAINER_NAME="safe-hash-$(date +%s)"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
INTERACTIVE=false
VOLUME_MOUNT=""
WORKING_DIR="/app"

# Function to show usage
show_usage() {
    echo "Usage: $0 [OPTIONS] [SAFE_HASH_ARGS...]"
    echo ""
    echo "Options:"
    echo "  -i, --interactive    Run in interactive mode with shell access"
    echo "  -v, --volume DIR     Mount a host directory to /app/input in the container"
    echo "  -h, --help          Show this help message"
    echo ""
    echo "Examples:"
    echo "  # Show safe-hash help"
    echo "  $0 --help"
    echo ""
    echo "  # Verify a transaction"
    echo "  $0 tx --chain ethereum --nonce 63 --safe-address 0x1c694Fc3006D81ff4a56F97E1b99529066a23725 --safe-version 1.4.1"
    echo ""
    echo "  # Mount a directory and use a file from it"
    echo "  $0 -v ./test msg --chain sepolia --safe-address 0x657ff0D4eC65D82b2bC1247b0a558bcd2f80A0f1 --input-file /app/input/test_message.txt --safe-version 1.4.1"
    echo ""
    echo "  # Run in interactive mode"
    echo "  $0 -i"
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -i|--interactive)
            INTERACTIVE=true
            shift
            ;;
        -v|--volume)
            if [ -z "$2" ]; then
                echo -e "${RED}Error: --volume requires a directory argument${NC}" >&2
                exit 1
            fi
            VOLUME_MOUNT="$2"
            shift 2
            ;;
        -h|--help)
            show_usage
            exit 0
            ;;
        *)
            # All remaining arguments are passed to safe-hash
            break
            ;;
    esac
done

# Check if image exists
if ! podman image exists "${REGISTRY_NAME}/${IMAGE_NAME}:${IMAGE_TAG}"; then
    echo -e "${YELLOW}Image ${REGISTRY_NAME}/${IMAGE_NAME}:${IMAGE_TAG} not found.${NC}"
    echo -e "${BLUE}Building the image first...${NC}"
    
    # Check if build script exists
    if [ -f "./scripts/podman-build.sh" ]; then
        ./scripts/podman-build.sh
    else
        echo -e "${RED}Build script not found. Please run the build script first.${NC}"
        exit 1
    fi
fi

# Prepare podman run command
PODMAN_ARGS=(
    "run"
    "--rm"
    "--name" "${CONTAINER_NAME}"
    "--security-opt" "label=disable"
    "--cap-drop" "ALL"
    "--read-only"
    "--tmpfs" "/tmp"
    "--network" "host"
)

# Add volume mount if specified
if [ -n "$VOLUME_MOUNT" ]; then
    # Convert to absolute path
    VOLUME_MOUNT=$(realpath "$VOLUME_MOUNT")
    
    if [ ! -d "$VOLUME_MOUNT" ]; then
        echo -e "${RED}Error: Directory $VOLUME_MOUNT does not exist${NC}" >&2
        exit 1
    fi
    
    echo -e "${GREEN}Mounting $VOLUME_MOUNT to /app/input${NC}"
    PODMAN_ARGS+=("--volume" "${VOLUME_MOUNT}:/app/input:ro,Z")
fi

# Add image
PODMAN_ARGS+=("${REGISTRY_NAME}/${IMAGE_NAME}:${IMAGE_TAG}")

# Run in interactive mode or with arguments
if [ "$INTERACTIVE" = true ]; then
    echo -e "${BLUE}Starting interactive container...${NC}"
    PODMAN_ARGS[2]="--rm"  # Replace --rm with -it
    podman run -it "${PODMAN_ARGS[@]:3}" /bin/bash
else
    if [ $# -eq 0 ]; then
        # No arguments provided, show help
        PODMAN_ARGS+=("--help")
    else
        # Pass all remaining arguments to safe-hash
        PODMAN_ARGS+=("$@")
    fi
    
    echo -e "${BLUE}Running safe-hash in container...${NC}"
    podman "${PODMAN_ARGS[@]}"
fi