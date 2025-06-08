#ifndef MENU_H
#define MENU_H

// Other Libraries
#include <stdint.h>



/// This function prints a regular menu to the screen:
/// `heading` will be purple and underlined
/// `prompt` will be the text left to the cursor
/// `options` This has to be a NULL terminated list of options the user can choose
char menu_regular(const char *heading, const char *prompt, const char *options[]);

/// This function prints a menu to the screen, which has a default entry, that
/// is colored green:
/// - `def_index` This is the index of the default value
char menu_w_default(const char *heading, const char *prompt, const char *options[], uint32_t def_index);



#endif // MENU_H
