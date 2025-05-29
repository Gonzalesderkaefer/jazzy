#ifndef J_CONFIG_H
#define J_CONFIG_H

/* Other files */
#include "def.h"


enum DISPLAYSERVER get_display_server();
enum WINDOWMANAGER get_window_manager(enum DISPLAYSERVER);
enum DISTRO get_distro();
enum TRANSFER get_transfer();
config *get_config();

#endif /* J_CONFIG_H */
