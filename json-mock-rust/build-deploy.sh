#!/bin/bash

# Build and deploy script for json-mock-rust
# Usage: ./build-deploy.sh [version]
# Example: ./build-deploy.sh v1.0.0

set -e

# Configuration
REGISTRY="${REGISTRY:-registry1.cdnline.cn:5000}"
IMAGE_NAME="pageconfig/json-mock"
TAG="${1:-rust}"
FULL_IMAGE_NAME="${REGISTRY}/${IMAGE_NAME}:${TAG}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Building Docker image: ${FULL_IMAGE_NAME}${NC}"

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
if ! command_exists docker; then
    echo -e "${RED}Error: Docker is not installed${NC}"
    exit 1
fi

# Build the Docker image
echo -e "${YELLOW}Building Docker image...${NC}"
docker build -t "${FULL_IMAGE_NAME}" .

# Check if build was successful
if [ $? -eq 0 ]; then
    echo -e "${GREEN}Successfully built ${FULL_IMAGE_NAME}${NC}"

    # Show image info
    echo -e "${YELLOW}Image details:${NC}"
    docker images "${FULL_IMAGE_NAME}"

    # Size check
    IMAGE_SIZE=$(docker images --format "table {{.Repository}}:{{.Tag}}\t{{.Size}}" "${FULL_IMAGE_NAME}" | grep "${TAG}" | awk '{print $2}')
    echo -e "${GREEN}Image size: ${IMAGE_SIZE}${NC}"

    # Tag for registry (if not already tagged)
    if [[ ! "${FULL_IMAGE_NAME}" =~ ${REGISTRY} ]]; then
        echo -e "${YELLOW}Tagging image for registry...${NC}"
        docker tag "${FULL_IMAGE_NAME}" "${REGISTRY}/${IMAGE_NAME}:${TAG}"
        FULL_IMAGE_NAME="${REGISTRY}/${IMAGE_NAME}:${TAG}"
    fi

    # Push to registry
    echo -e "${YELLOW}Pushing to registry ${REGISTRY}...${NC}"
    docker push "${FULL_IMAGE_NAME}"

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}Successfully pushed ${FULL_IMAGE_NAME} to registry${NC}"
    else
        echo -e "${RED}Failed to push ${FULL_IMAGE_NAME} to registry${NC}"
        exit 1
    fi

else
    echo -e "${RED}Failed to build ${FULL_IMAGE_NAME}${NC}"
    exit 1
fi

echo -e "${GREEN}Build and push completed successfully!${NC}"
echo -e "${YELLOW}To deploy to CI server n105, use:${NC}"
echo -e "ssh n105 'docker pull ${FULL_IMAGE_NAME}'"