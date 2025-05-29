#include <stdbool.h>
#include <stdio.h>
#include <string.h>
#include <stddef.h>
#include <stdlib.h>
#include <stdint.h>
#include <sys/stat.h>

/* Other files */
#include "include/utils/file_utils.h"


/* Headers */
#include "include/customized.h"


void customized(char *relpath, char *contents, mode_t mode, bool customperm) {
    /* Build absolute file path */
    size_t file_path_len = strlen(getenv("HOME")) + strlen(relpath);
    char filepath[file_path_len + 1];
    snprintf(filepath, file_path_len + 1, "%s%s", getenv("HOME"), relpath);

    /* Check if filepath exists */
    if (file_exists(filepath)) 
        return;

    /* Check whether dir or regular file */
    char end = filepath[strlen(filepath) - 1];
    if (end == '/' && !file_exists(filepath)) {
        mkdir_r(filepath, 0755);
    } else {
        /* Get last component */
        size_t fullpathlen = strlen(filepath);
        char fullpath[strlen(filepath) + 1];
        snprintf(fullpath, fullpathlen + 1, "%s", filepath);

        char *pretok, *token, *rest;
        rest = fullpath;
        while((token = strtok_r(rest, "/", &rest)))
            pretok = token;


        /* cut path */
        size_t cutpathlen = strlen(filepath) - strlen(pretok);
        char cutpath[cutpathlen + 1];
        memset(cutpath, '\0', cutpathlen + 1);
        strncpy(cutpath, filepath, cutpathlen);

        /* Create dir */
        if (!file_exists(cutpath))
            mkdir_r(cutpath, 0775);

        /* Write to file */
        FILE *fp = fopen(filepath, "w+");
        fwrite(contents, strlen(contents), sizeof(char), fp);
        fclose(fp);

        /* Set file mode */
        if (customperm)
            chmod(filepath, mode);
    }
}
