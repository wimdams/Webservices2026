/*
 * lcd_bl.c
 *
 *  Created on: Nov 23, 2026
 *      Author: decency
 */


#include "main.h"
#include "lcd_bl.h"

void LCD_BL_Enable(void){
    HAL_GPIO_WritePin(GPIOI, GPIO_PIN_12, GPIO_PIN_SET);

    HAL_GPIO_WritePin(GPIOK, GPIO_PIN_3, GPIO_PIN_SET);
}
