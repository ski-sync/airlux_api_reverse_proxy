# install app from scratch for debian
!#/bin/bash

$version = "feature/11-implement-traefik-configuration-auto-update"

# install docker
if ! [ -x "$(command -v docker)" ]; then
  # install docker
  echo "docker is not installed, installing docker"
fi

# download docker-compose
curl -L "https://raw.githubusercontent.com/ski-sync/api_reverse_proxy/{{$version}}/docker-compose.yml" -o docker-compose.yml

# update image version
docker compose pull

# start the app
docker compose up -d
