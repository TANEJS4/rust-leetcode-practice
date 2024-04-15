create-package: 
	mkdir src/$(package)
	touch src/$(package)/mod.rs
	echo -e "\npub mod $(package);" >> src/lib.rs
	echo -e "\nmod $(package); " >> src/main.rs
	
add-module: 
	@echo -e 'pub fn $(mod)() ' >> src/$(package)/$(mod).rs
	@echo  '{'>> src/$(package)/$(mod).rs
	@echo  ''>> src/$(package)/$(mod).rs
	@echo  '}' >> src/$(package)/$(mod).rs

	@echo -e "\npub mod $(mod);" >> src/$(package)/mod.rs
	
make-rust-test:
	echo -e '\n#[allow(dead_code)]\nfn test_$(mod)() {\n    use arrays_and_hash::$(mod)::$(mod);\n}' >> src/main.rs

update-md:
	@echo -e '\n- $(mod)' >> readme.md

add-test:
	echo -e "use leetcode::$(package);" >> tests/test_$(package).rs
	