# Specification for the network_media_displayer API and file formats

## File formats

### smiile

> [!NOTE]
> smiile stands for Silly Moving and Interchanching Image (Little Endian) 

A custom file format with the goal of more compact animated image representation.

The format is:

- Header:
  - Magic number: u32
  - Height of the image: u32
  - Width of the image: u32
  - Lookup tabe length: u16
- Body:
  - (Optional) Lookup table:
    - Colors(rgb565)
  - Frames:
    - Pixel color data (either rgb565 or LUT index) for every pixel
    - delay until next frame: u16
  
### bliss

> [!NOTE]
> bliss stands for Binary Lightweight Interchangeable Spatial Structure

A custom file format to minimally represent a 3d object.

The format is :

- Header:
  - Magic number: u32
  - Number of points: u32
- Body:
  - Points represented by `x: f32, y:f32, z:f32`
  - Array of indeces to represent "meshes"

Coordinates are represented by nubers between -1 and 1 with {0;0;0} being the center.

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
lcd_err_t LCD_img_display_raw(struct Color* data, uint16_t height, uint16_t width);
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
