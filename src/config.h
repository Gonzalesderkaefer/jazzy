#ifndef CONFIG_H
#define CONFIG_H
// Libraries
#include "types/customized/customized.h"
#include "types/distro/distro.h"
#include "types/display/windowmanager.h"
#include "types/display/dspserver.h"
#include "packages.h"
#include <stddef.h>
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


    /// Customized files for window managers and compositors
    {.path = ".config/i3/customzied/customzied", .content =  "", .permission = 0644},
    {.path = ".config/bspwm/customzied/customzied", .content =  "", .permission = 0644},
    {.path = ".config/awesome/customzied/customzied.lua", .content =  "", .permission = 0644},
    {.path = ".config/sway/customzied/customzied", .content =  "", .permission = 0644},
    {.path = ".config/hypr/customzied/customzied", .content =  "", .permission = 0644},
    {.path = ".config/river/customzied/customzied", .content =  "", .permission = 0644},


    /// Customized shell files
    {.path = ".customrc", .content =  "", .permission = 0644},
    {.path = ".customenv", .content =   "export BROWSER_PREFIX=\"firefox\"", .permission = 0644},
    {.path = ".customized.sh", .content = customized_sh, .permission = 0644},


    /// X11 Startup
    {.path = ".local/bin/x11startup", .content =  x11startup, .permission = 0755},


    /// Xinitrc
    {.path = ".xinitrc", .content =  startx_content, .permission = 0644},


    /// Myterm
    {.path = ".local/bin/myterm", .content =  myterm_content, .permission = 0755},


    /// GTK3
    {.path = ".config/gtk-3.0/settings.ini", .content =  gtk3_config, .permission = 0644},


    /// Menus
    {.path = ".local/bin/mdmenu", .content =  mdmenu_content, .permission = 0755},
    {.path = ".local/bin/mdrun", .content =  mdrun_content, .permission = 0755},
    {.path = NULL, .content = NULL, .permission = 0}, // The 'NULL terminator'
};


/// Predefined distros
static const Distro debian = {
    .install = (const char *[]) {"sudo", "apt", "install", "-y", NULL},
    .update = (const char *[]) {"sudo", "apt", "update", "-y", NULL},
    .upgrade = (const char *[]) {"sudo", "apt", "upgrade", "-y", NULL},
    .suffix = NULL,
    .basepkg = debian_base,
};


static const Distro fedora = {
    .install = (const char *[]) {"sudo", "dnf", "install", "-y", NULL},
    .update = (const char *[]) {"sudo", "dnf", "update", "-y", NULL},
    .upgrade = NULL,
    .suffix = NULL,
    .basepkg = fedora_base,
};


static const Distro arch = {
    .install = (const char *[]) {"sudo", "pacman", "-S", "--noconfirm", "--needed", NULL},
    .update = (const char *[]) {"sudo", "pacman", "Syu", "--noconfirm", "--needed", NULL},
    .upgrade = NULL,
    .suffix = (const char *[]) {"--noconfirm", "--needed", NULL},
    .basepkg = arch_base,
};


/// Predefined Window managers
static const WindowManager i3 = {
    .packages = (const char **[]){debian_i3, fedora_i3, arch_i3, NULL}
};

static const WindowManager awesome = {
    .packages = (const char **[]){debian_awesome, fedora_awesome, arch_awesome, NULL}
};

static const WindowManager bspwm = {
    .packages = (const char **[]){debian_bspwm, fedora_bspwm, arch_bspwm, NULL}
};

static const WindowManager sway = {
    .packages = (const char **[]){debian_sway, fedora_sway, arch_sway, NULL}
};

static const WindowManager hypr = {
    .packages = (const char **[]){debian_sway, fedora_hypr, arch_hypr, NULL}
};

static const WindowManager river = {
    .packages = (const char **[]){debian_sway, fedora_river, arch_river, NULL}
};



/// Predefined display servers
static const DspServer xorg = {
    .packages = (const char **[]){debian_xorg, fedora_xorg, arch_xorg, NULL}
};

static const DspServer wayland = {
    .packages = (const char **[]){debian_wayland, fedora_wayland, arch_wayland, NULL}
};

// TODO: Add tty packages
static const DspServer tty = {
    .packages = (const char **[]){debian_wayland, fedora_wayland, arch_wayland, NULL}
};

#endif // CONFIG_H
