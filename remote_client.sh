#!/bin/sh

REMOTE="http://localhost:3030"

command="\"$1\""
shift

args=""
first="$1"
shift
if [ -n "$first" ]; then
	args="\"$first\""
	while true; do
		a="$1"

		if [ -z $a ]; then
			break
		fi
		shift
		
		args="$args,\"$a\""
	done
fi

args="[$args]"
payload="{\"name\":$command, \"args\":$args}"

curl -H "Content-Type: application/json" -d "$payload" "$REMOTE"
echo
