
ROOT=../..

CC_PROD=lib

SRCDIR=.

BINDIR=$(BINROOT)/libgnu

MODULE_NAME=gnu

define CC_DEFS +=
	HAVE_CONFIG_H
endef

define CC_C_FLAGS.warnings +=
	no-cast-qual
	no-conversion
	no-float-equal
	no-sign-compare
	no-undef
	no-unused-function
	no-unused-parameter
	no-float-conversion
	implicit-fallthrough
	no-pedantic
	no-sign-conversion
	no-type-limits
	no-unsuffixed-float-constants
endef

CC_INCLUDES += $(OS) . $(ROOT)/make/src/$(OS) $(ROOT)/make/src

define CC_SOURCES
	findprog-in.c
	concat-filename.c
endef

CLASSICO=/v/classico/classico
include $(CLASSICO)/mk/main
