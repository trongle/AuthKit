#!/usr/bin/env bash

RUNNING_CONTAINER=$(docker ps --filter "name=redis" --format "{{.ID}}")
if [ -n "$RUNNING_CONTAINER" ]; then
	echo >&2 "Redis is already running."
	echo >&2 "To kill it, run: docker kill $RUNNING_CONTAINER"
	exit 1
fi

docker run -p "6379:6379" --name redis -d redis:latest

# purpose: wait for redis to be up and running.
>&2 

# Show success message.
echo "Redis is up and running on port 6379"
