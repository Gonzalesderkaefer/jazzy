
// Header file
#include "headers/types/customized.h"

// Libraries
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include "headers/utils/fileutils.h"
#include "headers/utils/path.h"


void create_customized(const char *path, const char *content, const mode_t perm) {
    // Check if path exists
    if (!access(path, F_OK))
        return;

    puts(path);

    // check if path is directory
    if (path[strlen(path) - 1] == '/') {
        mkdir_p(path, perm); // create all directories
        return;
    }

    // Create new path string
    Path *new_path = path_init(path);
    if (!new_path) { // Error checking
        return;
    }

    // Get parent of new_path to create the path
    const char *new_parent = path_parent(new_path);

    // Create new_path's parent dir
    if (mkdir_p(new_parent, perm) == -1) { // Error checking
        path_free(new_path); // We have to free this...
        return;
    }

    // Open file
    FILE *pathfp = fopen(path, "w");
    if (!pathfp) { // Error checking
        path_free(new_path); // We have to free this...
        return;
    }

    // Write to the file 
    fwrite(content, sizeof(char) , strlen(content), pathfp);


    // Cleanup
    fclose(pathfp);
    path_free(new_path);
}
