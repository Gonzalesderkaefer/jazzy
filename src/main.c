/* Other files */
#include "include/def.h"
#include "include/backup.h"
#include "include/config.h"
#include "include/custom.h"
#include "include/move.h"
#include "include/pacinst.h"
#include "include/utils/config_utils.h"
#include "include/utils/mymenu.h"

void init_system() {
  /* Get Config from user */
  config *system = get_config();

  /* Update system */
  update(system);

  /* Install required packages */
  inst_pac(system);

  /* Transfer config files */
  move_cfg(system->file_transfer);

  /* Create custom config files */
  create_customized(system);
}

int main() {
  const char *header = "The jazzy utility";
  const char *prompt = "Your Choice";
  const char *options[] = {"create [b]ackup", "[r]estore backup",
                           "[s]etup system", NULL};

  char choice = print_menu(header, options, prompt);
  flush_stdin();
  switch (choice) {
  case 'b':
    backup_cfgs();
    break;
  case 'r':
    restore_cfgs();
    break;
  case 's':
    init_system();
    break;
  }
  return 0;
}
