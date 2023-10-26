PROJECT_DIR=$HOME/Git/rust/abitofhelp/helloworld-tonic-bsr
PROTO_DIR=$(PROJECT_DIR)/proto
VERSION=

.PHONY:buflist 1.bufclean 2.buflint 3.bufgen 4.bufbld cratelist 1.cratepub_dryrun 2.cratepub

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

cratelist:
	@cargo package --list

cratepub_dryrun:
	@cargo publish --dry-run

cratepub:
	@cargo publish

