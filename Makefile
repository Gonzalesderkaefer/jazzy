# C Compiler
CC := gcc

# Compiler Flags
CFLAGS :=

# Source directory
SRCDIR := src

# Builds go here
BUILDDIR := build

# Get all source files
SOURCES := $(shell find $(SRCDIR) -type f -name *.c)

# Derive Object files from source files
OBJECTS := $(patsubst $(SRCDIR)/%,$(BUILDDIR)/%,$(SOURCES:.c=.o)) \
		$(BUILDDIR)/utils/fileutils.o

# Derive Header files from source files
HEADERS := $(SOURCES:.c=.h)

# Subprojects
SUBPROCS := subprojects

default: $(OBJECTS)
	@echo "Linking jazzy.out"
	@$(CC) $^ -o jazzy.out


$(BUILDDIR)/utils/fileutils.o: $(SUBPROCS)/utils/fileutils/Makefile
	@mkdir -p $(shell dirname $@)
	@(cd $(shell dirname $<); make export)
	cp $(shell dirname $<)/target/fileutils.o $@
	cp $(shell dirname $<)/target/*.h $(SRCDIR)/utils/




# Custom recipe for main.c
$(BUILDDIR)/main.o: $(SRCDIR)/main.c $(SRCDIR)/config.h
	@echo "Building $(shell basename $@)"
	@mkdir -p $(shell dirname $@)
	$(CC) $(CFLAGS) -c $< -o $@


# Default target for source files
$(BUILDDIR)/%.o: $(SRCDIR)/%.c $(SRCDIR)/%.h
	@echo "Building $(shell basename $@)"
	@mkdir -p $(shell dirname $@)
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	@rm -rf $(BUILDDIR)/*
	@rm jazzy.out
