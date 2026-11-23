#ifndef LCD_H
#define LCD_H

#include <stdint.h>

#define LCD_WIDTH  480U
#define LCD_HEIGHT 272U

#define LCD_FB0_ADDR 0xC0000000UL
#define LCD_FB1_ADDR 0xC0040000UL

void LCD_clear(
    uint16_t *buffer,
    uint16_t width,
    uint16_t height,
    uint16_t color
);

void LCD_fill_rect(
    uint16_t *buffer,
    uint16_t width,
    uint16_t height,
    uint16_t x,
    uint16_t y,
    uint16_t rect_width,
    uint16_t rect_height,
    uint16_t color
);


#endif
