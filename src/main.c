#include <stdio.h>
#include "config.h"


int main(void) {
    const char **inst = debian.install;
    for (int i = 0; inst[i] != NULL; ++i) {
        printf("%s\n", inst[i]);
    }
}
