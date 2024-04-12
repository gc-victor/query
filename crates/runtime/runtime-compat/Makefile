.PHONY: download

result:
	node scripts/process-test-results.mjs

download: # Before running this target, make sure the server is running
	mkdir -p download
	curl -o download/data.json 'http://localhost:3000/_/function/pages/data'

# catch anything and do nothing
%:
	@: