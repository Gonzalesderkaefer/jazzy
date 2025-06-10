// Header file
#include "distro.h"

// Libraries
#include <stdio.h>
#include <regex.h>

#include "../../config.h"

const DistroIndex get_distro() {
    FILE *file = fopen("/etc/os-release", "r");
    if (!file) {
        fprintf(stderr, "Could not open release file");
        return NONE;
    }
    // Determine size of file
    fseek(file, 0, SEEK_END); // Move to end
    int release_length = ftell(file); // Get position
    // Set file pointer to the beginning
    fseek(file, 0, SEEK_SET);

    // Read the file into buffer
    char release[release_length + 1];
    char curr;
    for (int i = 0; (curr = fgetc(file)) != EOF; ++i) {
        release[i] = curr;
    }

    fclose(file);

    // Constructing regexes
    regex_t arch_regex;
    regmatch_t arch_pmatch[5];
    regex_t debian_regex;
    regmatch_t debian_pmatch[5];
    regex_t fedora_regex;
    regmatch_t fedora_pmatch[5];

    // Compile regexes
    regcomp(&arch_regex, "ARCH LINUX|Arch Linux|arch linux", REG_EXTENDED);
    regcomp(&debian_regex, "DEBIAN|Debian|debian", REG_EXTENDED);
    regcomp(&fedora_regex, "FEDORA|Fedora|fedora", REG_EXTENDED);

    // Run regexes
    int archStat =
            regexec(&arch_regex, release, arch_regex.re_nsub + 1, arch_pmatch, REG_NOTEOL);
    int debStat =
            regexec(&debian_regex, release, debian_regex.re_nsub + 1, debian_pmatch, REG_NOTEOL);
    int fedStat =
            regexec(&fedora_regex, release, fedora_regex.re_nsub + 1, fedora_pmatch, REG_NOTEOL);

    // Free regexes
    regfree(&arch_regex);
    regfree(&debian_regex);
    regfree(&fedora_regex);

    if (archStat == 0) {
        printf("\033[0;32mFound Arch Linux\033[0m\n");
        return ARCH;
    }
    if (debStat == 0) {
        printf("\033[0;32mFound Debian\033[0m\n");
        return DEBIAN;
    }
    if (fedStat == 0) {
        printf("\033[0;32mFound Fedora\033[0m\n");
        return FEDORA;
    }
    return NONE;
}
