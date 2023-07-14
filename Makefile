.PHONY=test doc clean open-doc build-venv

test:
	cd cg_library && cargo test
	cd lab1 && cargo test
	cd lab2 && cargo test
	cd lab3 && cargo test

doc:
	cd cg_library && cargo doc
	cd lab1 && cargo doc
	cd lab2 && cargo doc
	cd lab3 && cargo doc

open-doc: | doc
	firefox cg_library/target/doc/cg_library/index.html

build-venv:
	cd python_venv && bash setup-venv.sh

clean:
	git clean -dxff
