#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: `psql` is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: `sqlx` command is not available. `sqlx-cli` needs to be installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install --version=0.6.1 sqlx-cli --features postgres"
  echo >&2 "to install it."
  exit 1
fi

DB_IMAGE="postgres:14"

# Check if a custom user has been set, otherwise default to 'postgres'
DB_USER="${POSTGRES_USER:=fs_rs_rwa}"
# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=fs_rs_rwa}"
# Check if a custom password has been set, otherwise default to 'fs_rs_rwa'
DB_NAME="${POSTGRES_DB:=fs_rs_rwa}"
# Check if a custom port has been set, otherwise default to '5432'
DB_PORT="${POSTGRES_PORT:=5441}"
# Check if a custom host has been set, otherwise default to 'localhost'
DB_HOST="${POSTGRES_HOST:=localhost}"

# Allow to skip Docker if a dockerized Postgres database is already running
if [[ -z "${SKIP_DOCKER}" ]]
then
  # if a postgres container is running, print instructions to kill it and exit
  RUNNING_POSTGRES_CONTAINER=$(docker ps --filter 'name=fs_rs_rwa' --format '{{.ID}}')
  if [[ -n $RUNNING_POSTGRES_CONTAINER ]]; then
    echo >&2 "there is a postgres container already running, kill it with"
    echo >&2 "    docker kill ${RUNNING_POSTGRES_CONTAINER}"
    exit 1
  fi
  # Launch postgres using Docker
  docker run \
      -e POSTGRES_USER=${DB_USER} \
      -e POSTGRES_PASSWORD=${DB_PASSWORD} \
      -e POSTGRES_DB=${DB_NAME} \
      -p "${DB_PORT}":5432 \
      -d \
      --name "fs_rs_rwa_$(date '+%Y%m%d_%H%M%S')" \
      ${DB_IMAGE} -N 1000
      # ^ Increased maximum number of connections for testing purposes
fi

# Keep polling Postgres for its readiness.
until PGPASSWORD="${DB_PASSWORD}" psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

echo "Postgres is up and running on port ${DB_PORT}."

echo "Running the database migrations now ..."

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create

sqlx migrate run
echo "Completed the database migrations."
