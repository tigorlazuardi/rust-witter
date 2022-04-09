#!/bin/bash
set -e
set -x

dropdb -h localhost -U tigor witter || true

createdb -h localhost -U tigor witter || true

psql -h localhost -U tigor -d witter <setup/setup.sql
