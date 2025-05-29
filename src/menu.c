#include "include/def.h"
#include "include/utils/mymenu.h"

char print_menu(const char* header, const char **options, const char *prompt) { /* NOTE: options has to be 'NULL terminated' */
  printf("%s%s%s\n", F_PURPLE, header, F_END);
  for (int i = 0; options[i] != NULL; ++i) {
    printf("%s%s%s\n", F_GREEN, options[i], F_END);
  }
  printf("%s: ", prompt);
  char choice = getchar();

  return choice;
}

