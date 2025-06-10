// Header file
#include "dspserver.h"

// Libraries
#include "../../utils/menu/menu.h"
#include <stdlib.h>


DspIndex get_dsp() {
    char dsp_choice = menu_w_default(
            "Choose Displayserver",
            "Your choice: ",
            (const char *[]){"[X]org", "[W]ayland", "[T]ty", NULL},
            0
    );


    switch (dsp_choice) {
        case 'w':
            return WAYLAND;
        case 'W':
            return WAYLAND;
        case 't':
            return TTY;
        case 'T':
            return TTY;
        default:
            return XORG;
    }
}
