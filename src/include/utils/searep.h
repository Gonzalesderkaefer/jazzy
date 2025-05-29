#include "../def.h"


#ifndef SEAREP_C
#define SEAREP_C


[[nodiscard("Heap allocated return value must be free'd")]]
char *rep_interval(char *freeable, int start, int end, char *insertion);

[[nodiscard("Heap allocated return value must be free'd")]]
char *search_replace(char *freeable, char *regexp, char *substitute);


#endif /* SEAREP_C */
