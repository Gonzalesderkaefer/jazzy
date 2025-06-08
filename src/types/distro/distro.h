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


typedef enum _DistroIndex {
    DEBIAN = 0,
    FEDORA = 1,
    ARCH = 2
} DistroIndex;

#endif // DISTRO_H
