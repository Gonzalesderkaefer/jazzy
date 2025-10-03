pub const DEB_GUI: &'static[&'static str] = &[
    "evince",
    "network-manager",
    "fonts-jetbrains-mono",
    "mpv",
    "zathura",
    "papirus-icon-theme",
    "gnome-themes-extra",
    "arc-theme",
    "libnotify-bin",
    "acpi-support",
    "acpid",
    "acpi",
    "linux-cpupower",
    "openssh-server",
    "firefox-esr",
    "tlp",
    "alacritty",
    "pipewire",
    "pipewire-alsa",
    "pipewire-pulse",
    "libglib2.0-bin",
    "network-manager-gnome",
    "thunar",
    "file-roller",
    "network-manager-openconnect-gnome",
    "eom",
    "network-manager-openconnect",
    "sddm",
    "polkitd",
    "wireplumber",
    "rtkit",
];


pub const DEB_BASE: &'static[&'static str] = &[
    "xz-utils",
    "curl",
    "git",
    "tmux",
    "vifm",
    "zsh",
    "flatpak",
    "git",
    "nala",
    "nnn",
    "fzf",
    "jq",
];


pub const _DEB_XORG: &'static[&'static str] = &[
    "lxappearance",
    "maim",
    "arandr",
    "rofi",
    "xclip",
    "i3lock",
    "picom",
    "dunst",
    "xinput",
    "xorg",
    "xwallpaper",
    "rxvt-unicode",
    "nitrogen",
];

pub const DEB_XORG: &'static[&'static str] = crate::concat_packages!(&[&str]: DEB_GUI, _DEB_XORG);


pub const _DEB_WAY: &'static[&'static str] = &[
    "grim",
    "swaylock",
    "wofi",
    "xwayland",
    "wlr-randr",
    "wl-clipboard",
    "swayidle",
    "mako-notifier",
    "slurp",
];



pub const DEB_WAY: &'static[&'static str] = crate::concat_packages!(&[&str]: DEB_GUI, _DEB_WAY);

pub const DEB_I3: &'static[&'static str] = &[
    "i3",
    "i3blocks",
];

pub const DEB_BSP: &'static[&'static str] = &[
    "bspwm",
    "sxhkd",
    "polybar",
];

/* Debian awesome packages */
pub const DEB_AWE: &'static[&'static str] = &[
    "awesome",
];


pub const DEB_SWAY: &'static[&'static str] = &[
    "sway",
    "i3blocks",
];





pub const FED_GUI: &'static[&'static str] = &[
    "flatpak",
    "pinentry",
    "alacritty",
    "pipewire",
    "jetbrains-mono-fonts",
    "papirus-icon-theme-dark",
    "network-manager-applet",
    "arc-theme",
    "pipewire-utils",
    "file-roller",
    "pipewire-pulseaudio",
    "NetworkManager-openconnect-gnome",
    "gsettings-desktop-schemas",
    "papirus-icon-theme",
    "NetworkManager-tui",
    "eom",
    "tlp",
    "libnotify",
    "pipewire-alsa",
    "qalculate-gtk",
    "mpv",
    "firefox",
    "zathura",
    "zathura-pdf-poppler",
    "evince",
    "thunar",
    "mate-polkit",
    "rofi-wayland",
    "sddm",
];




pub const FED_BASE: &'static[&'static str] = &[
    "xz",
    "curl",
    "git",
    "vifm",
    "tmux",
    "zsh",
    "nnn",
    "neovim",
    "git",
    "fzf",
    "jq",
];



pub const _FED_XORG: &'static[&'static str] = &[
    "xclip",
    "@base-x",
    "maim",
    "lxappearance",
    "xinput",
    "arandr",
    "nitrogen",
    "picom",
    "dunst",
    "xclip",
    "i3lock",
    "rxvt-unicode",
    "nitrogen",
];
pub const FED_XORG: &'static[&'static str] = crate::concat_packages!(&[&str]: FED_GUI, _FED_XORG);


