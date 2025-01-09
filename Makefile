
MK.rust=1

MK_CUSTOM_CLEAN=1

CLASSICO=/v/classico/classico
include $(CLASSICO)/mk/main

custom-clean:
	$(SHOW)cargo clean
	$(SHOW)$(MAKE) -C make clean
