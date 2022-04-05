#!/bin/bash
set -e
set -x

dropdb witter || true
dropdb witter-test || true

createdb witter || true
createdb witter-test || true

psql -d witter <setup/setup.sql
psql -d witter-test <setup/setup.sql
