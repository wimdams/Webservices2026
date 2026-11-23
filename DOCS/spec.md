# Specification for the network_media_displayer API and file formats

## File formats

### smiile

> [!NOTE]
> smiile stands for Silly Moving and Interchanching Image (Little Endian) 

A custom file format with the goal of more compact animated image representation. As the name suggests, the byte order is little endian.

The format is:

- Header:
  - Magic number: u32
  - Version: u8
  - Flags: u8
  - Lookup table length: u16 (0 when no LUT)
  - Height of the image: u32
  - Width of the image: u32
  - Frame count: u32
- Body:
  - (Optional) Lookup table:
    - Colors(rgb565)
  - Frames, repeated frame_count times:
    - Pixel color data (either rgb565 or LUT index) for every pixel
    - delay until next frame (ms): u16 

Flags:
|Bits|Meaning|
|:-|:-|
|0-1|Pixel encoding. `00` = RGB565, `01` = 8bit LUT, `10` = 16bit LUT|
|2-7|Reserved|

Pixel order is left to right, top to down. Each frame has exactly `width * height` pixels, follower by a u16 delay duration in ms. 
  
### bliss

> [!NOTE]
> bliss stands for Binary Lightweight Interchangeable Spatial Structure

A custom file format to minimally represent a 3d object. The byte order is little endian.

The format is :

- Header:
  - Magic number: u32
  - Version: u8
  - Reserved: \[u8, 3] (Padding)
  - Number of vertices: u32
  - Number of edges: u32
- Body:
  - Verteces: `vertex count * {x: f32, y:f32, z:f32}`
  - Edges: `edge count * {a: u32, b: u32}`

Coordinates are represented by f32 values in the inclusive range [-1.0, 1.0]. NaN and infinity are invalid. The coordinate system is left handed. Positive X points right, positive Y points up, positive Z points forward.

Indices are zero-based.

Self-referencing edges are invalid. Duplicate edges are invalid. Edge direction does not matter.
```
0 <= a < vertex_count
0 <= b < vertex_count

a < b
```

## LCD

### Error codes

|Code|Reason|
|:-:|:-:|
|0|OK|

TODO

### Functions

```C
lcd_err_t LCD_img_display(char* filename);
```
Displays the image with the specified file name.

```C
lcd_err_t LCD_img_display_raw(uint16_t* data, uint16_t height, uint16_t width);
```
Displays the image from raw data.

```C
lcd_err_t LCD_txt_display(char* text);
```
Displays the given text.

```C
lcd_err_t LCD_txt_set_color(struct Color color);
```
Sets text color.

```C
lcd_err_t LCD_bg_set_color(struct Color color);
```
Sets background color.

```C
lcd_err_t LCD_smiile_display(char* filemane);
```
Displays an animation of the smiile format withe a given file name.

```C
lcd_err_t LCD_bliss_display(char* filename);
```
displays a 3d object of the bliss format with a given file name.

## Filesystem

### What is a file?

```C
typedef struct file { 
    char* file_name;        // file name including extention
    uint32_t file_size;     // file size in bytes
    void* file_startaddr;   // pointer to where the file starts
} file;

typedef file* file_ID;
```

### Functions

```C
file_ID* fs_search_files(const char* filename);
```
Allows you to search the filesystem for one of the following:
* `"*"`: all files
* `"*.png"`: all files with a certain extension
* `"file.ext"`: specific file

The function returns a list of pointers to files that were found with the last element in the list being a NULL pointer.
This list is heap-allocated and must be freed by the user. 

```C
const char* fs_get_filename(file_ID file);
```
Returns the filename associated with the fileID.

```C
uint32_t fs_get_filesize(file_ID file);
```
Returns the filesize of a file.

```C
void* fs_get_file_addr(file_ID file);
```
Returns a pointer to the start of a file.
