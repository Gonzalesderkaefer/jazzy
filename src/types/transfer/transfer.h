#ifndef TRANSFER_H
#define TRANSFER_H

typedef enum _Transfer {
    COPY,
    LINK,
    LEAVE,
} Transfer;

Transfer get_transfer();

#endif // TRANSFER_H
