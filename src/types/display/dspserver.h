#ifndef DSP_SERVER_H
#define DSP_SERVER_H


typedef struct _DspServer {
    const char ***packages;

} DspServer;

typedef enum _DspIndex {
    XORG = 0,
    WAYLAND,
    TTY
} DspIndex;


DspIndex get_dsp();

#endif // DSP_SERVER_H
