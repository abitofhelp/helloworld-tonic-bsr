PROJECT_DIR=$HOME/Git/rust/abitofhelp/helloworld-tonic-bsr
PROTO_DIR=$(PROJECT_DIR)/proto

.PHONY:buflist bufclean buflint bufgen bufbld

buflist:
	@buf ls-files

1.bufclean:
	@rm -rf "$(PROJECT_DIR)/src/gen"

2.buflint:
	@buf lint

3.bufgen:
	@buf generate -v --include-imports

4.bufbld:
	@buf build

