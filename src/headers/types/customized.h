#ifndef CUSTOMIZED_H
#define CUSTOMIZED_H
// Libraries
#include <sys/types.h>


/// Customized
/// This struct exists, so that the customized 
/// files can be easily declared in config.h
/// There's no other use for this (so far)
typedef struct _Customized {
    const char *path;
    const char *content;
    const mode_t permission;

} Customized;


/// This function creates a customized
/// file where `path` is the absolute filepath
/// `content` is what the new file will contain
/// and `perm` is the mode of the file e.g 0644
/// The user running this program must have access to write
/// to the file path.
///
/// These files are usually ignored by
/// the VCS but are still in those repos
/// if the user decides to e.g symlink the 
/// config directories from the repo.
void create_customized(const char *path, const char *content, const mode_t perm);


#endif // CUSTOMIZED_H
