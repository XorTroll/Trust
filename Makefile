
.SUFFIXES:

ifeq ($(strip $(DEVKITPRO)),)
$(error "Please set DEVKITPRO in your environment. export DEVKITPRO=<path to>/devkitpro")
endif

TOPDIR ?= $(CURDIR)
ROOTDIR ?= $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

include $(DEVKITPRO)/libnx/switch_rules

NAME := Trust

# --------------------------------------------------------------------------------------

BUILD := target
OUTPUT := $(CURDIR)/target/aarch64-none-elf/debug/$(NAME)
TARGET := $(NAME)

ifneq ($(BUILD),$(notdir $(CURDIR)))

ifeq ($(strip $(ICON)),)
	icons := $(wildcard *.jpg)
	ifneq (,$(findstring $(TARGET).jpg,$(icons)))
		export APP_ICON := $(TOPDIR)/$(TARGET).jpg
	else
		ifneq (,$(findstring icon.jpg,$(icons)))
			export APP_ICON := $(TOPDIR)/icon.jpg
		endif
	endif
else
	export APP_ICON := $(TOPDIR)/$(ICON)
endif

ifeq ($(strip $(NO_ICON)),)
	export NROFLAGS += --icon=$(APP_ICON)
endif

ifeq ($(strip $(NO_NACP)),)
	export NROFLAGS += --nacp=$(OUTPUT).nacp
endif

ifneq ($(APP_TITLEID),)
	export NACPFLAGS += --titleid=$(APP_TITLEID)
endif

ifneq ($(ROMFS),)
	export NROFLAGS += --romfsdir=$(CURDIR)/$(ROMFS)
endif

.PHONY: $(BUILD) clean all

#---------------------------------------------------------------------------------
all: $(BUILD)

$(BUILD):
	@[ -d $@ ] || mkdir -p $@
	@$(MAKE) --no-print-directory -C $(BUILD) -f $(CURDIR)/Makefile

#---------------------------------------------------------------------------------
clean:
	@echo clean ...
	@rm -fr $(BUILD) $(TOPDIR)/Cargo.lock $(OUTPUT).pfs0 $(OUTPUT).nso $(OUTPUT).nro $(OUTPUT).nacp $(OUTPUT).elf


#---------------------------------------------------------------------------------
else

TOPDIR := $(CURDIR)/..
OUTPUT := $(TOPDIR)/target/aarch64-none-elf/debug/$(NAME)

.PHONY: all

#---------------------------------------------------------------------------------
# main targets
#---------------------------------------------------------------------------------

all	:	xargo $(OUTPUT).nso $(OUTPUT).nro

xargo:
	@echo compiling $(NAME).elf
	@env CARGO_INCREMENTAL=0 RUST_TARGET_PATH=$(TOPDIR) xargo build --target aarch64-none-elf -p $(NAME)

$(OUTPUT).pfs0	:	$(OUTPUT).nso

$(OUTPUT).nso	:	$(OUTPUT).elf

ifeq ($(strip $(NO_NACP)),)
$(OUTPUT).nro	:	$(OUTPUT).elf $(OUTPUT).nacp
else
$(OUTPUT).nro	:	$(OUTPUT).elf
endif

endif