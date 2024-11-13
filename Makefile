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

install-dpkg: ## Install dpkg package
	bash ./debian/build_deb.sh ;\
	sudo dpkg -i nflux.deb

paru-install: ## Install nflux with paru
	paru -U .
	#ls /usr/local/bin/nflux              # Check if the binary is installed
	#ls /etc/systemd/system/nflux.service  # Check if the service file is in place

paru-uninstall: ## Uninstall nflux with paru
	pacman -Rns nflux

run: ## Run nflux
	cargo xtask run -- -c nflux.toml

journal-logs: ## Show journal logs
	sudo journalctl -u nflux.service -f
