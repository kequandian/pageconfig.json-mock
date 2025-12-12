#!/bin/bash

# One-click deployment wrapper for sync-to-ci.sh
# This script provides a simple interface with pre-configured defaults
#
# Usage:
#   ./deploy-on-ci.sh              # Deploy with all defaults
#   ./deploy-on-ci.sh --sync-only  # Only sync files, don't build/push

set -e

# ============================================
# Configuration - Modify these as needed
# ============================================
CI_SERVER="n105"
DEST_PATH="/tmp/json-mock-rust"
REGISTRY="registry1.cdnline.cn:5000"
IMAGE_NAME="pageconfig/json-mock"
TAG="rust"

# ============================================
# Script Logic
# ============================================

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
SYNC_SCRIPT="${SCRIPT_DIR}/sync-to-ci.sh"

# Check if sync-to-ci.sh exists
if [ ! -f "${SYNC_SCRIPT}" ]; then
    echo "Error: sync-to-ci.sh not found in ${SCRIPT_DIR}"
    exit 1
fi

# Make sure sync-to-ci.sh is executable
chmod +x "${SYNC_SCRIPT}"

# Parse arguments
SYNC_ONLY=false
for arg in "$@"; do
    case $arg in
        --sync-only)
            SYNC_ONLY=true
            ;;
        --help|-h)
            echo "One-click deployment wrapper"
            echo ""
            echo "Usage:"
            echo "  ./deploy-on-ci.sh              # Deploy with all defaults"
            echo "  ./deploy-on-ci.sh --sync-only  # Only sync files"
            echo ""
            echo "Current configuration:"
            echo "  CI Server: ${CI_SERVER}"
            echo "  Dest Path: ${DEST_PATH}"
            echo "  Registry:  ${REGISTRY}"
            echo "  Image:     ${IMAGE_NAME}:${TAG}"
            echo ""
            echo "Edit this script to change default values."
            exit 0
            ;;
    esac
done

# Build command
if [ "$SYNC_ONLY" == "true" ]; then
    # Sync only
    exec "${SYNC_SCRIPT}" \
        "${CI_SERVER}" \
        "${DEST_PATH}"
else
    # Full deployment
    exec "${SYNC_SCRIPT}" \
        "${CI_SERVER}" \
        "${DEST_PATH}" \
        --deploy \
        --registry="${REGISTRY}" \
        --image="${IMAGE_NAME}" \
        --tag="${TAG}"
fi
