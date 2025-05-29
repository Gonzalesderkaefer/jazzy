#include "include/utils/searep.h"


[[nodiscard]]
char *rep_interval(char *freeable, int start, int end, char *insertion) {
    /* Bounds checking */
    if (start > end || end > strlen(freeable)) {
        fprintf(stderr, "Index error\n");
        return NULL;
    }

    /* Determine new size */
    int old_len = strlen(freeable);
    int new_len = strlen(insertion) + (old_len - (end - start));

    /* New Buffer  */
    char *newbuf;
    if (!(newbuf = (char *)calloc (sizeof(char), new_len + 1))) {
        fprintf(stderr, "Memory Allocation error\n");
        return NULL;
    }

    /* Copy first half */
    int nb_index = 0;
    int ob_index = 0;

    for (; nb_index < start; ++nb_index) {
        newbuf[nb_index] = freeable[ob_index++];
    }
    /* concat insertion */
    newbuf[nb_index] = '\0';
    strcat(newbuf, insertion);

    /* copy after end */
    nb_index += strlen(insertion);
    ob_index = end;
    for (;nb_index < new_len && ob_index < old_len; ++nb_index){
        newbuf[nb_index] = freeable[ob_index++];
    }

    return newbuf;
}

/* TODO: Warn user to not ignore return value */
[[nodiscard]]
char *search_replace(char *freeable, char *regexp, char *substitute) {
    /* Build regex */
    regex_t regex;
    regmatch_t pmatch[1];
    /* Compile regex */
    if (regcomp(&regex, regexp, REG_EXTENDED)){
        fprintf(stderr, "Error compiling regex\n");
        return freeable;
    }
    /* Check if there's a match */
    if (REG_NOMATCH == regexec(&regex, freeable, regex.re_nsub + 1, pmatch, REG_NOTEOL)) {
        return freeable;
    }

    /* Get interval */
    int start = pmatch[0].rm_so;
    int end = pmatch[0].rm_eo;

    /* Replace interval */
    char *edited = rep_interval(freeable, start, end, substitute);
    regfree(&regex);

    return edited;
}
