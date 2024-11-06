SHELL:=/bin/sh
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

compose-up: ## Run docker-compose up
	docker-compose -f docker/compose.yml up -d --force-recreate

compose-down: ## Run docker-compose down
	docker-compose -f docker/compose.yml down

remote-sync: ## Sync this repository to remote machine using rsync.
	rsync -avzh --exclude='.git/' --exclude='target/' --exclude='.idea/' $(shell pwd)/ $(USER)@$(IP):/tmp/nflux

install-systemd-service: ## Copy systemd service to /etc/systemd/system/
	sudo cp systemd/$(app_name).service /etc/systemd/system/$(app_name).service

install-binary: ## Install binary to /usr/local/bin/
	cargo xtask build --release ;\
	sudo cp target/release/$(app_name) /usr/local/bin/$(app_name)
