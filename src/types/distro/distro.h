#ifndef DISTRO_H
#define DISTRO_H

/// These will be predefined static objects
/// in config.h
typedef struct _Distro {
    static const char **install;
    static const char **update;
    static const char **upgrade;
    static const char **suffix;
    static const char **basepkg;
} Distro;

#endif // DISTRO_H
