#include "include/utils/config_utils.h"

char *wm_tostr(WINDOWMANAGER wm) {
  switch (wm) {
  case I3:
    return "i3";
    break;
  case AWESOME:
    return "awesome";
    break;
  case BSPWM:
    return "bspwm";
    break;
  case SWAY:
    return "sway";
    break;
  case HYPRLAND:
    return "Hyprland";
    break;
  case RIVER:
    return "river";
    break;
  default:
    return "i3";
  }
}

char *dsp_tostr(DISPLAYSERVER dsp) {
  switch (dsp) {
  case XORG:
    return "xorg";
  case WAYLAND:
    return "wayland";
  default:
    return "xorg";
  }
}

char *transfer_tostr(TRANSFER transfer) {
  switch(transfer) {
    case LINK:
      return "link";
      break;
    case COPY:
      return "copy";
      break;
    default:
      return "nothing";
      break;
  }
}

char *distro_tostr(DISTRO distro) {
  switch (distro) {
    case FEDORA:
      return "fedora";
      break;
    case ARCH:
      return "arch";
      break;
    case DEBIAN:
      return "debian";
      break;
    default:
      return "unknown";
      break;
  }
}



[[nodiscard]]
char *config_tostr(config *system) {
  /* Store the values */
  char *distro = distro_tostr(system->distro);
  char *wm = wm_tostr(system->window_manager);
  char *transfer = transfer_tostr(system->file_transfer);
  char *dsp = dsp_tostr(system->display_server);

  const char *format =
    "{\n"
    "    \"Displaymanager\": \"%s\",\n"
    "    \"Windowmanager\": \"%s\",\n"
    "    \"Transfer\": \"%s\",\n"
    "    \"Distro\": \"%s\",\n"
    "}";

  size_t systemlen = strlen(distro) + strlen(wm) + strlen(transfer) + strlen(dsp) + strlen(format);
  char *system_str = malloc(systemlen + 1);
  snprintf (system_str, systemlen + 1,
    "{\n"
    "    \"Displaymanager\": \"%s\",\n"
    "    \"Windowmanager\": \"%s\",\n"
    "    \"Transfer\": \"%s\",\n"
    "    \"Distro\": \"%s\",\n"
    "}",
    dsp, wm, transfer, distro
  );


  return system_str;
}






