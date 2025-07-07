SHELL:=/bin/sh

.PHONY: all

app_name="nflux"

help: ## this help
	@awk 'BEGIN {FS = ":.*?## ";  printf "Usage:\n  make \033[36m<target> \033[0m\n\nTargets:\n"} /^[a-zA-Z0-9_-]+:.*?## / {gsub("\\\\n",sprintf("\n%22c",""), $$2);printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

mtoc: ## Create table of contents with mtoc
	mtoc -e .target/ -e ./README.md

package: ## Package binary with zip
	zip -j nflux-$(PLATFORM)-$(TARGET).zip target/release/nflux

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

remote-sync: ## Sync this repository to remote machine using rsync.
	rsync -avzh --exclude='.git/' --exclude='target/' --exclude='.idea/' $(shell pwd)/ $(USER)@$(IP):/home/$(USER)/nflux

local-run: ## Run nflux locally
	cargo run --release --config 'target."cfg(all())".runner="sudo -E"' --

local-build: ## Build nflux locally
	cargo build --release --package nflux

local-install: local-build ## Install nflux binary locally
	sudo cp target/release/$(app_name) /usr/local/bin/$(app_name) ;
	sudo chmod +x /usr/local/bin/$(app_name)

nextest: ## Run nextest tests
	cargo nextest run --locked

test: ## Run tests
	cargo test --locked

check-unused-deps: ## Check for unused dependencies. $ cargo install cargo-machete
	cargo machete --with-metadata

generate-vmlinux: ## Generate vmlinux for kernel data structures
	aya-tool generate sock > nflux-ebpf/src/vmlinux.rs

setup-hooks:
	git config core.hooksPath .git-hooks

tag: ## Generate a new tag
	git-cliff -t $(version) -o CHANGELOG.md
	mtoc -f CHANGELOG.md
	git add CHANGELOG.md
	git commit -m "chore: update changelog for $(version)"
	git tag -a $(version) -m "$(message)"
