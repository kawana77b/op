build:
	@goreleaser release --snapshot --rm-dist

test:
	go test -v ./...

.PHONY: test