ESBUILD=node_modules/.bin/esbuild

if [ "$PROD" = "true" ]; then
  sourcemap_value=""
else
  sourcemap_value="--sourcemap=inline"
fi

ISLANDS=$(find src/pages -name '*.island.*')

bundle() {
  $ESBUILD $ISLANDS \
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
    $sourcemap_value
}

bundle
