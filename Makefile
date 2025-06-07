CC := gcc
SRCDIR := src
BUILDDIR := build
SRCEXT := c
SOURCES := $(shell find $(SRCDIR) -type f -name *.c)
OBJECTS := $(patsubst $(SRCDIR)/%,$(BUILDDIR)/%,$(SOURCES:.c=.o))
HEADERS := $(SOURCES:.c=.h)
CFLAGS :=

default: $(OBJECTS)
	@echo "Linking jazzy.out"
	$(CC) $^ -o jazzy.out

	


# Custom recipe for main.c
$(BUILDDIR)/main.o: $(SRCDIR)/main.c
	@echo "Building $(shell basename $@)"
	@mkdir -p $(shell dirname $@)
	$(CC) $(CFLAGS) -c $< -o $@

$(BUILDDIR)/%.o: $(SRCDIR)/%.c $(SRCDIR)/%.h
	@echo "Building $(shell basename $@)"
	@mkdir -p $(shell dirname $@)
	$(CC) $(CFLAGS) -c $< -o $@
