#!/bin/bash

sqlx migrate --source programs/server/src/data/migrations "$@"
