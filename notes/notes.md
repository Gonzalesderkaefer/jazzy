# Notes



## Procedure
  * Determine Distro
  * Present choices based on distro
  * Read choices
  * Create model from choices
  * 'apply' model

## Modelling (These are predefined)

### Machine
  * Has a home directory
  * Has a [Distro](#distro) as a reference
  * Has a setup function

### Distro
  * Has supported [display servers](#display-server) as references
  * Has supported [window managers](#window-manager) as references
  * Base packages (should work with 'TTY')
  * Has a setup function

### Display Server
  * Has packages
  * Has [window managers](#window-manager) as references
  * Has a setup function

### Window Manager
  * Has packages
  * Has a setup function



## File utils

### Copy dir behavior
  * if source does not exist, return
  * if target dir does not exist, create it and copy contents from inside src
    into newly created target.
  * if target dir does exist copy the entire srcdir as a childdir of target
