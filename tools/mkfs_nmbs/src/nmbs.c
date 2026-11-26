// typedef struct file { 
//    char* file_name;        // file name including extention
//    uint32_t file_size;     // file size in bytes
//    void* file_startaddr;   // pointer to where the file starts
// } file;

// filesystem max 16MB
// built up of:

// uint8 : amount of files
// {
//	file file1
//	file file2
//	file file...
// }
// file1.file_startaddr{
//	binary : filecontent
// }
// file2.file_startaddr{
//	binary : filecontent
// }
// ...
// 
// uint32 : checksum (last 32bits)

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stddef.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <dirent.h>
#include <string.h>

#define MAX_FILES 255
#define NAME_LEN 32

#pragma pack(push, 1) // remove padding from struct
typedef struct {
    char name[NAME_LEN];
    uint32_t file_startaddr;
    uint32_t size;
} FileEntry;
#pragma pack(pop)

typedef struct {
    char name[NAME_LEN];
    uint8_t *data;
    uint32_t size;
} LoadedFile;

uint32_t crc32(const uint8_t *data, size_t len) {
    uint32_t crc = 0xFFFFFFFF;
    for (size_t i = 0; i < len; i++) {
        crc ^= data[i];
        for (int j = 0; j < 8; j++) {
            crc = (crc >> 1) ^ (0xEDB88320 & -(crc & 1));
        }
    }
    return ~crc;
}

int main(int argc, char *argv[]) {
    if (argc < 3) {
        printf("Usage: %s [input_dir] [output.c]\n", argv[0]);
        return 1;
    }

    DIR *dp = opendir(argv[1]);
    if (!dp) {
        printf("Couldn't open the directory\n");
        return -1;
    }

    LoadedFile files[MAX_FILES];
    uint8_t file_count = 0;
    size_t total_data_size = 0;
    struct dirent *ep;

    while ((ep = readdir(dp)) != NULL) {
        if (strcmp(ep->d_name, ".") == 0 || strcmp(ep->d_name, "..") == 0) {
            continue;
        }

        char full_path[4096];
        snprintf(full_path, sizeof(full_path), "%s/%s", argv[1], ep->d_name);

        struct stat st;
        if (stat(full_path, &st) != 0 || !S_ISREG(st.st_mode)) { // a normal file?
            continue;
        }

        if (file_count >= MAX_FILES) {
            printf("Max file limit (%d) reached what the f*ck are you doing?\n", MAX_FILES);
            break;
        }

        FILE *in = fopen(full_path, "rb");
        if (!in) {
            continue;
        }

        printf("processing: %s...\n", ep->d_name);

        memset(files[file_count].name, 0, NAME_LEN);
        strncpy(files[file_count].name, ep->d_name, NAME_LEN - 1);
        files[file_count].size = (uint32_t)st.st_size;
        files[file_count].data = malloc(files[file_count].size > 0 ? files[file_count].size : 1); // return at least 1 cuz malloc(0) nono

        if (files[file_count].size > 0) {
            fread(files[file_count].data, 1, files[file_count].size, in);
        }

        fclose(in);
        total_data_size += files[file_count].size;
        file_count++;
    }
    closedir(dp);

    size_t header_size = sizeof(uint8_t) + (file_count * sizeof(FileEntry));
    size_t payload_end = header_size + total_data_size;
    size_t total_image_size = payload_end + sizeof(uint32_t);

    uint8_t *image = calloc(1, total_image_size);
    if (!image) {
        printf("Memory allocation failedi :c\n");
        return -1;
    }

    image[0] = file_count;

    FileEntry *table = (FileEntry *)(image + sizeof(uint8_t));
    uint32_t current_addr = (uint32_t)header_size;

    for (uint8_t i = 0; i < file_count; i++) {
        memcpy(table[i].name, files[i].name, NAME_LEN);
        table[i].file_startaddr = current_addr;
        table[i].size = files[i].size;

        if (files[i].size > 0) {
            memcpy(image + current_addr, files[i].data, files[i].size);
        }

        current_addr += files[i].size;
        free(files[i].data);
    }

    uint32_t checksum = crc32(image, payload_end);
    memcpy(image + payload_end, &checksum, sizeof(uint32_t));

    FILE *out = fopen(argv[2], "w");
    if (!out) {
        printf("Couldn't open output file\n");
        free(image);
        return -1;
    }

    fprintf(out, "#include <stdint.h>\n");
    fprintf(out, "#include <stddef.h>\n\n");
    fprintf(out, "__attribute__((section(\".ExtQSPIFlashSection\")))\n");
    fprintf(out, "const uint8_t fs_image[%zu] = {", total_image_size);

    for (size_t i = 0; i < total_image_size; i++) {
        if (i % 16 == 0) {
            fprintf(out, "\n    ");
        }
        fprintf(out, "0x%02x%s", image[i], (i + 1 < total_image_size) ? ", " : "");
    }

    fprintf(out, "\n};\n\n");
    fprintf(out, "fs_image_size = %zu;\n", total_image_size);

    fclose(out);
    free(image);

    return 0;
}
