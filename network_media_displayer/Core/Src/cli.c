/*
 * cli.c
 *
 *  Created on: Nov 19, 2026
 *      Author: Admin User
 */

#include <cli.h>


struct tcp_pcb *server;
ip_addr_t serverIp;

#define CLI_BUFFER_SIZE 128
static char cli_buffer[CLI_BUFFER_SIZE];
static uint16_t cli_index = 0;

static bool last_was_carr_return = false;

static const char WELCOME[] =
  "\r\n=== STM32 CLI ===\r\n"
  "Welcome! These are the available commands:\r\n\r\n";

static const char HELP[] =
  "  \r\n--------- These are all the commands that you can use: ---------\r\n"
  "  help                 Shows this list\r\n"
  "  text <text>         Displays text on LCD\r\n"
  "  color <color>        Sets background color of the LCD (RGBA format)\r\n"
  "  textColor <color>    Sets the text color of the LCD (RGBA format)\r\n"
  "  listImages           Shows all available images\r\n"
  "  listAudio            Shows all available audio files\r\n"
  "  setImage <name>      Displays the choosen image on the LCD\r\n"
  "  playAudio <name>     Plays the choosen audio file\r\n"
  "  exit                 Closes tcp connection\r\n\r\n";

static const char CLOSE[] =
  "\r\n ------- This connection is closed ------- \r\n";

static const char PROMPT[] = "> ";

static void cli_send_str(struct tcp_pcb *client, const char *s) {
	err_t err = tcp_write(client, s, strlen(s), 0);
	  if (err == ERR_OK) {
	    err = tcp_output(client);
	  }
	  printf("cli_send_str: err=%d\r\n", err);
}

static bool cli_process_command(struct tcp_pcb *client, char *command){

	char *cmd = command;
	char *arg = strchr(command, ' ');

	if (arg != NULL){
		*arg = '\0';
		arg++;
		while (*arg == ' ') arg++;
	} else {
		arg = "";
	}

	printf("cmd = %s\r\n", cmd);
	printf("arg = %s\r\n", arg);

	if (strcmp(command, "help") == 0){
		cli_send_str(client, HELP);
	}/*
	else if (strcmp(command, "text") == 0){
		LCD_txt_display(arg);
	}
	else if (strcmp(command, "color") == 0){
		LCD_bg_set_color(struct Color color);
	}
	else if (strcmp(command, "textColor") == 0){
		LCD_txt_set_color(struct Color color);
	}
	else if (strcmp(command, "listImages") == 0){
		fs_search_files(*.png);
	}
	else if (strcmp(command, "listAudio") == 0) {
		fs_search_files(*.mp3);
	}
	else if (strcmp(command, "setImage") == 0){
		LCD_img_display(char* filename);
	}
	else if (strcmp(command, "playAudio") == 0){

	}*/
	else if (strcmp(command, "exit") == 0){
		cli_send_str(client, CLOSE);
		tcp_close(client);
		return true;
	}
	return false;
}

err_t cli_callback_recv(void *arg, struct tcp_pcb *client, struct pbuf *p, err_t err) {
    if (p != NULL) {
        struct pbuf *q = p;
        while (q != NULL) {
            char *data = (char *)q->payload;
            for (uint16_t i = 0; i < q->len; i++) {
                char c = data[i];
                if (c == '\r' || c == '\n') {
                	if (c == '\n' && last_was_carr_return) {
                		last_was_carr_return = false;
                	        continue;
                	    }
                	last_was_carr_return = (c == '\r');
                    if (cli_index > 0) {
                        cli_buffer[cli_index] = '\0';
                        printf("Commando: %s\r\n", cli_buffer);
                        bool tcp_closed = cli_process_command(client, cli_buffer);
                        cli_index = 0;

                        if (tcp_closed) {
                        	tcp_recved(client, p->tot_len);
                        	pbuf_free(p);
                        	return ERR_OK;
                        }
                    }
                    cli_send_str(client, PROMPT);
                } else  {
                	last_was_carr_return = false;
                	if (cli_index < CLI_BUFFER_SIZE - 1) {
                    	cli_buffer[cli_index++] = c;
                	}
                }
            }
            q = q->next;
        }
        tcp_recved(client, p->tot_len);
        pbuf_free(p);
    } else {
        printf("Connection is getting closed by the server\r\n");
        tcp_close(client);
    }
    return ERR_OK;
}

static err_t cli_accept(void *arg, struct tcp_pcb *client, err_t err) {
  tcp_recv(client, cli_callback_recv);
  cli_send_str(client, WELCOME);
  cli_send_str(client, HELP);
  cli_send_str(client, PROMPT);
  return ERR_OK;
}

void cli_init() {
	server = tcp_new();
	IP4_ADDR(&serverIp, 192,168,69,10);
	tcp_bind(server, &serverIp, CLI_PORT);
	server = tcp_listen(server);
	tcp_accept(server, cli_accept);
}

