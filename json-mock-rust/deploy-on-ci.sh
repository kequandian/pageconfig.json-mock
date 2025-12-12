#!/bin/bash

# Deploy script to be executed on CI server n105
# This script builds and pushes the Docker image on the CI server

set -e

# Configuration
REGISTRY="registry1.cdnline.cn:5000"
IMAGE_NAME="pageconfig/json-mock"
TAG="rust"
FULL_IMAGE_NAME="${REGISTRY}/${IMAGE_NAME}:${TAG}"
PROJECT_DIR="/tmp/json-mock-rust"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Deploying json-mock-rust to CI server${NC}"
echo -e "${GREEN}Target image: ${FULL_IMAGE_NAME}${NC}"

# Check if we're on CI server
if [ ! -f /etc/ci-server ]; then
    echo -e "${YELLOW}Warning: This script should be executed on CI server n105${NC}"
fi

# Clean up previous build
echo -e "${YELLOW}Cleaning up previous build...${NC}"
rm -rf "${PROJECT_DIR}"

# Create project directory
mkdir -p "${PROJECT_DIR}"
cd "${PROJECT_DIR}"

echo -e "${YELLOW}Copying source files...${NC}"

# Note: You need to copy the source files to CI server first
# Use: scp -r json-mock-rust/ n105:/tmp/
echo -e "${RED}Please ensure source files are copied to ${PROJECT_DIR}${NC}"
echo -e "${YELLOW}Run this command from your local machine:${NC}"
echo -e "scp -r $(pwd)/../json-mock-rust n105:${PROJECT_DIR}"

# Wait for user to copy files
read -p "Press Enter after copying source files to CI server..."

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

# Cleanup build directory
echo -e "${YELLOW}Cleaning up build directory...${NC}"
cd /
rm -rf "${PROJECT_DIR}"

echo -e "${GREEN}Deployment completed successfully!${NC}"