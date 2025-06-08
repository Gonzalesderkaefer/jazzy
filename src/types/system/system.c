


#include "../distro/distro.h"
#include "../display/dspserver.h"
#include "../display/windowmanager.h"



typedef struct _System {
    const DistroIndex distro_index;
    const Distro distro;
    const WindowManager wm;
    const DspServer dsp;
} System;
