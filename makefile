add-arrays-module: make-pub-rust
	@echo -e "\npub mod $(mod);" >> src/arrays_and_hash/mod.rs

make-pub-rust:
	@touch src/arrays_and_hash/$(mod).rs;
	@echo -e 'pub fn $(mod)() ' >> src/arrays_and_hash/$(mod).rs;
	@echo  '{'>> src/arrays_and_hash/$(mod).rs
	@echo  ''>> src/arrays_and_hash/$(mod).rs
	@echo  '}' >> src/arrays_and_hash/$(mod).rs

update-md:
	@echo -e '\n- $(mod)' >> readme.md