#include "finger.h"
#include "driver/gpio.h"
#include "config.h"
#include "mqtt_client.h"
#include "esp_wifi.h"
#include "esp_event.h"
#include "nvs_flash.h"
#include "esp_log.h"

#include <stdio.h>

static void wifi_event_handler(void *arg, esp_event_base_t event_base,
                               int32_t event_id, void *event_data)
{
    if (event_base == WIFI_EVENT && event_id == WIFI_EVENT_STA_START)
    {
        esp_wifi_connect();
    }
    else if (event_base == WIFI_EVENT && event_id == WIFI_EVENT_STA_DISCONNECTED)
    {
        esp_wifi_connect();
    }
}

static void wifi_init(void)
{
    ESP_ERROR_CHECK(esp_netif_init());
    ESP_ERROR_CHECK(esp_event_loop_create_default());
    esp_netif_create_default_wifi_sta();
    wifi_init_config_t cfg = WIFI_INIT_CONFIG_DEFAULT();
    ESP_ERROR_CHECK(esp_wifi_init(&cfg));
    wifi_config_t wifi_config = {
        .sta = {
            .ssid = WIFI_SSID,
            .password = WIFI_PASSWORD,
            .threshold.authmode = WIFI_AUTH_WPA2_PSK,
        },
    };
    ESP_ERROR_CHECK(esp_event_handler_register(WIFI_EVENT, ESP_EVENT_ANY_ID, &wifi_event_handler, NULL));
    ESP_ERROR_CHECK(esp_wifi_set_mode(WIFI_MODE_STA));
    ESP_ERROR_CHECK(esp_wifi_set_config(WIFI_IF_STA, &wifi_config));
    ESP_ERROR_CHECK(esp_wifi_start());
}

static const char *TAG = "MQTT_FINGER";
static esp_mqtt_client_handle_t mqtt_client = NULL;

static void mqtt_event_handler(void *handler_args, esp_event_base_t base,
                               int32_t event_id, void *event_data)
{
    esp_mqtt_event_handle_t event = event_data;
    switch ((esp_mqtt_event_id_t)event_id)
    {
    case MQTT_EVENT_CONNECTED:
        ESP_LOGI(TAG, "MQTT conectado");
        break;
    case MQTT_EVENT_DISCONNECTED:
        ESP_LOGW(TAG, "MQTT desconectado");
        break;
    default:
        break;
    }
}

static void mqtt_app_start(void)
{
    esp_mqtt_client_config_t mqtt_cfg = {
        .broker.address.uri = MQTT_BROKER_URI
        // .username = MQTT_USERNAME, // Vai ser necessário
        // .password = MQTT_PASSWORD, // Vai ser necessário
    };
    mqtt_client = esp_mqtt_client_init(&mqtt_cfg);
    esp_mqtt_client_register_event(mqtt_client, ESP_EVENT_ANY_ID, mqtt_event_handler, NULL);
    esp_mqtt_client_start(mqtt_client);
}

void app_main(void)
{
    ESP_ERROR_CHECK(nvs_flash_init());
    wifi_init();

    vTaskDelay(pdMS_TO_TICKS(3000));
    mqtt_app_start();
    vTaskDelay(pdMS_TO_TICKS(2000));

    uart_init();
    gpio_init();

    printf("Iniciando sensor...\n");
    sensor_reset();

    while (1)
    {
        vTaskDelay(pdMS_TO_TICKS(500));
        if (set_compare_level(5) == 5)
        {
            printf("Sensor pronto!\n");
            uint8_t user_count = get_user_count();
            printf("Usuários cadastrados: %d\n", user_count);
            // uint16_t user_id = get_user_count() + 1;
            // enroll_fingerprint(user_id);
            // break;

            uint8_t *image_buffer = malloc(IMAGE_DATA_SIZE);
            if (image_buffer != NULL)
            {
                ImagePlusResult result = acquire_fingerprint_image(image_buffer, IMAGE_DATA_SIZE);
                if (result.code == ESP_OK)
                {
                    printf("Printing image:\n");
                    for (int i = 0; i < IMAGE_DATA_SIZE; i++)
                    {
                        printf("%02X", result.imagearray[i]);
                        if ((i + 1) % 16 == 0)
                            printf("\n");
                    }
                    printf("\n");
                    // coloca tudo o que  este no result.imagearray em uma string normal
                    char *image_string = malloc((IMAGE_DATA_SIZE * 2 + 1) * sizeof(char));
                    if (image_string != NULL)
                    {
                        for (int i = 0; i < IMAGE_DATA_SIZE; i++)
                        {
                            sprintf(&image_string[i * 2], "%02X", result.imagearray[i]);
                        }
                        image_string[IMAGE_DATA_SIZE * 2] = '\0';
                    }
                    else
                    {
                        ESP_LOGE(TAG, "Erro ao alocar memória para string de imagem");
                    }

                    // // agora envia essa string para o mqtt
                    int msg_id = esp_mqtt_client_publish(
                        mqtt_client,
                        "fingerprint/image",
                        image_string,
                        0,
                        0,
                        0);

                    ESP_LOGI(TAG, "Imagem publicada no MQTT, msg_id=%d", msg_id);
                }

                free(image_buffer);
                free(result.imagearray);
            }
        }
        else
        {
            printf("Erro: Verifique alimentação e conexão UART.\n");
        }
    }
}
