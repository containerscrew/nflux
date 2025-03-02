SHELL:=/bin/bash
.PHONY: all

app_name="nflux"

help: ## this help
	@awk 'BEGIN {FS = ":.*?## ";  printf "Usage:\n  make \033[36m<target> \033[0m\n\nTargets:\n"} /^[a-zA-Z0-9_-]+:.*?## / {gsub("\\\\n",sprintf("\n%22c",""), $$2);printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

mtoc: ## Create table of contents with mtoc
	mtoc

pre-commit-install: ## Install pre-commit
	pre-commit install

pre-commit-uninstall: ## Uninstall pre-commit
	pre-commit uninstall

run-pre-commit: ## Run pre-commit locally
	pre-commit run -a

generate-changelog: ## Generate changelog
	git cliff -o CHANGELOG.md

init-gitmoji: ## Init gitmoji (sudo npm i -g gitmoji-cli)
	gitmoji --init

DOCKER_COMPOSE := $(shell which docker-compose 2>/dev/null || echo podman-compose)

compose-build: ## Run docker-compose build
	$(DOCKER_COMPOSE) -f compose.yml build

compose-up: ## Run docker-compose up
	$(DOCKER_COMPOSE) -f compose.yml up -d --force-recreate

compose-up-build: ## Run docker-compose and build
	$(DOCKER_COMPOSE) -f compose.yml up --build -d

compose-down: ## Run docker-compose down
	$(DOCKER_COMPOSE) -f compose.yml down

remote-sync: ## Sync this repository to remote machine using rsync.
	rsync -avzh --exclude='.git/' --exclude='target/' --exclude='.idea/' $(shell pwd)/ $(USER)@$(IP):/home/$(USER)/nflux

local-run: ## Run nflux locally
	cargo run --release --config 'target."cfg(all())".runner="sudo -E"'

local-build: ## Build nflux locally
	cargo build --release

local-install: local-build ## Install nflux binary locally
	sudo cp target/release/$(app_name) /usr/local/bin/$(app_name) ;
	sudo chmod +x /usr/local/bin/$(app_name)

test: ## Run tests
	cargo test --locked
