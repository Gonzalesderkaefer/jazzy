#ifndef DISTRO_H
#define DISTRO_H

/// These will be predefined static objects
/// in config.h
typedef struct _Distro {
    const char **install;
    const char **update;
    const char **upgrade;
    const char **suffix;
    const char **basepkg;
} Distro;

#endif // DISTRO_H
