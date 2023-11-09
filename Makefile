.Phony: run
.Phony: build

run:
	@./command.sh $(filter-out $@,$(MAKECMDGOALS))
build: 
	cargo build --release --workspace
%:
	@:
