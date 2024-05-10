create-package: 
	mkdir src/$(package)
	echo -e "\npub mod $(package);" >> src/lib.rs
	echo -e "\nmod $(package); " >> src/main.rs
	
add-module: add-to-main 

	@echo -e '#[allow(dead_code)]\npub fn $(mod)() ' >> src/$(package)/$(mod).rs
	@echo  '{'>> src/$(package)/$(mod).rs
	@echo  ''>> src/$(package)/$(mod).rs
	@echo  '}' >> src/$(package)/$(mod).rs
	@echo -e "\npub mod $(mod);" >> src/$(package)/mod.rs
	echo -e "use stack::valid_parentheses::valid_parentheses"

add-to-main: make-rust-test
	echo -e "use $(package)::$(mod)::$(mod)" >> src/main.rs
make-rust-test: 
	@echo -e '\n\n#[test]\nfn test_$(mod)(){\n\tuse $(package)::$(mod)::$(mod);\n\t\n} ' >> tests/test_$(package).rs

update-md:
	@echo -e '\n- $(mod)' >> readme.md

add-test:
	echo -e "use leetcode::$(package);" >> tests/test_$(package).rs
	
run:
	clear && timeout ${timeout} cargo run