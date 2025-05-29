#ifndef DEF_H
#define DEF_H

/* Libraries */
#include <dirent.h>
#include <errno.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <unistd.h>
#include <regex.h>
#include <sys/types.h>

/* For flushing stdin */
extern int flush_CHAR;
#define flush_stdin() while ((flush_CHAR = getchar()) != '\n' && flush_CHAR != EOF) ;

/* Format colors */
#define F_BLACK "\e[0;30m"
#define F_RED "\e[0;31m"
#define F_GREEN "\e[0;32m"
#define F_YELLOW "\e[0;33m"
#define F_BLUE "\e[0;34m"
#define F_PURPLE "\e[0;35m"
#define F_CYAN "\e[0;36m"
#define F_WHITE "\e[0;37m"
#define F_END "\e[0m"

/* Display server */
typedef enum DISPLAYSERVER { XORG = 0, WAYLAND } DISPLAYSERVER;

/* Windowmanager */
typedef enum WINDOWMANAGER {
  I3 = 0,
  AWESOME,
  BSPWM,
  SWAY,
  HYPRLAND,
  RIVER
} WINDOWMANAGER;

/* Distro */
typedef enum DISTRO { DEBIAN = 0, FEDORA, ARCH, UNKNOWN } DISTRO;

/* Type of transfer */
typedef enum TRANSFER { NOTHING = 0, LINK, COPY } TRANSFER;

/* Config */
typedef struct setup_config {
  DISPLAYSERVER display_server;
  WINDOWMANAGER window_manager;
  DISTRO distro;
  TRANSFER file_transfer;
} config;
#endif
