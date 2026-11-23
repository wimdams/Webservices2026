/*
 * sdram.c
 *
 *  Created on: Nov 22, 2026
 *      Author: decency
 */

#include "sdram.h"

extern SDRAM_HandleTypeDef hsdram1;

#define SDRAM_TIMEOUT 0xFFFFU

static HAL_StatusTypeDef send_cmd(
    uint32_t mode,
    uint32_t refresh,
    uint32_t mode_reg
) {
    FMC_SDRAM_CommandTypeDef cmd = {0};

    cmd.CommandMode = mode;
    cmd.CommandTarget = FMC_SDRAM_CMD_TARGET_BANK1;
    cmd.AutoRefreshNumber = refresh;
    cmd.ModeRegisterDefinition = mode_reg;

    return HAL_SDRAM_SendCommand(
        &hsdram1, &cmd, SDRAM_TIMEOUT
    );
}

HAL_StatusTypeDef SDRAM_Startup(void){
    HAL_StatusTypeDef status;

    status = send_cmd(FMC_SDRAM_CMD_CLK_ENABLE, 1, 0);
    if (status != HAL_OK) return status;

    HAL_Delay(1);

    status = send_cmd(FMC_SDRAM_CMD_PALL, 1, 0);
    if (status != HAL_OK) return status;

    status = send_cmd(
        FMC_SDRAM_CMD_AUTOREFRESH_MODE, 8, 0
    );
    if (status != HAL_OK) return status;

    uint32_t mode_reg =
        0x0000U |
        0x0030U |
        0x0200U;

    status = send_cmd(
        FMC_SDRAM_CMD_LOAD_MODE, 1, mode_reg
    );
    if (status != HAL_OK) return status;

    return HAL_SDRAM_ProgramRefreshRate(
        &hsdram1, 0x0603U
    );
}
