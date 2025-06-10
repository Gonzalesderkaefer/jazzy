// Header file
#include "transfer.h"

// Libraries
#include "../../utils/menu/menu.h"
#include <stdlib.h>

Transfer get_transfer() {
    char wm_choice = menu_w_default(
            "Choose a Windowmanager",
            "Your choice: ",
            (const char *[]){"[N]one", "[C]opy", "[L]ink", NULL},
            0
            );

    switch (wm_choice) {
        case 'c':
            return COPY;
            break;
        case 'C':
            return COPY;
            break;
        case 'L':
            return COPY;
            break;
        case 'l':
            return COPY;
            break;
        default:
            return LEAVE;
            break;
    }

}
