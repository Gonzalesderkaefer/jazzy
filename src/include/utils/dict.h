#ifndef J_DICT_H
#define J_DICT_H
/* Other headers */
#include <stddef.h>
#include <stdbool.h>

/**
 * This struct represents a
 * dictionary which is a type of 
 * hashtable where key value pairs
 * are (char *, void *).
 */
typedef struct _Dict Dict;



/**
 * Initialize a dictionary
 *
 * @return pointer to a dictionary
 */
Dict *dict_init();

/**
 * Insert a <key, value> pair into dictionary
 *
 * @param dictionary pointer to Dict struct
 * @param key the string-key associated with the value This should be a string literal
 * @param pointer to the value
 *
 * @return true if there were no colliions, false if there were
 */
bool dict_insert(Dict *dictionary, const char *key, void *value);

/**
 * Get pointer to a value associated with a key
 *
 * @param dictionary pointer to Dict struct
 * @param key key to look for
 *
 * @return pointer to the value if value exists, NULL else
 */
void *dict_get(Dict *dictionary, const char *key);


/**
 * deletes the dictionary
 */
void dict_free(Dict *dictionary);

/**
 * Perform an action on all values in the dictionary
 *
 * @param dictionary pointer to Dict struct
 * @param action a function that gets a (void *) as a parameter
 */
void dict_action(Dict *dictionary, void (*action) (void *));

#endif /* J_DICT_H */
