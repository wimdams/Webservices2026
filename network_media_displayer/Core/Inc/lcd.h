#ifndef LCD_H
#define LCD_H

#include <stdint.h>
#include <stdbool.h>

#define LCD_WIDTH  480U
#define LCD_HEIGHT 272U

#define LCD_FB0_ADDR 0xC0000000UL
#define LCD_FB1_ADDR 0xC0040000UL

typedef uint8_t lcd_err_t;

typedef struct{
    uint8_t r;
    uint8_t g;
    uint8_t b;
} Color;

lcd_err_t LCD_init();

void LCD_clear(
    uint16_t *buffer,
    uint16_t width,
    uint16_t height,
    uint16_t color
);

void LCD_fill_rect(
    uint16_t* buffer,
    uint16_t width,
    uint16_t height,
    uint16_t x,
    uint16_t y,
    uint16_t rect_width,
    uint16_t rect_height,
    uint16_t color
);

lcd_err_t LCD_img_display_raw(uint16_t* data, uint16_t width, uint16_t height);

lcd_err_t LCD_txt_display(char* text);

lcd_err_t LCD_txt_set_color(Color color);

lcd_err_t LCD_bg_set_color(Color color);

lcd_err_t LCD_smiile_display(char* filename);

lcd_err_t LCD_bliss_display(char* filename);

#endif
