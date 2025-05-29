#include "include/backup.h"
#include "include/utils/file_utils.h"
#include <stdio.h>
#include <string.h>
#include <sys/stat.h>

#define HOME getenv("HOME")


static void _move_backup(const char *srcfile, const char *destfile) {
    /* Build src */
    size_t srclen = strlen(srcfile) + strlen(HOME);
    char src[srclen + 1];
    snprintf(src, srclen + 1, "%s%s", HOME, srcfile);

    /* Build dest */
    size_t destlen = strlen(destfile) + strlen(HOME);
    char dest[destlen + 1];
    snprintf(dest, destlen + 1, "%s%s", HOME, destfile);


    /* Determine filetype */
    struct stat srcstat;
    lstat(src, &srcstat);

    if(S_ISDIR(srcstat.st_mode)) {
        /* Check if directory exists and delete if it does */
        if (file_exists(dest))
            rm_dir(dest);

        copy_dir_r(src, dest);
    } else {
        /* Check if file exists and delete if it does */
        if (file_exists(dest))
            unlink(dest);

        copy_file(src,dest,srcstat.st_mode);
    }
}



void backup_cfgs() {

    /* Backup dir */
    size_t backuplen = strlen(HOME) + strlen("/myBackup");
    char backupdir[backuplen];
    snprintf(backupdir, backuplen + 1, "%s/myBackup", HOME);
    if (!file_exists(backupdir)) 
        mkdir(backupdir, 0755);




    /* Customized files for window managers/compositors */
    _move_backup("/.config/sway/devicespecific", "/myBackup/sway");
    _move_backup("/.config/hypr/devicespecific", "/myBackup/hypr");
    _move_backup("/.config/river/devicespecific", "/myBackup/river");
    _move_backup("/.config/i3/devicespecific", "/myBackup/i3");
    _move_backup("/.config/awesome/devicespecific", "/myBackup/awesome");
    _move_backup("/.config/bspwm/devicespecific", "/myBackup/bspwm");

    /* Customized shell files */
    _move_backup("/.devicespecific.sh", "/myBackup/devicespecific.sh");
    _move_backup("/.devicerc", "/myBackup/devicerc");

    /* X11 startup */
    _move_backup("/.local/bin/x11startup", "/myBackup/x11startup");


    /* xinitrc */
    _move_backup("/.xinitrc", "/myBackup/xinitrc");
}

void restore_cfgs() {
    /* Backup dir */
    size_t backuplen = strlen(HOME) + strlen("/myBackup");
    char backupdir[backuplen];
    snprintf(backupdir, backuplen + 1, "%s/myBackup", HOME);
    if (!file_exists(backupdir)) return;


    /* Customized files for window managers/compositors */
    _move_backup("/myBackup/sway", "/.config/sway/devicespecific");
    _move_backup("/myBackup/hypr", "/.config/hypr/devicespecific");
    _move_backup("/myBackup/river", "/.config/river/devicespecific");
    _move_backup("/myBackup/i3", "/.config/i3/devicespecific");
    _move_backup("/myBackup/awesome", "/.config/awesome/devicespecific");
    _move_backup("/myBackup/bspwm", "/.config/bspwm/devicespecific");

    /* Customized shell files */
    _move_backup("/myBackup/devicespecific.sh", "/.devicespecific.sh");
    _move_backup("/myBackup/devicerc", "/.devicerc");

    /* X11 startup */
    _move_backup("/myBackup/x11startup", "/.local/bin/x11startup");


    /* xinitrc */
    _move_backup("/myBackup/xinitrc", "/.xinitrc");
}
