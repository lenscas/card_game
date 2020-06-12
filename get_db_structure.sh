#!/bin/sh

export $(egrep -v '^#' .env | xargs)
pg_dump ${DATABASE_URL} --schema-only > database/schema.sql