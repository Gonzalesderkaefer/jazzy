// The config
#include "config.h"
#include "utils/menu/menu.h"

// Libraries
#include <assert.h>

extern char *home_dir;


int main(void) {
    // Get home
    char *home_dir = getenv("HOME");
    assert(home_dir);


    char ch = menu_w_default(
            "Choose Editor",
            "Your Choice: ",
            (const char *[]){"Vim", "Emacs", "Kate", NULL},
            0
    );
}
