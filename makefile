add-arrays-module: make-rust-mod
	@echo -e "\npub mod $(mod);" >> src/arrays_and_hash/mod.rs

make-rust-mod:make-rust-test
	@touch src/arrays_and_hash/$(mod).rs;
	@echo -e 'pub fn $(mod)() ' >> src/arrays_and_hash/$(mod).rs;
	@echo  '{'>> src/arrays_and_hash/$(mod).rs
	@echo  ''>> src/arrays_and_hash/$(mod).rs
	@echo  '}' >> src/arrays_and_hash/$(mod).rs

make-rust-test:
	echo -e '\n#[allow(dead_code)]\nfn test_$(mod)() {\n    use arrays_and_hash::$(mod)::$(mod);\n}' >> src/main.rs
update-md:
	@echo -e '\n- $(mod)' >> readme.md