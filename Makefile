publish-dev:
	@goreleaser release --snapshot --clean

test:
	go test -v ./...

.PHONY: test