#include "finger.h"
#include "driver/uart.h"
#include "driver/gpio.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include <stdio.h>
#include <string.h>

#include <stdlib.h>
#include <stdint.h>
#include "esp_vfs.h"

ImagePlusResult acquire_fingerprint_image(uint8_t *image_buf, size_t buf_size)
{
    ImagePlusResult result = {.code = ESP_FAIL, .imagearray = NULL};

    if (image_buf == NULL || buf_size < IMAGE_DATA_SIZE)
    {
        printf("Erro: Buffer inválido ou tamanho insuficiente\n");
        result.code = ESP_ERR_INVALID_ARG;
        return result;
    }

    uint8_t cmd_buf[5] = {CMD_ACQUIRE_IMAGE, 0, 0, 0, 0};
    uint8_t rx_buf[8];

    // Enviar comando para adquirir imagem
    printf("Coloque o dedo no sensor para capturar imagem...\n");
    if (tx_rx_cmd(cmd_buf, 5, rx_buf, 8, pdMS_TO_TICKS(6000)) != ESP_OK)
    {
        printf("Erro na comunicação UART\n");
        result.code = ESP_FAIL;
        return result;
    }

    // Verificar resposta do cabeçalho
    if (rx_buf[0] != CMD_HEAD || rx_buf[7] != CMD_TAIL)
    {
        printf("Erro: Formato de resposta inválido\n");
        result.code = ESP_FAIL;
        return result;
    }

    // Verificar se houve sucesso na captura
    if (rx_buf[4] == ACK_FAIL)
    {
        printf("Falha na captura da imagem\n");
        result.code = ESP_FAIL;
        return result;
    }
    else if (rx_buf[4] == ACK_TIMEOUT)
    {
        printf("Timeout na captura da imagem\n");
        result.code = ESP_ERR_TIMEOUT;
        return result;
    }
    else if (rx_buf[4] != ACK_SUCCESS)
    {
        printf("Erro desconhecido na captura: 0x%02X\n", rx_buf[4]);
        result.code = ESP_FAIL;
        return result;
    }

    // Extrair o tamanho dos dados da imagem
    uint16_t data_len = (rx_buf[2] << 8) | rx_buf[3];
    printf("Tamanho dos dados da imagem: %d bytes\n", data_len);

    if (data_len != IMAGE_DATA_SIZE)
    {
        printf("Erro: Tamanho de dados inesperado (%d != %d)\n", data_len, IMAGE_DATA_SIZE);
        result.code = ESP_FAIL;
        return result;
    }
    // Preparar buffer para receber o pacote de dados
    uint8_t *data_packet = malloc(data_len + 3); // +3 para 0xF5, CHK, 0xF5
    if (data_packet == NULL)
    {
        printf("Erro: Falha na alocação de memória\n");
        result.code = ESP_ERR_NO_MEM;
        return result;
    }

    // Receber o pacote de dados da imagem
    int received_len = uart_read_bytes(UART_NUM, data_packet, data_len + 3, pdMS_TO_TICKS(10000));
    printf("Last: %02X\n", data_packet[3202]);
    printf("Last-1: %02X\n", data_packet[3201]);

    if (received_len != data_len + 3)
    {
        printf("Erro: Dados da imagem incompletos (%d/%d)\n", received_len, data_len + 3);
        free(data_packet);
        result.code = ESP_FAIL;
        return result;
    }

    // Verificar formato do pacote de dados
    if (data_packet[0] != CMD_HEAD || data_packet[data_len + 2] != CMD_TAIL)
    {
        printf("Erro: Formato do pacote de dados inválido\n");
        free(data_packet);
        result.code = ESP_FAIL;
        return result;
    }

    // Verificar checksum
    uint8_t calculated_checksum = 0;
    for (int i = 1; i < data_len + 1; i++)
    {
        calculated_checksum ^= data_packet[i];
    }

    if (calculated_checksum != data_packet[data_len + 1])
    {
        printf("Erro: Checksum inválido\n");
        free(data_packet);
        result.code = ESP_FAIL;
        return result;
    }

    // Copiar dados da imagem para o buffer fornecido
    memcpy(image_buf, &data_packet[1], data_len);

    free(data_packet);

    printf("Imagem capturada com sucesso! (%d bytes)\n", data_len);
    result.code = ESP_OK;
    result.imagearray = malloc(data_len);
    memcpy(result.imagearray, image_buf, data_len);
    return result;
}

