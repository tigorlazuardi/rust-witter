#!/bin/bash
set -e
set -x

dropdb -h localhost -U tigor witter || true
dropdb -h localhost -U tigor witter-test || true

createdb -h localhost -U tigor witter || true
createdb -h localhost -U tigor witter-test || true

psql -h localhost -U tigor -d witter <setup/setup.sql
psql -h localhost -U tigor -d witter-test <setup/setup.sql
