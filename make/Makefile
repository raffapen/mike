
ROOT=..

CC_PROD=lib

SRCDIR=src

BINDIR=$(BINROOT)/make

define CC_FLAGS
	-fdiagnostics-color=always 
endef

define CC_DEFS
	LIBMAKE
	HAVE_CONFIG_H
	LOCALEDIR=\"/usr/local/share/locale\"
	LIBDIR=\"/usr/local/lib\"
endef

define CC_C_FLAGS.warnings
	all
	extra
	error
	write-strings
	shadow
	declaration-after-statement
	bad-function-cast
	format-security
	type-limits
	unused-but-set-parameter
	logical-op
	pointer-arith
	ignored-qualifiers
	format-signedness
	duplicated-cond
endef

CC_INCLUDES=src/$(OS) src lib

define CC_SOURCES
	ar.c
	arscan.c
	commands.c
	default.c
	dir.c
	expand.c
	file.c
	function.c
	getopt.c
	getopt1.c
	guile.c
	hash.c
	implicit.c
	job.c
	load.c
	loadapi.c
	main.c
	misc.c
	output.c
	read.c
	remake.c
	rule.c
	shuffle.c
	signame.c
	strcache.c
	variable.c
	version.c
	vpath.c
	posixos.c
	remote-stub.c
endef

LD_FLAGS=-Wl,--export-dynamic

LIBGNU=$(BINROOT)/libgnu/libgnu.a

LD_LIBS=$(LIBGNU)

CLASSICO=/v/classico/classico
include $(CLASSICO)/mk/main

$(LIBGNU):
	@$(MAKE) -C lib

clean-all: clean
	$(MAKE) -C lib clean
