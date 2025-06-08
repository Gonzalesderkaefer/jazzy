#ifndef FORMAT_H
#define FORMAT_H

/// These are foreground colors
#define FgBlack "\e[0;30m"
#define FgRed "\e[0;31m"
#define FgGreen "\e[0;32m"
#define FgYellow "\e[0;33m"
#define FgBlue "\e[0;34m"
#define FgPurple "\e[0;35m"
#define FgCyan "\e[0;36m"
#define FgWhite "\e[0;37m"
#define FgEnd "\x1b[39m"

/// These are for formatting
#define Bold  "\e[1m"
#define Faint  "\e[2m"
#define Italic  "\e[3m"
#define Underline  "\e[4m"
#define BlinkS  "\e[5m"
#define BlinkF  "\e[6m"
#define Conceal  "\e[8m"
#define Crossed  "\e[9m"
#define BoldOff  "\e[22m"
#define ItalicOff  "\e[23m"
#define ULineOff  "\e[24m"
#define BlinkOff  "\e[25m"
#define ConcealOff  "\e[28m"
#define CrossedOff  "\e[29m"
#define Reset   "\e[0m"






#endif // FORMAT_H
