.PHONY: all
all: run-go

.PHONY: run-go
run-go:
	go run .

.PHONY: dev-web
dev-web:
	cd ./web && pnpm dev

.PHONY: build-and-run
build-and-run:
	cd ./web && pnpm build
	go run .
