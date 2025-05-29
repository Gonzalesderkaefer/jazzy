#ifndef JAZZCUSTOMIZED_H
#define JAZZCUSTOMIZED_H
#include <sys/types.h>

/**
 * This function creates a file if it does not exist
 * This should usually be used for customized files
 *
 * @param relpath a file path relative to $HOME
 * @param contents of that file
 * @param mode permissions of that file
 */
void customized(char *relpath, char *contents, mode_t mode, bool customperm);

#endif /* JAZZCUSTOMIZED_H */
