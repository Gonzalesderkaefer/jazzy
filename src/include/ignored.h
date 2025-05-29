#ifndef JAZZIGNORED_H
#define JAZZIGNORED_H

/* custom types */
#include "def.h"


/**
 * This struct represents a file/directory
 * that is in $HOME/Jazzian/configs and should
 * be ignored so that it can be moved to a non-standard
 * location (i.e not in ~/Jazzian/configs)
 */
typedef struct _ignored Ignored;


/**
 * This function initializes an Ignored struct 
 * 
 * @param name identifier
 * @param src the source of the directory
 * @param dest the destination of the directory
 *
 * @return pointer to an Ignored struct that has 
 *         to be cleaned up with ignored_free()
 *         returns NULL on failure
 */
Ignored *ignored_init(const char *name, const char *src, const char *dest);


/**
 * This function applies the Ignored file operation
 *
 * @param to_apply Ignored struct to whose file operation are
 *        to be applied
 * @param method can be LINK COPY or NOTHING
 */
void ignored_apply(Ignored *to_apply, TRANSFER method);


/**
 * This function cleans up a Ignored struct 
 *
 * @param to_delete Ignored to free
 */
void ignored_free(Ignored *to_delete);
#endif /* JAZZIGNORED_H */
