// The config
#include "config.h"
#include "types/system/system.h"
#include "types/display/windowmanager.h"
#include "utils/menu/menu.h"

// Libraries
#include <assert.h>
#include <stdlib.h>

int main(void) {
    // Get home
    char *home_dir = getenv("HOME");
    assert(home_dir);

    const System *sys = get_system();
    update((System *)sys);



}
