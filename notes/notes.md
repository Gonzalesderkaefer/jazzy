# Notes



## Procedure
  * Determine Distro
  * Present choices based on distro
  * Read choices
  * Create model from choices
  * 'apply' model

## Modelling (Instances of these are predefined in the config module)

### Machine
  * Has a home directory
  * Has a [Distro](#distro) as a reference
  * Has a setup function

### Distro
  * Has an id
  * Has supported [display servers](#display-server) as references
  * Has supported [window managers](#window-manager) as references
  * Has an install command
  * Has an update command
  * Has an upgrade command (Optional)
  * Has Base packages (should work with 'TTY')
  * Has a setup function

### Display Server
  * Has an id
  * Has packages
  * Has [window managers](#window-manager) as references
  * Has a setup function

### Window Manager
  * Has an id
  * Has packages
  * Has a setup function
  * Implements `PartialEq`

All the types above execept [Machine](#machine) has its own enum that contains the types themselves

## File utils

### Copy dir behavior
  * if source does not exist, return
  * if target dir does not exist, create it and copy contents from inside src
    into newly created target.
  * if target dir does exist copy the entire srcdir as a childdir of target



## Moving files
There will be two ways to move a file:
  * for each item in `src/` move them to `dest/`
    where `src/` and `dest/` are directories
  * move one item `src` to `dest` 



