
PHONY: build
build: download
	cargo build

NITRIC_VERSION := 1.0.0

PHONY: download
download:
	curl -L https://github.com/nitrictech/nitric/releases/download/v${NITRIC_VERSION}/proto.tgz -o nitric.tgz
	tar xvzf nitric.tgz
	rm nitric.tgz