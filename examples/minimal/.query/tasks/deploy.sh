#!/bin/sh

export PROD=true

QUERY=node_modules/.bin/query

echo "Deploying to production..."
echo "$QUERY"

deploy_migration() {
    start=$(date +%s%N)
    $QUERY task migration -y
    end=$(date +%s%N)
    echo "Migration executed: $(((end - start) / 1000000))ms"
}

deploy_bundle() {
    start=$(date +%s%N)
    $QUERY task bundle
    end=$(date +%s%N)
    echo "Bundle created: $(((end - start) / 1000000))ms"
}

deploy_tailwind() {
    start=$(date +%s%N)
    $QUERY task tailwindcss
    end=$(date +%s%N)
    echo "Styles created: $(((end - start) / 1000000))ms"
}

deploy_asset() {
    start=$(date +%s%N)
    $QUERY asset dist
    $QUERY asset public
    end=$(date +%s%N)
    echo "Assets updated: $(((end - start) / 1000000))ms"
}

deploy_function() {
    start=$(date +%s%N)
    $QUERY function
    end=$(date +%s%N)
    echo "Functions updated: $(((end - start) / 1000000))ms"
}

deploy_migration &
deploy_tailwind &
deploy_bundle &
wait
deploy_asset
deploy_function
