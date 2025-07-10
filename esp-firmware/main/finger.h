#pragma once
#include <stdint.h>
#include "esp_err.h"
#include "freertos/FreeRTOS.h"

#define UART_NUM UART_NUM_2
#define UART_TX_PIN 17
#define UART_RX_PIN 18
#define UART_BAUD_RATE 19200

#define FINGER_RST_PIN 4
#define FINGER_WAKE_PIN 5

#define CMD_HEAD 0xF5
#define CMD_TAIL 0xF5

#define CMD_ADD_1 0x01
#define CMD_ADD_2 0x02
#define CMD_ADD_3 0x03
#define CMD_MATCH 0x0C
#define CMD_DEL 0x04
#define CMD_DEL_ALL 0x05
#define CMD_USER_CNT 0x09
#define CMD_COM_LEV 0x28
#define CMD_TIMEOUT 0x2E

#define ACK_SUCCESS 0x00
#define ACK_FAIL 0x01
#define ACK_FULL 0x04
#define ACK_NO_USER 0x05
#define ACK_TIMEOUT 0x08

#define CMD_ACQUIRE_IMAGE 0x24
#define IMAGE_DATA_SIZE 3200 // Tamanho fixo da imagem conforme manual

#define PERMISSION 3 // Pode ser 1, 2 ou 3

// Estrutura para cabeçalho BMP
#pragma pack(push, 1)
typedef struct
{
    esp_err_t code;      // Código de resultado
    uint8_t *imagearray; // Dados da imagem
} ImagePlusResult;
#pragma pack(pop)

void uart_init(void);
void gpio_init(void);
void sensor_reset(void);
uint8_t enroll_fingerprint(uint16_t user_id);
// Novas funções para captura de imagem
ImagePlusResult acquire_fingerprint_image(uint8_t *image_buf, size_t buf_size);
esp_err_t save_fingerprint_image_bmp(uint8_t *image_data, const char *filename);

uint8_t get_user_count(void);
uint8_t add_user(void);
uint8_t set_compare_level(uint8_t level);
esp_err_t tx_rx_cmd(uint8_t *cmd_buf, size_t cmd_len, uint8_t *rx_buf, size_t rx_len, TickType_t timeout_ticks);
