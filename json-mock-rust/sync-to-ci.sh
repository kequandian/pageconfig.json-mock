#!/bin/bash

# Sync source files to CI server using rsync and optionally build & push
#
# Usage: 
#   ./sync-to-ci.sh [ci-server] [dest-path] [--deploy] [--registry=URL]
#
# Arguments:
#   ci-server     : CI server hostname (default: n105)
#   dest-path     : Destination path on CI server (default: /tmp/json-mock-rust)
#   --deploy      : Build and push Docker image after sync
#   --registry=URL: Docker registry URL (default: registry1.cdnline.cn:5000)
#   --image=NAME  : Docker image name (default: pageconfig/json-mock)
#   --tag=TAG     : Docker image tag (default: rust)
#
# Examples:
#   ./sync-to-ci.sh                                    # Only sync files
#   ./sync-to-ci.sh n105 /tmp/rust-app                 # Sync to custom path
#   ./sync-to-ci.sh n105 /tmp/json-mock-rust --deploy # Full deployment
#   ./sync-to-ci.sh n105 /tmp/app --deploy --registry=docker.io --image=myapp/rust --tag=v1.0

set -e

# Default configuration
DEFAULT_REGISTRY="registry1.cdnline.cn:5000"
DEFAULT_IMAGE_NAME="pageconfig/json-mock"
DEFAULT_TAG="rust"

# Parse positional arguments
CI_SERVER="${1:-n105}"
DEST_DIR="${2:-/tmp/json-mock-rust}"
SOURCE_DIR="$(cd "$(dirname "$0")" && pwd)"

# Initialize with defaults
REGISTRY="${DEFAULT_REGISTRY}"
IMAGE_NAME="${DEFAULT_IMAGE_NAME}"
TAG="${DEFAULT_TAG}"
SHOULD_DEPLOY=false

# Parse all arguments for flags
for arg in "$@"; do
    case $arg in
        --deploy|--build)
            SHOULD_DEPLOY=true
            ;;
        --registry=*)
            REGISTRY="${arg#*=}"
            ;;
        --image=*)
            IMAGE_NAME="${arg#*=}"
            ;;
        --tag=*)
            TAG="${arg#*=}"
            ;;
    esac
done

FULL_IMAGE_NAME="${REGISTRY}/${IMAGE_NAME}:${TAG}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${BLUE}════════════════════════════════════════${NC}"
if [ "$SHOULD_DEPLOY" == "true" ]; then
    echo -e "${GREEN}  One-Click Deploy to CI Server${NC}"
else
    echo -e "${GREEN}  Syncing to CI Server: ${CI_SERVER}${NC}"
fi
echo -e "${BLUE}════════════════════════════════════════${NC}"
echo -e "${YELLOW}Source:      ${SOURCE_DIR}${NC}"
echo -e "${YELLOW}Destination: ${CI_SERVER}:${DEST_DIR}${NC}"
if [ "$SHOULD_DEPLOY" == "true" ]; then
    echo -e "${CYAN}Registry:    ${REGISTRY}${NC}"
    echo -e "${CYAN}Image:       ${IMAGE_NAME}:${TAG}${NC}"
    echo -e "${CYAN}Full Name:   ${FULL_IMAGE_NAME}${NC}"
fi
echo -e ""

# Check if rsync is available
if ! command -v rsync &> /dev/null; then
    echo -e "${RED}Error: rsync is not installed${NC}"
    echo -e "${YELLOW}Install it with:${NC}"
    echo -e "  macOS:   brew install rsync"
    echo -e "  Ubuntu:  apt-get install rsync"
    exit 1
fi

# Create destination directory on CI server
echo -e "${YELLOW}Creating destination directory on CI server...${NC}"
ssh "${CI_SERVER}" "mkdir -p ${DEST_DIR}"

# Sync files with rsync
echo -e "${YELLOW}Syncing files...${NC}"
rsync -avz --progress --delete \
    --exclude='target/' \
    --exclude='.git/' \
    --exclude='*.md' \
    --exclude='.dockerignore' \
    --exclude='Cargo.lock' \
    --exclude='.vscode/' \
    --exclude='.idea/' \
    --exclude='*.swp' \
    --exclude='*.swo' \
    --exclude='*~' \
    "${SOURCE_DIR}/" "${CI_SERVER}:${DEST_DIR}/"

if [ $? -ne 0 ]; then
    echo -e ""
    echo -e "${RED}✗ Failed to sync files${NC}"
    exit 1
fi

echo -e ""
echo -e "${GREEN}✓ Files synced successfully!${NC}"
echo -e ""

# If --deploy flag is set, build and push Docker image
if [ "$SHOULD_DEPLOY" == "true" ]; then
    echo -e "${YELLOW}Building and pushing Docker image on ${CI_SERVER}...${NC}"
    echo -e ""
    
    ssh "${CI_SERVER}" "cd ${DEST_DIR} && export DOCKER_BUILDKIT=1 && bash -c '
set -e

# Disable Docker registry mirrors, use docker.io directly
unset DOCKER_REGISTRY_MIRROR
export BUILDKIT_HOST=

REGISTRY=\"${REGISTRY}\"
IMAGE_NAME=\"${IMAGE_NAME}\"
TAG=\"${TAG}\"
FULL_IMAGE_NAME=\"\${REGISTRY}/\${IMAGE_NAME}:\${TAG}\"

echo \"Building Docker image with BuildKit (using docker.io)...\"
docker build -t \"\${FULL_IMAGE_NAME}\" .

if [ \$? -eq 0 ]; then
    echo \"\"
    echo \"✓ Build successful\"
    echo \"\"
    echo \"Image details:\"
    docker images \"\${FULL_IMAGE_NAME}\"
    echo \"\"
    
    echo \"Pushing to registry...\"
    docker push \"\${FULL_IMAGE_NAME}\"
    
    if [ \$? -eq 0 ]; then
        echo \"\"
        echo \"✓ Push successful\"
    else
        echo \"\"
        echo \"✗ Failed to push image\"
        exit 1
    fi
else
    echo \"\"
    echo \"✗ Build failed\"
    exit 1
fi
'"

    if [ $? -eq 0 ]; then
        echo -e ""
        echo -e "${GREEN}═══════════════════════════════════════════${NC}"
        echo -e "${GREEN}  ✓ Deployment completed successfully!${NC}"
        echo -e "${GREEN}═══════════════════════════════════════════${NC}"
        echo -e ""
        echo -e "${YELLOW}Image pushed to:${NC} ${BLUE}${FULL_IMAGE_NAME}${NC}"
    else
        echo -e ""
        echo -e "${RED}✗ Deployment failed${NC}"
        exit 1
    fi
else
    # Show next steps
    echo -e "${YELLOW}To build and push Docker image, add --deploy flag:${NC}"
    echo -e "${BLUE}./sync-to-ci.sh ${CI_SERVER} ${DEST_DIR} --deploy${NC}"
    echo -e ""
    echo -e "${YELLOW}Custom registry example:${NC}"
    echo -e "${BLUE}./sync-to-ci.sh ${CI_SERVER} ${DEST_DIR} --deploy --registry=docker.io --image=myapp/rust --tag=v1.0${NC}"
fi
