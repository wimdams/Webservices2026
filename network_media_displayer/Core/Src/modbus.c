/*
 * modbus.c
 *
 *  Created on: Nov 18, 2026
 *      Author: larsv
 */


//#INCLUDES
#include "modbus.h"		// eigen header file waarin je de server aanzet.
#include "tcp.h"		// Deze is nodig om geheugen van een pcb te alloceren en andere zaken om met de juiste poort te verbinden, ... .
#include <stdio.h>		// dit dient voor de printf te kunnen gebruiken en is een C-lib.
#include <stdint.h>		//deze include dient voor uint te gebruiken.

//Functie Definities
static err_t modbus_recv(void *arg, struct tcp_pcb *tpcb, struct pbuf *p, err_t err){
	// controleert dat de buffer geen ongeldige pointer terug geeft
	if(p == NULL){

		tcp_close(tpcb);

		return ERR_OK;
	}

	// als de totale lengte van de buffer groter is dan 300 dan sturen we wel nog iets dat we iets ontvangen hebben en maken we het geheugen weeer vrij
	if(p -> tot_len > 300){

		tcp_recved(tpcb, p->tot_len);
		pbuf_free(p);

		return ERR_OK;
	}

	// dit gebeurd er wanneer alles goed gaat.
	uint8_t rx_buf[300];		// aanmaken van een buffer var

	uint16_t len = p->tot_len;	// de totale lengte van de pbuf type in len plaatsten

	pbuf_copy_partial(p, rx_buf, len, 0);	// het kopieren van de data in rx_buf

	tcp_recved(tpcb, len);

	pbuf_free(p);

	for(int i = 0; i < len; i++){
		printf("%02X ", rx_buf[i]);

	}

	printf("\r\n");
	return ERR_OK;
}


// de callback-functie is static dat wil zeggen dat alleen deze functie in dit c-bestand kan gebruikt worden en niet bv. in de main zoals de enable_modbus_server.
static err_t modbus_accept(void *arg, struct tcp_pcb *tpcb, err_t err){
	//deze functie best boven de enable_modbus_server want deze is de caalback functie daar in gebruikt wordt.

	// check of de nieuwe verbinding gelukt is
	if(err != ERR_OK || tpcb == NULL){

		return ERR_VAL;
	} else {
		// koppel de ontvangst-callback aan deze client-verbinding
		tcp_recv(tpcb, modbus_recv);

		return ERR_OK;
	}
}


void  enable_modbus_server(void){

	// tcp_pcb is een type waar alle info van de netwerkverbinding in staat. connection is een pointer naar het adres van tcp_pcb.
	struct tcp_pcb *connection;

	// error type var aanamekn
	err_t err;

	// hier alloceer je het geheugen in de LwIP memory pool (soort van heap) van het type struct tcp_pcb
	connection = tcp_new();

	if (connection != NULL){
		// we gaan de poort, en het IP-adress koppelen met de connectie.
		err = tcp_bind(connection, IP_ADDR_ANY ,502);

			if(err != ERR_OK){
				printf("the port is not free!\r\n");

				tcp_close(connection);

				return;
			}

		//de server wacht op een connectie van de client
		connection = tcp_listen(connection);

			if (connection != NULL){
				// dan kan de connectie geaccepteerd worden door de server en wanneer de server de verbdinding met de client accepteert
				tcp_accept(connection, modbus_accept);

			} else {
				// we sturen een foutmelding door
				printf("the memory is full!\r\n");
			}

	} else {
		// we sturen een foutmelding door
		printf("the memory is full!\r\n");
	}

}

