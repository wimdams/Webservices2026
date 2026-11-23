/*
 * cli.h
 *
 *  Created on: Nov 19, 2026
 *      Author: Admin User
 */

#ifndef CLI_H_
#define CLI_H_

#include "lwip/tcp.h"
#include "lwip/ip_addr.h"
#include <stdio.h>
#include <string.h>
#include <stdbool.h>

#define CLI_PORT 23

void cli_init(void);

#endif /* CLI_H_ */
