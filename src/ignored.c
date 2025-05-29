/* Libraries */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

/* Other files */
#include "include/utils/file_utils.h"

/* Header */
#include "include/ignored.h"



typedef struct _ignored {
    char *name;
    char *src;
    char *dest;
} Ignored;

typedef struct _IgnoredString{
    const char *string;
    const size_t len;
} IgnoredString;


Ignored *ignored_init(const char *name, const char *src, const char *dest) {
    /* Allocate struct */
    Ignored *new_ignored = (Ignored *)malloc(sizeof(Ignored));
    if (!new_ignored) return NULL;

    /* Allocate name */
    if (name) {
        size_t ignored_namelen = strlen(name);
        char *ignored_name = (char *)calloc(ignored_namelen + 1, sizeof(char));
        if (!ignored_name) {
            free(new_ignored);
            return NULL;
        }
        /* Copy string into buffer */
        snprintf(ignored_name, ignored_namelen + 1, "%s", name);

        /* Assign string to struct */
        new_ignored->name = ignored_name;
    } else {
        new_ignored->name = NULL;
    }

    /* Allocate src */
    if (src && *src == '/') {
        size_t ignored_srclen = strlen(getenv("HOME")) + strlen(src);
        char *ignored_src = (char *)calloc(ignored_srclen + 1, sizeof(char));
        if (!ignored_src) {
            /* free name */
            if (new_ignored->name)
                free(new_ignored->name);

            /* free Ignored struct */
            free(new_ignored);

            return NULL;
        }
        /* Copy string into buffer */
        snprintf(ignored_src, ignored_srclen + 1, "%s%s", getenv("HOME"), src);

        /* Assign string to struct */
        new_ignored->src = ignored_src;
    } else {
        new_ignored->src = NULL;
    }

    /* Allocate dest */
    if (dest && *dest == '/') {
        size_t ignored_destlen = strlen(getenv("HOME")) + strlen(dest);
        char *ignored_dest = (char *)calloc(ignored_destlen + 1, sizeof(char));
        if (!ignored_dest) {
            /* free name */
            if (new_ignored->name)
                free(new_ignored->name);

            /* free src */
            if (new_ignored->src)
                free(new_ignored->src);


            /* free Ignored struct */
            free(new_ignored);

            return NULL;
        }
        /* Copy string into buffer */
        snprintf(ignored_dest, ignored_destlen + 1, "%s%s", getenv("HOME"), dest);

        /* Assign string to struct */
        new_ignored->dest = ignored_dest;
    } else {
        new_ignored->dest = NULL;
    }

    return new_ignored;
}



void ignored_apply(Ignored *to_apply, TRANSFER method) {
    if (to_apply && to_apply->src && to_apply->dest)
        switch(method) {
            case NOTHING:
                return;
            case COPY:
                if (file_exists(to_apply->src) && !file_exists(to_apply->dest))
                    copy_dir_r(to_apply->src, to_apply->dest);
                break;
            case LINK:
                if (file_exists(to_apply->src) && !file_exists(to_apply->dest))
                    symlink(to_apply->src, to_apply->dest);
                break;
        }
}


void ignored_free(Ignored *to_delete) {
    /* Sanity check */
    if (!to_delete) return;

    /* Free fields */
    free(to_delete->dest);
    free(to_delete->src);
    free(to_delete->name);

    /* Free struct */
    free(to_delete);
}






