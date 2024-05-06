#!/bin/bash

version="feature/11-implement-traefik-configuration-auto-update"

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
  echo "Docker is not installed, installing Docker..."
  apt-get update
  apt-get install -y apt-transport-https ca-certificates curl software-properties-common
  curl -fsSL https://download.docker.com/linux/ubuntu/gpg | apt-key add -
  add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable"
  apt-get update
  apt-get install -y docker-ce
fi

# Check if Docker Compose is installed
if ! command -v docker compose &> /dev/null; then
  echo "Docker Compose is not installed, installing Docker Compose..."
  curl -L "https://github.com/docker/compose/releases/download/v2.4.1/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
  chmod +x /usr/local/bin/docker-compose
fi

# Download docker-compose.yml
curl -L "https://raw.githubusercontent.com/ski-sync/api_reverse_proxy/$version/docker-compose.yml" -o docker-compose.yml

# Update image version
docker compose pull

# Start the app
docker compose up -d
