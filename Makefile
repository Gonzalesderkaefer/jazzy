# Compiler
CC := gcc

# C-Compiler flags
CFLAGS := -Wall -Werror

# Source directory
SRCDIR := src

# This is for static libraries and other binaries
BUILDDIR := build

# Here are the used header files
HEADDIR := src/headers

# This is where the header file and compiled libraries will land
TARGETDIR := target

# This is where executables will land
RUNDIR := run

# Test directory
TESTDIR := test

# Other libraries we intend to use in our code
LIBRARIES := lib

# These are all directories
DIRS := $(SRCDIR) $(BUILDDIR) $(HEADDIR) $(TARGETDIR) $(RUNDIR) $(TESTDIR) $(LIBRARIES)

# Source files
SRCFILES := $(wildcard src/*.c)

# Addional Object files
AOBJFILES := $(BUILDDIR)/fileutils.o

# Object files
OBJFILES := $(patsubst $(SRCDIR)/%.c, $(BUILDDIR)/%.o, $(SRCFILES)) $(AOBJFILES)

# This is the header file that will be provided provided
HEADER :=



test: $(OBJFILES) $(BUILDDIR)/test.o
	$(CC) $(CFLAGS) $^ -o $(RUNDIR)/test.out



# update all submodules
update_sub:
	git submodule update --recursive --remote



# Compile submodules
submod: update_sub
	# Path
	(cd $(LIBRARIES)/utils/path && make export)
	(cp $(LIBRARIES)/utils/path/target/*.h $(HEADDIR)/utils/)
	(cp $(LIBRARIES)/utils/path/target/*.o $(BUILDDIR)/)
	# Fileutils
	(cd $(LIBRARIES)/utils/fileutils && make export)
	(cp $(LIBRARIES)/utils/fileutils/target/*.h $(HEADDIR)/utils/)
	(cp $(LIBRARIES)/utils/fileutils/target/*.o $(BUILDDIR)/)




# Target for test file
$(BUILDDIR)/test.o: $(TESTDIR)/test.c
	$(CC) $(CFLAGS) -c $< -o $@




# Target for Build files
$(BUILDDIR)/%.o: $(SRCDIR)/%.c
	$(CC) $(CFLAGS) -c $< -o $@


