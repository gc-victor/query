#!/bin/sh

ESBUILD=node_modules/.bin/esbuild

if [ "$PROD" = "true" ]; then
    sourcemap_value=""
else
    sourcemap_value="--sourcemap=inline"
fi

bundle_public() {
    ISLANDS_PUBLIC=$(find src/pages -not -path '*/admin/*' -name '*.island.*')

    $ESBUILD $ISLANDS_PUBLIC \
        --bundle \
        --target=es2020 \
        --format=esm \
        --jsx-factory=h \
        --jsx-fragment=Fragment \
        --minify=true \
        --legal-comments=none \
        --splitting \
        --entry-names=[dir]/[name] \
        --chunk-names=cache/[name]-[hash] \
        --public-path=/_/asset/dist/ \
        --outdir=dist \
        --log-level=error \
        --loader:.html=text \
        $sourcemap_value
}

bundle_public
