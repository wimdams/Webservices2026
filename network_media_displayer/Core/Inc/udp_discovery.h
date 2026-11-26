/*
 * udp_discovery.h
 *
 *  Created on: Nov 23, 2026
 *      Author: simon
 */

#ifndef INC_UDP_DISCOVERY_H_
#define INC_UDP_DISCOVERY_H_

#include "lwip/udp.h"
#include "lwip/pbuf.h"
#include "lwip/ip_addr.h"
#include <stdint.h>

/**
 * @brief Initializes the UDP broadcast listener on port 64000 and sets the owner's name.
 *
 * @param firstName First name of the owner
 * @param lastName  Last name of the owner
 */
err_t UDP_Discovery_Init(const char *firstName, const char *lastName);

#endif /* INC_UDP_DISCOVERY_H_ */
