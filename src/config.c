/* Other files */
#include "include/config.h"

/* Flush char */
int flush_CHAR;

enum DISPLAYSERVER get_display_server() {
  /* Ask user */
  printf("\033[1;35mChoose a Displayserver\033[0m\n");
  printf("\033[0;32m[x]org (default)\033[0m\n");
  printf("\033[0;32m[w]ayland \033[0m\n");
  printf("Your Choice: ");
  /* Get users choice */
  char choice = getchar();

  if (choice == 'w' || choice == 'W') {
    return WAYLAND;
  }

  return XORG;
}




enum WINDOWMANAGER get_window_manager(enum DISPLAYSERVER display_server) {
  /* Ask user */
  printf("\033[1;35mChoose a Windowmanager\033[0m\n");
  char choice;
  switch (display_server) {
  case XORG:
    printf("\033[0;32m[i]3 (default)\033[0m\n");
    printf("\033[0;32m[a]wesome \033[0m\n");
    printf("\033[0;32m[b]spwm \033[0m\n");
    printf("Your Choice: ");

    /* Get users choice */
    choice = getchar();

    /* Check user choice and return */
    if (choice == 'b' || choice == 'B') {
      return BSPWM;
    } else if (choice == 'a' || choice == 'A') {
      return AWESOME;
    } else {
      return I3;
    }

    break;
  default:
    printf("\033[0;32m[s]way (default)\033[0m\n");
    printf("\033[0;32m[h]yprland \033[0m\n");
    printf("\033[0;32m[r]iver \033[0m\n");
    printf("Your Choice: ");

    /* Get users choice */
    choice = getchar();

    /* Check user choice and return */
    if (choice == 'h' || choice == 'H') {
      return HYPRLAND;
    } else if (choice == 'r' || choice == 'R') {
      return RIVER;
    } else {
      return SWAY;
    }

    break;
  }
}




enum DISTRO get_distro() {
  FILE *file = fopen("/etc/os-release", "r");
  if (!file) {
    fprintf(stderr, "Could not open release file");
    return UNKNOWN;
  }
  /* Determine size of file */
  fseek(file, 0, SEEK_END);
  int length = ftell(file);
  /* Set file pointer to the beginning */
  fseek(file, 0, SEEK_SET);

  /* Read the file */
  char release[length + 1];
  int i = 0;
  char curr;
  while ((curr = fgetc(file)) != EOF) {
    release[i] = curr;
    ++i;
  }

  fclose(file);

  /* Constructing regexes */
  regex_t arch;
  regmatch_t arch_pmatch[5];
  regex_t debian;
  regmatch_t debian_pmatch[5];
  regex_t fedora;
  regmatch_t fedora_pmatch[5];

  regcomp(&arch, "ARCH LINUX|Arch Linux|arch linux", REG_EXTENDED);
  regcomp(&debian, "DEBIAN|Debian|debian", REG_EXTENDED);
  regcomp(&fedora, "FEDORA|Fedora|fedora", REG_EXTENDED);

  int archStat =
      regexec(&arch, release, arch.re_nsub + 1, arch_pmatch, REG_NOTEOL);
  int debStat =
      regexec(&debian, release, debian.re_nsub + 1, debian_pmatch, REG_NOTEOL);
  int fedStat =
      regexec(&fedora, release, fedora.re_nsub + 1, fedora_pmatch, REG_NOTEOL);

  regfree(&arch);
  regfree(&debian);
  regfree(&fedora);

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
  return UNKNOWN;
}


enum TRANSFER get_transfer() {
  /* Ask user */
  printf("\033[1;35mChoose method of transfer\033[0m\n");
  printf("\033[0;32mDo [N]othing (default)\033[0m\n");
  printf("\033[0;32m[l]ink \033[0m\n");
  printf("\033[0;32m[c]opy \033[0m\n");
  printf("Your Choice: ");
  /* Get users choice */
  char choice = getchar();
  /* Check user choice and return */
  if (choice == 'l' || choice == 'L') {
    return LINK;
  } else if (choice == 'c' || choice == 'C') {
    return COPY;
  } else {
    return NOTHING;
  }
}





config *get_config() {
  /* Config struct */
  static config this_config;
  /* get display manager from user */
  this_config.display_server = get_display_server();
  /* getchar won't work otherwise */
  flush_stdin();
  /* get window manager */
  this_config.window_manager = get_window_manager(this_config.display_server);
  /* getchar won't work otherwise */
  flush_stdin();
  /* Get Distro */
  this_config.distro = get_distro();
  /* Get Transfer type */
  this_config.file_transfer = get_transfer();
  flush_stdin();
  return &this_config;
}


