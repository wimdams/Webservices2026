#include "filesystem.h"
#include <stddef.h>
#include <stdlib.h>
#include <string.h>

#define FS_ENTRY_SIZE 40
#define FS_NAME_LEN   32

static struct file* files = NULL;
static uint8_t file_amount = 0;

uint8_t fs_init(const uint8_t* fs_base, size_t fs_total_size) {
    if (!fs_base || fs_total_size < 1) {
        return 0;
    }

    uint8_t file_count = fs_base[0];
    size_t header_end = 1 + ((size_t)file_count * FS_ENTRY_SIZE);

    if (fs_total_size < header_end) {
        return 0;
    }

    free(files);
    files = malloc(file_count * sizeof(struct file));
    if (!files) {
        file_amount = 0;
        return 0;
    }

    for (uint8_t i = 0; i < file_count; i++) {
        const uint8_t* entry = fs_base + 1 + (i * FS_ENTRY_SIZE);
        uint32_t offset = 0;
        uint32_t size = 0;

        if (memchr(entry, '\0', FS_NAME_LEN) == NULL) {
            free(files);
            files = NULL;
            file_amount = 0;
            return 0;
        }

        memcpy(&size, entry + FS_NAME_LEN, sizeof(uint32_t));
        memcpy(&offset, entry + FS_NAME_LEN + sizeof(uint32_t), sizeof(uint32_t));

        if ((size_t)offset + (size_t)size > fs_total_size) {
            free(files);
            files = NULL;
            file_amount = 0;
            return 0;
        }

        files[i].file_name = (char*)entry;
        files[i].file_size = size;
        files[i].file_startaddr = (void*)(fs_base + offset);
    }

    file_amount = file_count;
    return file_count;
}

file_ID* fs_search_files(const char* filename) {
    //tbi
    return NULL;
}

const char* fs_get_filename(file_ID file) {
    return file ? file->file_name : NULL;
}

uint32_t fs_get_filesize(file_ID file) {
    return file ? file->file_size : 0;
}

void* fs_get_file_addr(file_ID file) {
    return file ? file->file_startaddr : NULL;
}
