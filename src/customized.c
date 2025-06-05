
// Header file
#include "headers/types/customized.h"

// Libraries
#include <string.h>
#include "headers/utils/fileutils.h"


void create_customized(const char *path, const char *content, const mode_t perm) {
    // check if path is directory
    if (path[strlen(path) - 1] == '/') {
        mkdir_p(path, perm);
    }
}
