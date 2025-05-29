/* Libraries */
#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>

/* Other files */
#include "include/move.h"
#include "include/custom.h"
#include "include/customized.h"
#include "include/def.h"
#include "include/utils/dict.h"
#include "include/ignored.h"


static int _move_cfg(const char *src_par, const char *dest_par, Dict *ignored, TRANSFER mode_of_transfer, bool hide) {
    DIR *directory = opendir(src_par);
    struct dirent *cfg_content;
    while ((cfg_content = readdir(directory)) != NULL) {
        /* Check if file is supposed to be ignored */
        Ignored *ig;
        if ((ig = dict_get(ignored, cfg_content->d_name))) {
            ignored_apply(ig, mode_of_transfer);
            continue;
        }

        /* Build src */
        size_t srclen = strlen(src_par) + 1 + strlen(cfg_content->d_name);
        char src[srclen + 1];
        snprintf(src, srclen + 1, "%s/%s", src_par, cfg_content->d_name);


        /* Build dest */
        size_t destlen = strlen(dest_par) + 2 + strlen(cfg_content->d_name);
        char dest[destlen + 1];
        if(hide)
            snprintf(dest, destlen + 1, "%s/.%s", dest_par, cfg_content->d_name);
        else
            snprintf(dest, destlen + 1, "%s/%s", dest_par, cfg_content->d_name);


        printf("%s to %s\n", src, dest);

        /* If file exists, skip for now */

        /*
         * TODO: create a backup of of customized files
         *       Then delete them if necessary
         */
        if(file_exists(dest)) {
            continue;
        }


        switch(mode_of_transfer) {
            case NOTHING:
                closedir(directory);
                return 0;
                break;
            case COPY:
                copy_dir_r(src, dest);
                break;
            case LINK:
                if (file_exists(dest))
                    break;
                symlink(src, dest);
                break;
        }
        printf("%s to %s\n", src, dest);
    }
    closedir(directory);



    return 0;
}



int move_cfg(TRANSFER mode_of_transfer) {
  /* Define config src directory */
  int cfg_src_len = strlen(getenv("HOME")) + strlen("/Jazzian/cfg_files");
  char cfg_src[cfg_src_len + 1];
  snprintf(cfg_src, cfg_src_len + 1, "%s/Jazzian/cfg_files", getenv("HOME"));


  /* Define script directory */
  int binsrc_len = strlen(getenv("HOME")) + strlen("/Jazzian/bin");
  char binsrc[binsrc_len + 1];
  snprintf(binsrc, binsrc_len + 1, "%s/Jazzian/bin", getenv("HOME"));


  /* Define script directory */
  int shell_len = strlen(getenv("HOME")) + strlen("/Jazzian/cfg_files/shell");
  char shellsrc[shell_len + 1];
  snprintf(shellsrc, shell_len + 1, "%s/Jazzian/cfg_files/shell", getenv("HOME"));


  /* Define config dest directory */
  int cfg_dest_len = strlen(getenv("HOME")) + strlen("/.config");
  char cfg_dest[cfg_dest_len + 1];
  snprintf(cfg_dest, cfg_src_len + 1, "%s/.config", getenv("HOME"));
  if(!file_exists(cfg_dest))
      mkdir(cfg_dest, 0755);


  /* Define local dest directory */
  int local_len = strlen(getenv("HOME")) + strlen("/.local");
  char local[local_len + 1];
  snprintf(local, local_len + 1, "%s/.local", getenv("HOME"));


  /* Define local_bin dest directory */
  int localbin_len = strlen(getenv("HOME")) + strlen("/.local/bin");
  char localbin[localbin_len + 1];
  snprintf(localbin, localbin_len + 1, "%s/.local/bin", getenv("HOME"));
  if(!file_exists(localbin))
      mkdir_r(localbin, 0755);


  /* Define ignored files */
  Ignored *passgen = ignored_init("passgen", NULL, NULL);
  Ignored *shell = ignored_init("shell", NULL, NULL);
  Ignored *qutebrowser = ignored_init("qutebrowser", NULL, NULL);
  Ignored *code = ignored_init("code", NULL, NULL);
  Ignored *powersave = ignored_init("powersave", NULL, NULL);
  Ignored *this = ignored_init(".", NULL, NULL);
  Ignored *parent = ignored_init("..", NULL, NULL);
  Ignored *vim = ignored_init("vim", "/Jazzian/cfg_files/vim", "/.vim");


  /* insert ignored files */
  Dict *ignoredfiles = dict_init();
  dict_insert(ignoredfiles, "passgen", passgen);
  dict_insert(ignoredfiles, "shell", shell);
  dict_insert(ignoredfiles, "qutebrowser", qutebrowser);
  dict_insert(ignoredfiles, "code", code);
  dict_insert(ignoredfiles, "powersave", powersave);
  dict_insert(ignoredfiles, "vim", vim);
  dict_insert(ignoredfiles, ".", this);
  dict_insert(ignoredfiles, "..", parent);


  /* Move the main config files */
  _move_cfg(cfg_src, cfg_dest, ignoredfiles, mode_of_transfer, false);

  /* Move the scripts */
  _move_cfg(binsrc, localbin, ignoredfiles, mode_of_transfer, false);

  /* Move shell files */
  _move_cfg(shellsrc, getenv("HOME"), ignoredfiles, mode_of_transfer, true);


  /* Free individual Ignored structs */
  dict_action(ignoredfiles, (void (*) (void *))ignored_free);


  /* Free the actual dictionary */
  dict_free(ignoredfiles);

  return 0;
}
