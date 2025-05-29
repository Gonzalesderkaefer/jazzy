/* Other files */
#include "include/utils/file_utils.h"
#include <sys/types.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <dirent.h>
#include <stdbool.h>
#include <errno.h>

int mkdir_r(char const *pathname, mode_t mode) {
    /* Copy String to array */
    size_t pathlen = strlen(pathname);
    char path[pathlen + 1];
    snprintf(path, pathlen + 1, "%s", pathname);


    /* tokenize */
    char *curPath = (char *)calloc(strlen("/") + 1, sizeof(char));
    if (!curPath)
        return -1;
    curPath[0] = '/';


    char *token , *rest;
    rest = path;
    while ((token = strtok_r(rest, "/", &rest))) {
        size_t newpathlen = strlen(curPath) + strlen(token) + 1;
        /* Allocate new path */
        char *newpath = (char *)calloc(newpathlen + 1, sizeof(char));
        if (!newpath) {
            free(curPath);
            return -1;
        }
        /* Copy to new string */
        snprintf(newpath, newpathlen + 1, "%s%s/", curPath, token);

        /* free curpath */
        free(curPath);
        curPath = newpath;

        mkdir(curPath, mode);
    }

    free(curPath);
    return 0;
}



void copy_file(char *src_file, char *dest_file, mode_t mode) {
  /* Open source file for reading */
  FILE *srcfp = fopen(src_file, "r");
  fseek(srcfp, 0, SEEK_END);

  /* Get size */
  int file_size = ftell(srcfp);
  fseek(srcfp, 0, SEEK_SET);

  /* Data buffer */
  char data[file_size + 3];

  /* Read data into buffer */
  fread(data, sizeof(char), file_size, srcfp);

  /* Open file for writing */
  FILE *destfp = fopen(dest_file, "w");
  fwrite(data, sizeof(char), file_size, destfp);

  fclose(srcfp);
  chmod(dest_file, mode);
}

void write_to_file(char *data, int length, const char *file_path,
                   const char *modes, mode_t mode) {
  /* Open file for writing */
  FILE *file = fopen(file_path, modes);

  /* Write into the file */
  fwrite(data, sizeof(char), length, file);

  /* Close file */
  fclose(file);

  /* Change file permissions */
  chmod(file_path, mode);
}


int rmfile(const char *fpath, const struct stat *s, int typeflag,
           struct FTW *ftwbuf) {
  struct stat statbuf;
  lstat(fpath, &statbuf);
  if (S_ISLNK(statbuf.st_mode) || S_ISREG(statbuf.st_mode)) {
    unlink(fpath);
  } else if (S_ISDIR(statbuf.st_mode)) {
    rmdir(fpath);
  }

  return 0;
}

int rm_dir(char *path) {
  nftw(path, rmfile, 10, 0);
  nftw(path, rmfile, 10, 0);
  rmdir(path);
  return 0;
}




static int _copy_dir_r(char *src_dir, char const *dest_dir) {
    DIR *directory = opendir(src_dir);
    struct dirent *dircontent;
    while ((dircontent = readdir(directory)) != NULL) {
        /* Don't include ".." and "." */
        if (!(strcmp(dircontent->d_name, "..") && strcmp(dircontent->d_name, ".")))
            continue;

        /* Build src */
        uint32_t src_len = strlen(src_dir) + 1 + strlen(dircontent->d_name);
        char src[src_len + 1];
        snprintf(src, src_len + 1, "%s/%s", src_dir, dircontent->d_name);

        /* Build dest */
        uint32_t dest_len = strlen(dest_dir) + 1 + strlen(dircontent->d_name);
        char dest[dest_len + 1];
        snprintf(dest, dest_len + 1, "%s/%s", dest_dir, dircontent->d_name);


        /* if dest already exists, skip */
        if (file_exists(dest)) continue;



        /* Determine type of src */
        struct stat src_stat;
        lstat(src, &src_stat);

        if (S_ISDIR(src_stat.st_mode)) {
            if (access(src, R_OK) == -1) continue; /* If we can't access don't do anything */
            puts("Creating new dir in dest");
            mkdir(dest, src_stat.st_mode);
            _copy_dir_r(src,dest);
        } else {
            if (access(src, R_OK) == -1) continue; /* If we can't access don't do anything */
            printf("%s to %s\n", src, dest);
            copy_file(src, dest, src_stat.st_mode);
        }
    }

    closedir(directory);
    return 0;
}





int copy_dir_r(char *src_dir, char const *dest_dir) {
    /* Sanity Check */
    if (!src_dir || !dest_dir || *src_dir != '/' || *dest_dir != '/')
        return -1;

    /* Sanitize src_dir */
    char *clean_src;
    char src[strlen(src_dir)];
    /* Check if there's / at the end if so remove it */
    if (src_dir[strlen(src_dir) - 1] == '/'){
        memset(src, '\0', strlen(src_dir));
        strncpy(src, src_dir, strlen(src_dir) - 1);
        clean_src = src;
    } else {
        clean_src = src_dir;
    }

    /* Check if src is directory */
    struct stat srcst;
    lstat(clean_src, &srcst);
    if (!S_ISDIR(srcst.st_mode) || !file_exists(clean_src))
        return -1;

    /* This will hold the actual dest base dir */
    char *actual_dest = (char *)dest_dir;
    /* Check if dest_dir is a parent */
    if (dest_dir[strlen(dest_dir) - 1] == '/') {
        /* Get dir name */
        size_t srclen = strlen(clean_src);
        char src[srclen + 1]; /* Otherwise strtok_r will SEGV */
        snprintf(src, srclen + 1, "%s", clean_src);
        char *rest, *token, *pretok;
        rest = src;
        while((token = strtok_r(rest, "/", &rest)))
            pretok = token;

        size_t new_dest_len = strlen(dest_dir) + strlen(pretok);
        char *new_dest =(char *)calloc(new_dest_len + 1, sizeof(char));
        snprintf(new_dest, new_dest_len + 1, "%s%s", dest_dir, pretok);
        actual_dest = new_dest;

        /* Check if new dest exists */
        if (file_exists(new_dest)) {
            free(new_dest);
            return -1;
        }
    } else if (file_exists(dest_dir)) {
        /* Check if new dest exists */ 
        return -1;
    }
    mkdir(actual_dest, srcst.st_mode);





    int cpystat = _copy_dir_r(clean_src, actual_dest);

    if (actual_dest != dest_dir)
        free(actual_dest);

    return cpystat;
}


