// Header file
#include "system.h"



// Libraries
#include "../distro/distro.h"
#include "../display/dspserver.h"
#include "../display/windowmanager.h"
#include "../transfer/transfer.h"
#include "../../config.h"
#include <stdlib.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>






typedef struct _System {
    const char *home;
    Transfer tr;
    DistroIndex distro_index;
    const Distro *distro;
    const DspServer *dsp;
    DspIndex dsp_index;
    const WindowManager *wm;
} System;


const System *get_system() {
    // This should as long as the program
    static System sys;

    // Get Distro enum
    sys.distro_index = get_distro();

    // Get actual struct
    switch (sys.distro_index) {
    case DEBIAN:
        sys.distro = &debian;
        break;
    case FEDORA:
        sys.distro = &fedora;
        break;
    case ARCH:
        sys.distro = &arch;
        break;
    case NONE:
        sys.distro = NULL;
        break;
      break;
    }

    // Get Dsp Enum
    sys.dsp_index = get_dsp();

    // Get actual struct
    switch (sys.dsp_index) {
    case XORG:
        sys.dsp = &xorg;
    case WAYLAND:
        sys.dsp = &wayland;
    case TTY:
        sys.dsp = &tty;
      break;
    }
    // WM
    sys.wm = get_wm(sys.dsp_index);

    // Assign home
    sys.home = getenv("HOME");

    // Transfer
    sys.tr = get_transfer();
    return &sys;
}


void update(System *system) {
    const char **upgrade = system->distro->upgrade;
    const char **update = system->distro->update;

    if (update) {
        pid_t pid = fork();

        switch (pid) {
            case -1:
                return;
                break;
            case 0:
                execvp("sudo", (char **) update);
                break;
            default:
                wait(NULL);
                break;
        }
    }

    if (upgrade) {
        pid_t pid = fork();

        switch (pid) {
            case -1:
                return;
                break;
            case 0:
                execvp("sudo", (char **) upgrade);
                break;
            default:
                wait(NULL);
                break;
        }
    }
}








char *home(System *sys) {
    return (char *) sys->home;
}
