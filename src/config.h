#ifndef CONFIG_H
#define CONFIG_H
// Libraries
#include "types/customized/customized.h"
#include "types/distro/distro.h"
#include "packages.h"
#include <stdlib.h>
#include <stddef.h>

/// # Config file for jazzy
/// This file defines various constans for the
/// jazzy installer such as config source and destination
/// and more..
///
/// File paths are relative to $HOME and cannot start with '/'

/// This is the home dir and will be defined in
/// in main.c
extern char *home_dir;

/// ## Config files
/// Move everything in $HOME/Jazzian/dotfiles/config/ into $HOME/.config/
static const char *config_src = "Jazzian/dotfiles/config/";
static const char *config_dest = ".config/";

/// ## Scripts and executables
/// Move everything in $HOME/Jazzian/dotfiles/bin into $HOME/.local/bin/
static const char *bin_src = "Jazzian/dotfiles/bin/";
static const char *bin_dest = ".local/bin/";

/// ## Customized files
/// ### Predifined file contens
#define customized_sh "killshells() {\n" \
    "    pkill -KILL -u $USER -t tty1\n" \
    "}\n" \
    "export BROWSER_PREFIX=\"firefox\"\n" \
    "[ \"$(tty)\" = \"/dev/tty1\" ] && (startx; killshells)\n"

#define debmdmenu "#!/usr/bin/sh\n" \
                  "if [ $XDG_SESSION_TYPE = \"wayland\" ]; then\n" \
                  "    exec wofi_dmenu;\n" \
                  "else\n" \
                  "    exec rofi_dmenu;\n" \
                  "fi\n"; \

#define debmdrun "#!/usr/bin/sh\n" \
                 "if [ $XDG_SESSION_TYPE = \"wayland\" ]; then\n" \
                 "    exec wofi_app;\n" \
                 "else\n" \
                 "    exec rofi_app;\n" \
                 "fi\n" \

#define mdmenu_content "#!/usr/bin/sh\n" \
                       "exec rofi_dmenu\n" \

#define mdrun_content "#!/usr/bin/sh\n" \
                      "exec rofi_app\n" \

#define myterm_content "#!/bin/sh\n" \
                       "case $XDG_SESSION_TYPE in\n" \
                       "    \"wayland\")\n" \
                       "        exec alacritty -o font.size=12 $@\n" \
                       "        ;;\n" \
                       "    *)\n" \
                       "        exec alacritty -o font.size=12 $@\n" \
                       "        ;;\n" \
                       "esac\n" \

#define startx_content "x11startup &\nexec i3\n"

#define x11startup "#!/usr/bin/bash\n"


#define gtk3_config "[Settings]\n" \
                    "gtk-theme-name=Adwaita-dark\n" \
                    "gtk-icon-theme-name=Papirus-Dark\n" \
                    "gtk-font-name=JetBrains Mono Light 12\n" \
                    "gtk-cursor-theme-size=0"


/// These files will be created by the installer
// TODO: expand these!!
static const Customized custom_files[] = {
    {.path = ".customenv", .content = "", .permission = 0644},
    {.path = ".customized.sh", .content = customized_sh, .permission = 0644},

    {.path = ".config/i3/customzied/customzied", .content =  "", .permission = 0644},
    {.path = ".config/bspwm/customzied/customzied", .content =  "", .permission = 0644},
    {.path = ".config/awesome/customzied/customzied.lua", .content =  "", .permission = 0644},
    {.path = ".config/sway/customzied/customzied", .content =  "", .permission = 0644},
    {.path = ".config/hypr/customzied/customzied", .content =  "", .permission = 0644},
    {.path = ".config/river/customzied/customzied", .content =  "", .permission = 0644},


    // Customized shell files
    {.path = ".customrc", .content =  "", .permission = 0644},
    {.path = ".customenv", .content =   "export BROWSER_PREFIX=\"firefox\"", .permission = 0644},


    // X11 Startup
    {.path = ".local/bin/x11startup", .content =  x11startup, .permission = 0755},


    // Xinitrc
    {.path = ".xinitrc", .content =  startx_content, .permission = 0644},


    // Myterm
    {.path = ".local/bin/myterm", .content =  myterm_content, .permission = 0755},


    // GTK3
    {.path = ".config/gtk-3.0/settings.ini", .content =  gtk3_config, .permission = 0644},


    // Menus
    {.path = ".local/bin/mdmenu", .content =  mdmenu_content, .permission = 0755},
    {.path = ".local/bin/mdrun", .content =  mdrun_content, .permission = 0755},
    {.path = NULL, .content = NULL, .permission = 0}, // The 'NULL terminator'
};








#endif // CONFIG_H
