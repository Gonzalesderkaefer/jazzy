/* Other files */
#include "include/pacinst.h"
#include "include/def.h"
#include "include/utils/pkgutils.h"
#include "include/packages.h"
#include <stdio.h>

static int install_packages(const char **base, const char **dsp,
                            const char **wm, const char **suffix) {
  /* Determine lengths */
  int baselen, dsplen, wmlen, suflen;
  lst_len(baselen, base);
  lst_len(dsplen, dsp);
  lst_len(wmlen, wm);

  /* Check if suffix exists */
  if (suffix == NULL)
    suflen = 0;
  else
    lst_len(suflen, suffix);

  /* init array for full command */
  const char *full_cmd[baselen + dsplen + wmlen + suflen + 1];
  /* override with NULL */
  for (int j = 0; j < baselen + dsplen + wmlen + suflen + 1; ++j)
    full_cmd[j] = NULL;

  int glob_i = 0;
  int i = 0;

  /* Copy package names */
  for (; base[i] != NULL; ++i)
    full_cmd[glob_i++] = base[i];
  i = 0;

  for (; dsp[i] != NULL; ++i)
    full_cmd[glob_i++] = dsp[i];
  i = 0;

  for (; wm[i] != NULL; ++i)
    full_cmd[glob_i++] = wm[i];
  i = 0;

  if (suffix != NULL)
    for (; suffix[i] != NULL && suffix != NULL; ++i)
      full_cmd[glob_i++] = suffix[i];

  i = 0;


  pid_t pid = fork();

  switch (pid) {
    case -1:
      return -1;
    case 0:
      execvp("sudo",(char **)full_cmd);
      break;
    default:
      wait(NULL);
      break;

  }
  return 0;
}

int inst_pac(config *system) {
  /* array names */
  const char **base;
  const char **dsp_server;
  const char **wm;
  const char **suffix = NULL;

  /* NOTE: Might  have to make this cleaner don't know how */

  /* Getting packages */
  switch (system->distro) {
  /* Filtering distro */
  case DEBIAN:
    base = debian_base;
    /* Display server */
    switch (system->display_server) {
    case WAYLAND:
      dsp_server = debian_wayland;
      /* Window manager */
      switch (system->window_manager) {
      case RIVER:
        wm = debian_river;
        break;
      case HYPRLAND:
        wm = debian_hypr;
        break;
      default:
        wm = debian_sway;
        break;
      }
      break;
    default:
      dsp_server = debian_xorg;
      /* Window manager */
      switch (system->window_manager) {
      case AWESOME:
        wm = debian_awesome;
        break;
      case BSPWM:
        wm = debian_bspwm;
        break;
      default:
        wm = debian_i3;
        break;
      }
      break;
    }
    break;
  case FEDORA:
    base = fedora_base;
    /* Display server */
    switch (system->display_server) {
    case WAYLAND:
      dsp_server = fedora_wayland;
      /* Window manager */
      switch (system->window_manager) {
      case RIVER:
        wm = fedora_river;
        break;
      case HYPRLAND:
        wm = fedora_hypr;
        break;
      default:
        wm = fedora_sway;
        break;
      }
      break;
    default:
      dsp_server = fedora_xorg;
      /* Window manager */
      switch (system->window_manager) {
      case AWESOME:
        wm = fedora_awesome;
        break;
      case BSPWM:
        wm = fedora_bspwm;
        break;
      default:
        wm = fedora_i3;
        break;
      }
      break;
    }
    break;
  case ARCH:
    base = arch_base;
    /* Display server */
    switch (system->display_server) {
    case WAYLAND:
      dsp_server = arch_wayland;
      /* Window manager */
      switch (system->window_manager) {
      case RIVER:
        wm = arch_river;
        break;
      case HYPRLAND:
        wm = arch_hypr;
        break;
      default:
        wm = arch_sway;
        break;
      }
      break;
    default:
      dsp_server = arch_xorg;
      /* Window manager */
      switch (system->window_manager) {
      case AWESOME:
        wm = arch_awesome;
        break;
      case BSPWM:
        wm = arch_bspwm;
        break;
      default:
        wm = arch_i3;
        break;
      }
      break;
    }
    suffix = arch_suffix;
    break;

  default:
    return -1;
    break;
  }

  install_packages(base, dsp_server, wm, suffix);
  return 0;
}

int update(config *system) {
  const char **upgrade_cmd;
  pid_t debPid;

  switch (system->distro) {
  case ARCH:
      upgrade_cmd = arch_update;
    break;
  case FEDORA:
      upgrade_cmd = fedora_update;
    break;
  case DEBIAN:
      upgrade_cmd = debian_upgrade;
      debPid = fork();
      /* Updating */
      switch (debPid) {
        case 0:
          execvp("sudo",(char **)debian_update);
          break;
        case -1:
          fprintf(stderr, "fork failed\n");
          return -1;
        default:
          wait(NULL);
          break;
      }
    break;
  default:
    return -1;
  }

  pid_t pid = fork();
  switch (pid) {
  case 0:
    execvp("sudo", (char **)upgrade_cmd);
    break;
  case -1:
    fprintf(stderr, "fork failed\n");
    return -1;
    break;
  default:
    wait(NULL);
    break;
  }
  return 0;
}
