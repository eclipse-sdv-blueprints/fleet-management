#!/bin/bash
deps_file=${DEPS_FILE:-"DEPS.txt"}
dash_jar=${DASH_JAR:-"/tmp/dash.jar"}
dash_summary=${DASH_SUMMARY:-"DASH_SUMMARY.txt"}
dash_url=${DASH_URL:-"https://repo.eclipse.org/service/rest/v1/search/assets/download?sort=version&repository=dash-maven2-releases&maven.groupId=org.eclipse.dash&maven.artifactId=org.eclipse.dash.licenses&maven.extension=jar"}
project=${PROJECT:-"automotive.uprotocol"}
token=$1

echo "creating 3rd party dependency list..."
cargo tree --manifest-path components/Cargo.toml -e no-build,no-dev --prefix none --no-dedupe --locked \
  | sed -n '2~1p' \
  | sort -u \
  | grep -v '^[[:space:]]*$' \
  | grep -v fms- \
  | grep -v influx-client \
  | grep -v up-transport-hono \
  | sed -E 's|([^ ]+) v([^ ]+).*|crate/cratesio/-/\1/\2|' \
  > "$deps_file"

if [[ ! -r "$dash_jar" ]]; then
  echo "Eclipse Dash JAR file [${dash_jar}] not found, downloading latest version from Eclipse repo..."
  wget_bin=$(which wget)
  if [[ -z "$wget_bin" ]]; then
    echo "wget command not available on path"
    exit 127
  else
    wget --quiet -O "$dash_jar" "$dash_url"
    echo "successfully downloaded Eclipse Dash JAR to ${dash_jar}"
  fi
fi

if ! jar tf "$dash_jar" >/dev/null 2>&1; then
  echo "Downloaded Eclipse Dash file [${dash_jar}] is not a valid JAR"
  exit 1
fi

args=(-jar "$dash_jar" -timeout 60 -batch 90 -summary "$dash_summary")
if [[ -n "$token" ]]; then
  args=("${args[@]}" -review -token "$token" -project "$project")
fi
args=("${args[@]}" "$deps_file")

echo "checking 3rd party licenses..."
java "${args[@]}"
