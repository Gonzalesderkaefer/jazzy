#ifndef DSP_SERVER_H
#define DSP_SERVER_H


typedef struct _DspServer {
    const char **debpackages;
    const char **fedpackages;
    const char **archpackages;
} DspServer;

#endif // DSP_SERVER_H