pub const _FED_WAY: &'static[&'static str] = &[
    "grim",
    "swaybg",
    "swayidle",
    "waybar",
    "wl-clipboard",
    "swaylock",
    "mako",
    "slurp",
];
pub const FED_WAY: &'static[&'static str] = crate::concat_packages!(&[&str]: FED_GUI, _FED_WAY);



pub const FED_AWE: &'static[&'static str] = &[
    "awesome",
];

pub const FED_BSP: &'static[&'static str] = &[
    "bspwm",
    "polybar",
    "sxhkd",
];

pub const FED_I3: &'static[&'static str] = &[
    "i3",
    "i3blocks",
];


pub const FED_HYPR: &'static[&'static str] = &[
    "hyprland",
    "waybar",
];

pub const FED_RIV: &'static[&'static str] = &[
    "river",
    "waybar",
];

pub const FED_SWAY: &'static[&'static str] = &[
    "sway",
    "i3blocks",
];

pub const FED_NIR: &'static[&'static str] = &[
    "niri",
    "waybar",
];




pub const ARCH_GUI: &'static[&'static str] = &[
    "rofi-wayland",
    "zathura-pdf-poppler",
    "tlp",
    "gcr",
    "alacritty",
    "pipewire",
    "pipewire-pulse",
    "cpupower",
    "pipewire-alsa",
    "wireplumber",
    "mpv",
    "gsettings-desktop-schemas",
    "eom",
    "network-manager-applet",
    "openconnect",
    "lxappearance",
    "file-roller",
    "papirus-icon-theme",
    "gnome-themes-extra",
    "ttf-jetbrains-mono-nerd",
    "ttf-jetbrains-mono",
    "zathura",
    "evince",
    "webkit2gtk-4.1",
    "networkmanager-openconnect",
    "firefox",
    "networkmanager",
    "thunar",
    "nm-connection-editor",
    "sddm",
];

pub const ARCH_BASE: &'static[&'static str] = &[
    "xz",
    "curl",
    "git",
    "flatpak",
    "fzf",
    "tmux",
    "zsh",
    "nnn",
    "neovim",
    "bash-completion",
    "zsh-completions",
    "less",
    "wget",
    "git",
    "jq",
];



pub const _ARCH_XORG: &'static[&'static str] = &[
    "xorg",
    "lxappearance",
    "xwallpaper",
    "maim",
    "picom",
    "xorg-xinput",
    "xorg-xinit",
    "xclip",
    "i3lock",
    "rxvt-unicode",
    "nitrogen",
];
pub const ARCH_XORG: &'static[&'static str] = crate::concat_packages!(&[&str]: ARCH_GUI, _ARCH_XORG);


pub const _ARCH_WAY: &'static[&'static str] = &[
    "grim",
    "swaybg",
    "waybar",
    "swayidle",
    "swaylock",
    "wl-clipboard",
    "mako",
    "slurp",
];
pub const ARCH_WAY: &'static[&'static str] = crate::concat_packages!(&[&str]: ARCH_GUI, _ARCH_WAY);



pub const ARCH_AWE: &'static[&'static str] = &[
    "awesome",
];

pub const ARCH_BSP: &'static[&'static str] = &[
    "bspwm",
    "polybar",
    "sxhkd",
];

pub const ARCH_I3: &'static[&'static str] = &[
    "i3",
    "i3blocks",
];


pub const ARCH_HYPR: &'static[&'static str] = &[
    "hyprland",
    "waybar",
];

pub const ARCH_RIV: &'static[&'static str] = &[
    "river",
    "waybar",
];

pub const ARCH_SWAY: &'static[&'static str] = &[
    "sway",
    "i3blocks",
];

pub const ARCH_NIR: &'static[&'static str] = &[
    "niri",
    "waybar",
];
