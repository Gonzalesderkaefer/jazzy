#include "menu.h"
#include "format.h"
#include <stdint.h>
#include <stdio.h>

int flush_CHAR;
#define flush_stdin() while ((flush_CHAR = getchar()) != '\n' && flush_CHAR != EOF)

char menu_regular(const char *heading, const char *prompt, const char *options[]) {
    // Print the heading
    printf("%s%s%s%s\n", FgPurple, Underline, heading, Reset);

    // Print the options
    for (uint32_t i = 0; options[i] != NULL; ++i) {
        puts(options[i]);
    }

    // Print prompt
    printf("%s", prompt);

    char choice = getchar();

    if (choice == '\n') {
        return choice;
    } else {
        flush_stdin();
    }

    return choice;
}



char menu_w_default(
        const char *heading,
        const char *prompt,
        const char *options[],
        uint32_t def_index) {
    // Print the heading
    printf("%s%s%s%s\n", FgPurple, Underline, heading, Reset);

    // Print the options
    for (uint32_t i = 0; options[i] != NULL; ++i) {
        if (i == def_index) {
            printf("%s%s%s\n", FgGreen, options[i], Reset);
        } else {
            puts(options[i]);
        }
    }

    // Print prompt
    printf("%s", prompt);

    char choice = getchar();

    if (choice == '\n') {
        return choice;
    } else {
        flush_stdin();
    }

    return choice;
}
