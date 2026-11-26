/*
 * udp_discovery.c
 *
 *  Created on: Nov 23, 2026
 *      Author: simon
 */

#include "udp_discovery.h"
#include "lwip/netif.h"
#include <stdio.h>
#include <string.h>

#define DISCOVERY_PORT        64000
#define DISCOVERY_QUERY_MSG   "Where are you?v1.0"
#define DISCOVERY_QUERY_LEN   (sizeof(DISCOVERY_QUERY_MSG) - 1)

/* Externally defined in lwip.c */
extern struct netif gnetif;

static struct udp_pcb *discovery_pcb = NULL;
static char first_name[64] = "";
static char last_name[64]  = "";

/**
 * @brief Callback function called by LwIP when a UDP packet is received.
 *
 * @param arg  Optional user argument passed to udp_recv() (unused)
 * @param pcb  The UDP Protocol Control Block
 * @param p    Pointer to the received pbuf data ("Where are you?v1.0")
 * @param addr IP address of the sender
 * @param port Port number of the sender
 */
static void udp_discovery_recv_callback(void *arg, struct udp_pcb *pcb, struct pbuf *p, const ip_addr_t *addr, u16_t port)
{
    (void)arg;

    if (p == NULL) {
        return;
    }

    // Check if the incoming payload matches "Where are you?v1.0" exactly
    if (p->tot_len == DISCOVERY_QUERY_LEN) {
        char rx_buf[DISCOVERY_QUERY_LEN];
        if ((pbuf_copy_partial(p, rx_buf, DISCOVERY_QUERY_LEN, 0) == DISCOVERY_QUERY_LEN) &&
            (memcmp(rx_buf, DISCOVERY_QUERY_MSG, DISCOVERY_QUERY_LEN) == 0)) {
            char reply[192];
            const uint8_t *mac = gnetif.hwaddr;

            int written = snprintf(reply, sizeof(reply),
                                   "%02X:%02X:%02X:%02X:%02X:%02X is present and my owner is %s %s",
                                   mac[0], mac[1], mac[2], mac[3], mac[4], mac[5],
                                   first_name, last_name);

            if ((written > 0) && ((size_t)written < sizeof(reply))) {
                struct pbuf *tx_pbuf = pbuf_alloc(PBUF_TRANSPORT, (u16_t)written, PBUF_RAM);
                if (tx_pbuf != NULL) {
                    pbuf_take(tx_pbuf, reply, (u16_t)written);
                    // Send the reply directly back to the sender's IP address and port
                    udp_sendto(pcb, tx_pbuf, addr, port);
                    pbuf_free(tx_pbuf);
                }
            }
        }
    }

    pbuf_free(p);
}

err_t UDP_Discovery_Init(const char *firstName, const char *lastName)
{
    if (firstName != NULL) {
        snprintf(first_name, sizeof(first_name), "%s", firstName);
    } else {
        first_name[0] = '\0';
    }

    if (lastName != NULL) {
        snprintf(last_name, sizeof(last_name), "%s", lastName);
    } else {
        last_name[0] = '\0';
    }

    // Delete any previous PCB in case the init function is called more than once
    if (discovery_pcb != NULL) {
        udp_remove(discovery_pcb);
        discovery_pcb = NULL;
    }

    discovery_pcb = udp_new();
    if (discovery_pcb == NULL) {
        return ERR_MEM;
    }

    // Listen on all local IP addresses on port 64000
    err_t err = udp_bind(discovery_pcb, IP_ADDR_ANY, DISCOVERY_PORT);
    if (err != ERR_OK) {
        udp_remove(discovery_pcb);
        discovery_pcb = NULL;
        return err;
    }

    udp_recv(discovery_pcb, udp_discovery_recv_callback, NULL);
    return ERR_OK;
}
