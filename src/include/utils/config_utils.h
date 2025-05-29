#ifndef J_CONFIG_UTILS_H
#define J_CONFIG_UTILS_H
#include "../def.h"


char *wm_tostr(WINDOWMANAGER wm);
char *dsp_tostr(DISPLAYSERVER dsp);
char *transfer_tostr(TRANSFER transfer);
char *distro_tostr(DISTRO distro);



char *config_tostr(config *system);
#endif /* J_CONFIG_UTILS_H */
