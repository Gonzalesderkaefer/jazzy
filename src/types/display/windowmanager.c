// Header file
#include "windowmanager.h"

// Libraries
#include "dspserver.h"
#include "../../utils/menu/menu.h"
#include "../../config.h"


const WindowManager *get_wm(DspIndex index) {
    char wm_choice;
    switch (index) {
    case XORG:
             wm_choice = menu_w_default(
                    "Choose a Windowmanager",
                    "Your choice: ",
                    (const char *[]){"[I]3", "[A]wesome", "[B]spwm", NULL},
                    0
            );
             switch (wm_choice) {
                 case 'a':
                     return &awesome;
                     break;
                 case 'A':
                     return &awesome;
                     break;
                 case 'b':
                     return &bspwm;
                     break;
                 case 'B':
                     return &bspwm;
                     break;
                 default:
                     return &i3;
                     break;
             }
        break;
    case WAYLAND:
             wm_choice = menu_w_default(
                    "Choose a Compositor",
                    "Your choice: ",
                    (const char *[]){"[S]way", "[H]yprland", "[R]iver", NULL},
                    0
            );
             switch (wm_choice) {
                 case 'h':
                     return &hypr;
                     break;
                 case 'H':
                     return &hypr;
                     break;
                 case 'r':
                     return &river;
                     break;
                 case 'R':
                     return &river;
                     break;
                 default:
                     return &sway;
                     break;
             }
        break;
    case TTY:
        return NULL;
      break;
    }
}
