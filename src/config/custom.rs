/// This file defines all the filepaths and contents
/// for custom configuration that is specific for each device

/// Predifined file contents
pub static DEBMDMENU: &'static str = "#!/usr/bin/sh\n\
                  if [ $XDG_SESSION_TYPE = \"wayland\" ]; then\n\
                      exec wofi_dmenu;\n\
                  else\n\
                      exec rofi_dmenu;\n\
                  fi\n";

pub static DEBMDRUN: &'static str = "#!/usr/bin/sh\n\
                 if [ $XDG_SESSION_TYPE = \"wayland\" ]; then\n\
                     exec wofi_app;\n\
                 else\n\
                     exec rofi_app;\n\
                 fi\n";

pub static MDMENU_CONTENT: &'static str = "#!/usr/bin/sh\n\
                       exec rofi_dmenu\n";

pub static MDRUN_CONTENT: &'static str = "#!/usr/bin/sh\n\
                      exec rofi_app\n";

pub static MYTERM_CONTENT: &'static str = "#!/bin/sh\n\
                       case $XDG_SESSION_TYPE in\n\
                           \"wayland\")\n\
                               exec alacritty -o font.size=12 $@\n\
                               ;;\n\
                           *)\n\
                               exec alacritty -o font.size=12 $@\n\
                               ;;\n\
                       esac\n";


pub static X11STARTUP: &'static str = "#!/usr/bin/bash\n";


pub static GTK3_CONFIG: &'static str = "[Settings]\n\
                    gtk-theme-name=Adwaita-dark\n\
                    gtk-icon-theme-name=Papirus-Dark\n\
                    gtk-font-name=JetBrains Mono Light 12\n\
                    gtk-cursor-theme-size=0";
// Customized files
pub static CUSTOMIZED: &'static [(&str, &str, u32)] =
&[
    // Customized files for window managers/compositors
    (".config/i3/customized/customized", "", 0o644),
    (".config/bspwm/customized/customized", "", 0o644),
    (".config/awesome/customized/customized.lua", "", 0o644),
    (".config/sway/customized/customized", "", 0o644),
    (".config/hypr/customized/customized", "", 0o644),
    (".config/river/customized/customized", "", 0o644),
    (".config/niri/customized/customized.kdl", "", 0o644),

    // Customized shell files
    (".customized.sh", "", 0o644),
    (".customrc", "", 0o644),
    (".customenv",  "export BROWSER_PREFIX=\"firefox\"", 0o644),

    // X11 Startup
    (".local/bin/x11startup", X11STARTUP, 0o755),

    // Myterm
    (".local/bin/myterm", MYTERM_CONTENT, 0o755),

    // GTK3
    (".config/gtk-3.0/settings.ini", GTK3_CONFIG, 0o644),
];
