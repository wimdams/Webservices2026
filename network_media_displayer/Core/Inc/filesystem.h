#include <stdint.h>
#include <stddef.h>

typedef struct file { 
    char* file_name;        // file name including extention
    uint32_t file_size;     // file size in bytes
    void* file_startaddr;   // pointer to where the file starts
} file;

typedef file* file_ID;

uint8_t fs_init(const uint8_t* fs_base, size_t fs_total_size);
file_ID* fs_search_files(const char* filename);
const char* fs_get_filename(file_ID file);
uint32_t fs_get_filesize(file_ID file);
void* fs_get_file_addr(file_ID file);
