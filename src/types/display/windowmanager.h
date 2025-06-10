#ifndef WINDOW_MANAGER_H
#define WINDOW_MANAGER_H

#include "dspserver.h"

typedef struct _WindowManager {
    const char ***packages;
} WindowManager;


const WindowManager *get_wm(DspIndex index);

#endif // WINDOW_MANAGER_H