uint8_t enroll_fingerprint(uint16_t user_id)
{
    uint8_t cmd_buf[5];
    uint8_t rx_buf[8];

    // 1ª etapa
    cmd_buf[0] = CMD_ADD_1;
    cmd_buf[1] = (user_id >> 8) & 0xFF;
    cmd_buf[2] = user_id & 0xFF;
    cmd_buf[3] = PERMISSION;
    cmd_buf[4] = 0;
    printf("Coloque o dedo no sensor (1ª etapa)...\n");
    if (tx_rx_cmd(cmd_buf, 5, rx_buf, 8, pdMS_TO_TICKS(6000)) != ESP_OK || rx_buf[4] != ACK_SUCCESS)
    {
        printf("Falha na 1ª etapa: 0x%02X\n", rx_buf[4]);
        return rx_buf[4];
    }

    // 2ª etapa
    cmd_buf[0] = CMD_ADD_2;
    printf("Coloque o dedo novamente (2ª etapa)...\n");
    if (tx_rx_cmd(cmd_buf, 5, rx_buf, 8, pdMS_TO_TICKS(6000)) != ESP_OK || rx_buf[4] != ACK_SUCCESS)
    {
        printf("Falha na 2ª etapa: 0x%02X\n", rx_buf[4]);
        return rx_buf[4];
    }

    // 3ª etapa
    cmd_buf[0] = CMD_ADD_3;
    printf("Coloque o dedo mais uma vez (3ª etapa)...\n");
    if (tx_rx_cmd(cmd_buf, 5, rx_buf, 8, pdMS_TO_TICKS(6000)) != ESP_OK || rx_buf[4] != ACK_SUCCESS)
    {
        printf("Falha na 3ª etapa: 0x%02X\n", rx_buf[4]);
        return rx_buf[4];
    }

    printf("Impressão digital enrolled com sucesso!\n");
    return ACK_SUCCESS;
}

void uart_init(void)
{
    uart_config_t uart_config = {
        .baud_rate = UART_BAUD_RATE,
        .data_bits = UART_DATA_8_BITS,
        .parity = UART_PARITY_DISABLE,
        .stop_bits = UART_STOP_BITS_1,
        .flow_ctrl = UART_HW_FLOWCTRL_DISABLE};
    uart_param_config(UART_NUM, &uart_config);
    uart_set_pin(UART_NUM, UART_TX_PIN, UART_RX_PIN, UART_PIN_NO_CHANGE, UART_PIN_NO_CHANGE);
    uart_driver_install(UART_NUM, 12288, 0, 0, NULL, 0); // 12KB buffer (maior que 3200)
}

void gpio_init(void)
{
    gpio_set_direction(FINGER_RST_PIN, GPIO_MODE_OUTPUT);
    gpio_set_direction(FINGER_WAKE_PIN, GPIO_MODE_INPUT);
    gpio_set_level(FINGER_RST_PIN, 1);
}

void sensor_reset(void)
{
    gpio_set_level(FINGER_RST_PIN, 0);
    vTaskDelay(pdMS_TO_TICKS(250));
    gpio_set_level(FINGER_RST_PIN, 1);
    vTaskDelay(pdMS_TO_TICKS(250));
}

esp_err_t tx_rx_cmd(const uint8_t *cmd_buf, size_t cmd_len, uint8_t *rx_buf, size_t rx_len, TickType_t timeout_ticks)
{
    uint8_t tx_buf[8];
    uint8_t checksum = 0;
    tx_buf[0] = CMD_HEAD;
    for (int i = 0; i < cmd_len; i++)
    {
        tx_buf[i + 1] = cmd_buf[i];
        checksum ^= cmd_buf[i];
    }
    tx_buf[6] = checksum;
    tx_buf[7] = CMD_TAIL;

    uart_flush(UART_NUM);
    uart_write_bytes(UART_NUM, (const char *)tx_buf, 8);

    int len = uart_read_bytes(UART_NUM, rx_buf, rx_len, timeout_ticks);
    if (len != rx_len)
        return ESP_FAIL;
    if (rx_buf[0] != CMD_HEAD || rx_buf[rx_len - 1] != CMD_TAIL)
        return ESP_FAIL;
    return ESP_OK;
}

uint8_t get_user_count(void)
{
    uint8_t cmd_buf[5] = {CMD_USER_CNT, 0, 0, 0, 0};
    uint8_t rx_buf[8];
    if (tx_rx_cmd(cmd_buf, 5, rx_buf, 8, pdMS_TO_TICKS(100)) != ESP_OK)
        return 0xFF;
    if (rx_buf[4] == ACK_SUCCESS)
        return rx_buf[3];
    return 0xFF;
}

uint8_t set_compare_level(uint8_t level)
{
    uint8_t cmd_buf[5] = {CMD_COM_LEV, 0, level, 0, 0};
    uint8_t rx_buf[8];
    if (tx_rx_cmd(cmd_buf, 5, rx_buf, 8, pdMS_TO_TICKS(100)) != ESP_OK)
        return 0xFF;
    if (rx_buf[4] == ACK_SUCCESS)
        return rx_buf[3];
    return 0xFF;
}

uint8_t add_user(void)
{
    uint8_t user_id = get_user_count() + 1;
    uint8_t cmd_buf[5] = {CMD_ADD_1, 0, user_id, 3, 0};
    uint8_t rx_buf[8];
    if (tx_rx_cmd(cmd_buf, 5, rx_buf, 8, pdMS_TO_TICKS(6000)) != ESP_OK)
        return ACK_TIMEOUT;
    if (rx_buf[4] == ACK_SUCCESS)
    {
        cmd_buf[0] = CMD_ADD_3;
        if (tx_rx_cmd(cmd_buf, 5, rx_buf, 8, pdMS_TO_TICKS(6000)) != ESP_OK)
            return ACK_TIMEOUT;
        if (rx_buf[4] == ACK_SUCCESS)
            return ACK_SUCCESS;
        return ACK_FAIL;
    }
    return ACK_FAIL;
}
