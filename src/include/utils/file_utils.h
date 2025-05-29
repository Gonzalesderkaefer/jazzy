#define _XOPEN_SOURCE 500
#ifndef FILE_UTILS_C
#define FILE_UTILS_C


/* Other files */
#include "../def.h"
#include<ftw.h>


#define file_exists(filepath) !access(filepath, F_OK)


int mkdir_r(char const *pathname, mode_t mode);

void copy_file(char *src_file, char *dest_file, mode_t mode);

void write_to_file(char *data, int length, const char *file_path, const char *modes, mode_t mode);

int copy_dir(char *src_dir, char *dest_parent, char **ill_cfg, bool hide);

int copy_dir_r(char *src_dir, char const *dest_dir);

int link_dir(char *src_dir, char *dest_dir, char **ill_cfg, bool hide);

int rm_dir(char *path);

#endif
