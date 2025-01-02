.PHONY: download

result:
	node scripts/process-test-results.mjs

download: # Before running this target, make sure the server is running
	mkdir -p download
	curl -o download/data.json 'http://localhost:3000/_/function/pages/data'

vendor:
	rm -rf src/pages/vendor || true && \
	git clone --depth 1 --sparse --filter=blob:none https://github.com/unjs/runtime-compat.git temp-clone && \
	(cd temp-clone && git sparse-checkout set vendor) && \
	cp -r temp-clone/vendor src/pages/ && \
	rm -rf temp-clone

# catch anything and do nothing
%:
	@: