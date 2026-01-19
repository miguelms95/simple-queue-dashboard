current-dir := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))
SHELL = /bin/sh

help: ## show make targets
	@awk 'BEGIN {FS = ":.*?## "} /[a-zA-Z_-]+:.*?## / {sub("\\\\n",sprintf("\n%22c"," "), $$2);printf " \033[36m%-20s\033[0m  %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## build dist
	pnpm run build

run: ## run web
	pnpm run dev

tauri: ## run tauri desktop app
	pnpm run tauri dev

tauri-build:
	pnpm run tauri-build