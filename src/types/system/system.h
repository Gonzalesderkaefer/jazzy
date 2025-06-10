#ifndef SYSTEM_H
#define SYSTEM_H


typedef struct _System  System;


const System *get_system();
void update(System *system);
char *home(System *sys);

#endif // SYSTEM_H
