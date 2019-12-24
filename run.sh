#!/usr/bin/env bash

set -e

DOCKERFILE_HASH=$(md5 -q <(echo $(md5 -q ./Dockerfile) $(md5 -q ./docker-compose.yml)))
APP_NAME=$(basename $(pwd))
IMAGE_NAME=${APP_NAME}:${DOCKERFILE_HASH}

if ! docker inspect ${IMAGE_NAME} > /dev/null; then
  echo "--- BUILDING image '${IMAGE_NAME}'---"
  APP_NAME=${APP_NAME} IMAGE_TAG="${DOCKERFILE_HASH}" docker-compose --verbose build --build-arg APP_NAME=${APP_NAME}
fi

APP_NAME=${APP_NAME} IMAGE_TAG="${DOCKERFILE_HASH}" docker-compose run --rm --service-ports app $@
