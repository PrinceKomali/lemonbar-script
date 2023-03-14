DC := /usr/bin/gdc
PULSEFLAGS := $(shell pkg-config --libs --cflags libpulse)

CFLAGS := -Wall
BUILD := build
TARGET := target/debug
SRCDIR := src
O := $(BUILD)/o

TARGET_NAME := out

SRC := $(wildcard $(SRCDIR)/*.c) 
SRC := $(SRC)_$(wildcard $(SRCDIR)/*.d)
SRC := $(notdir $(SRC))
OBJS := $(patsubst %, $(O)/%.o, $(SRC))
RLIB := $(O)/lib.rs.a

define bold
	@printf "\x1b[1m"
endef
define print
	@echo -e "\x1b[33m$(1)\x1b[34m -> \x1b[32m$(2)\x1b[0m"
endef

all: dircheck $(BUILD)/$(TARGET_NAME) 

$(BUILD)/$(TARGET_NAME): $(OBJS) $(RLIB)
	$(call bold)
	$(call print,$^,$@)
	@$(DC) -o $@ $^ $(PULSEFLAGS)

$(RLIB): $(wildcard $(SRCDIR)/*.rs)
	$(call print,"src/",$@)
	@cargo build
	@cp target/debug/*.a $(O)/lib.rs.a

$(O)/%.o: $(SRCDIR)/%
	$(call print,$^,$@)
	@$(DC) $(CFLAGS) -I$(SRCDIR) -c -o $@ $^  


dircheck:
	@mkdir -p $(BUILD)
	@mkdir -p $(O)
	@mkdir -p target
	@mkdir -p target/debug

clean: 
	@find $(BUILD) -type f -delete
	@rm -rf $(TARGET)
run: all
	$(BUILD)/$(TARGET_NAME)
.PHONY: clean run 