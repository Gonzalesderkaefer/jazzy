#include "include/custom.h"
#include "include/customized.h"
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h> 


/* Predifined file contents */
char *devicespecific_sh = "killshells() {\n"
    "    pkill -KILL -u $USER -t tty1\n"
    "}\n"
    "export BROWSER_PREFIX=\"firefox\"\n"
    "[ \"$(tty)\" = \"/dev/tty1\" ] && (startx; killshells)\n";

char *debmdmenu = "#!/usr/bin/sh\n"
                  "if [ $XDG_SESSION_TYPE = \"wayland\" ]; then\n"
                  "    exec wofi_dmenu;\n"
                  "else\n"
                  "    exec rofi_dmenu;\n"
                  "fi\n";

char *debmdrun = "#!/usr/bin/sh\n"
                 "if [ $XDG_SESSION_TYPE = \"wayland\" ]; then\n"
                 "    exec wofi_app;\n"
                 "else\n"
                 "    exec rofi_app;\n"
                 "fi\n";

char *mdmenu_content = "#!/usr/bin/sh\n"
                       "exec rofi_dmenu\n";

char *mdrun_content = "#!/usr/bin/sh\n"
                      "exec rofi_app\n";

char *myterm_content = "#!/bin/sh\n"
                       "case $XDG_SESSION_TYPE in\n"
                       "    \"wayland\")\n"
                       "        exec alacritty -o font.size=12 $@\n"
                       "        ;;\n"
                       "    *)\n"
                       "        exec alacritty -o font.size=12 $@\n"
                       "        ;;\n"
                       "esac\n";

char *startx_content = "x11startup &\nexec i3\n";

char *x11startup = "#!/usr/bin/bash\n";


char *gtk3_config = "[Settings]\n"
                    "gtk-theme-name=Adwaita-dark\n"
                    "gtk-icon-theme-name=Papirus-Dark\n"
                    "gtk-font-name=JetBrains Mono Light 12\n"
                    "gtk-cursor-theme-size=0";



static int edit_files(config *system);
static int set_theme();


void create_customized(config *system) {
    /* Customized files for window managers/compositors */
    customized("/.config/i3/devicespecific/devicespecific", "",-1, false);
    customized("/.config/bspwm/devicespecific/devicespecific", "",-1, false);
    customized("/.config/awesome/devicespecific/devicespecific.lua", "",-1, false);
    customized("/.config/sway/devicespecific/devicespecific", "",-1, false);
    customized("/.config/hypr/devicespecific/devicespecific", "",-1, false);
    customized("/.config/river/devicespecific/devicespecific", "",-1, false);

    /* Customized shell files */
    customized("/.devicerc", "",-1, false);
    customized("/.devicespecific.sh", devicespecific_sh,-1, false);

    /* X11 startup */
    customized("/.local/bin/x11startup", x11startup, 0755, true);

    /* xinitrc */
    customized("/.xinitrc", startx_content,-1, false);

    /* myterm */
    customized("/.local/bin/myterm", myterm_content, 0775, true);

    /* GTK 3 config */
    customized("/.config/gtk-3.0/settings.ini", gtk3_config, -1, false);




    /* Menus */
    switch (system->distro) {
        case DEBIAN:
            customized("/.local/bin/mdmenu",debmdmenu, 0755, true);
            customized("/.local/bin/mdrun",debmdrun, 0755, true);
            break;
        default:
            customized("/.local/bin/mdmenu",mdmenu_content, 0755, true);
            customized("/.local/bin/mdrun",mdrun_content, 0755, true);
            break;
    }

    edit_files(system);
}

