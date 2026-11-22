#ifndef SDRAM_H
#define SDRAM_H

#include "stm32f7xx_hal.h"
#include <stdbool.h>

#define SDRAM_BASE_ADDR 0xC0000000UL

HAL_StatusTypeDef SDRAM_Startup(void);

#endif