int edit_files(config *system) {
  /* Deterimine new window manager */
  char *windowmanager;
  bool skip_xinit = false;

  switch (system->window_manager) {
  case AWESOME:
    windowmanager = "exec awesome";
    break;
  case I3:
    windowmanager = "exec i3";
    break;
  case BSPWM:
    windowmanager = "exec bspwm";
    break;
  case SWAY:
    windowmanager = "&& (sway;";
    skip_xinit = true;
    break;
  case RIVER:
    windowmanager = "&& (river;";
    skip_xinit = true;
    break;
  case HYPRLAND:
    windowmanager = "&& (Hyprland;";
    skip_xinit = true;
    break;
  }

  if (!skip_xinit) {
    /* xinitrc filename */
    int xinitlen = strlen(getenv("HOME")) + strlen(".xinitrc") + 1;
    char xinit[xinitlen + 1];
    snprintf(xinit,xinitlen + 1, "%s/.xinitrc", getenv("HOME"));

    /* Open file */
    FILE *startx = fopen(xinit, "r");

    /* Determine size */
    fseek(startx, 0, SEEK_END);
    size_t startx_buf_size = ftell(startx);
    fseek(startx, 0, SEEK_SET); /* Have to reset position */

    /* Read file */
    char startx_buf[startx_buf_size + 1];
    memset(startx_buf, '\0', startx_buf_size + 1);
    fread(startx_buf, sizeof(char), startx_buf_size, startx);
    fclose(startx);

    /* Substitute */
    char *newstartx_buf = search_replace(startx_buf, "exec i3|exec awesome|exec bspwm", windowmanager);
    if(!newstartx_buf) return -1;

    /* Open file for writing */
    startx = fopen(xinit, "w");
    fwrite(newstartx_buf, sizeof(char), strlen(newstartx_buf), startx);
    fclose(startx);

    windowmanager = "&& (startx;";

    /* free new substition */
    free(newstartx_buf);
  }

  /* devicespecific filename */
  int devspclen = strlen(getenv("HOME")) + strlen(".devicespecific.sh") + 1;
  char devspc[devspclen + 1];
  snprintf(devspc, devspclen + 1, "%s/.devicespecific.sh", getenv("HOME"));

  /* Open file */
  FILE *device = fopen(devspc, "r");

  /* Determine size */
  fseek(device, 0, SEEK_END);
  size_t device_buf_size = ftell(device);
  fseek(device, 0, SEEK_SET);


  /* Read from file */
  char device_buf[device_buf_size + 1];
  memset(device_buf, '\0', device_buf_size + 1);
  fread(device_buf, sizeof(char), device_buf_size, device);
  fclose(device);

  /* Substitute */
  char *newdevice_buf = search_replace(device_buf, "&& \\(.*;", windowmanager); /* Has to be freed!! */

  /* Reopen for writing */
  device = fopen(devspc, "w");
  fwrite(newdevice_buf, sizeof(char), strlen(newdevice_buf), device);
  fclose(device);

  free(newdevice_buf);

  set_theme();
  return 0;
}




int set_theme() {
    char *themecmd[] = { 
        "gsettings",
        "set",
        "org.gnome.desktop.interface",
        "gtk-theme",
        "\'Adwaita-dark\'",
        NULL
    };

    char *fontcmd[] = { 
        "gsettings",
        "set",
        "org.gnome.desktop.interface",
        "font-name",
        "\'Jetbrains Mono\'",
        NULL
    };

    char *iconcmd[] = { 
        "gsettings",
        "set",
        "org.gnome.desktop.interface",
        "icon-theme",
        "\'Papirus-Dark\'",
        NULL
    };

    char *colorcmd[] = {
    "gsettings",
    "set",
    "org.gnome.desktop.interface",
    "color-scheme",
    "\'prefer-dark\'",
    NULL
    };


    pid_t pidtheme = fork();

    switch (pidtheme) {
    case 0:
        execvp("gsettings", (char **)themecmd);
        break;
    case -1:
        fprintf(stderr, "fork failed\n");
        return -1;
        break;
    default:
        wait(NULL);
        break;
    }

    pid_t pidicon = fork();

    switch (pidicon) {
    case 0:
        execvp("gsettings", (char **)iconcmd);
        break;
    case -1:
        fprintf(stderr, "fork failed\n");
        return -1;
        break;
    default:
        wait(NULL);
        break;
    }

    pid_t pidfont = fork();

    switch (pidfont) {
    case 0:
        execvp("gsettings", (char **)fontcmd);
        break;
    case -1:
        fprintf(stderr, "fork failed\n");
        return -1;
        break;
    default:
        wait(NULL);
        break;
    }


    pid_t pidcolor = fork();

    switch (pidcolor) {
    case 0:
        execvp("gsettings", (char **)colorcmd);
        break;
    case -1:
        fprintf(stderr, "fork failed\n");
        return -1;
        break;
    default:
        wait(NULL);
        break;
    }

    return 0;
}
