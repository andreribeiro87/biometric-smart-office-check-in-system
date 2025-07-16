# Biometric Smart Office Check-in System

Developed in APSIDE Portugal as part of the summer internship program, this project uses Rust and Docker, together with an ESP32 microcontroller and a fingerprint sensor, to a check-in system for office employees. 

## Technical Details
Waveshare Fingerprint Sensor (D) is connected to an ESP32-S3-DEV-KIT-N8R8, which is powered via USB.

### Connections

|Fingerprint Sensor             | ESP32-S3-DEV-KIT-N8R8         |
|-------------------------------|-------------------------------|
| GND                           | GND                           |
| VCC                           | 3.3V                          |
| RX                            | GPIO 17 (TX)                  |
| TX                            | GPIO 18 (RX)                  |
| EN                            | No Connection                 |
| IRQ                           | No Connection                 |

### Firmware
The ESP32 firmware is written in C and uses the ESP-IDF framework. It handles the communication with the fingerprint sensor, processes the fingerprint data, and publishes it to an MQTT topic.

## Docker Setup

- Postgres DB 
- MQTT Connection
- Rust API

### Postgres DB

#TODO

### MQTT Connection
ESP32 publises messages to the MQTT topic `fingerprint/image`, which the Rust API subscribes to. 

### Rust API
The Rust API is responsible for receiving the data from the sensor via MQTT and processing it. Once received, the data is exposed on `\fingerprint` endpoint. It also processes the data into an image which is exposed on the `\image` endpoint.

#### Endpoints
These endpoints are available at `localhost:3000`:
- `/fingerprint` - returns the fingerprint data 
- `/image` - returns the processed image data
- `/health` - returns the health status of the API

## Running the Project
1. Ensure Docker is installed and running.
2. Clone the repository.
3. Navigate to the project directory.
4. Build and run the Docker containers:
   ```bash
   docker-compose up --build
   ```
5. Access the API at `http://localhost:3000\*endpoint*`.

## Running the ESP32
1. Install the ESP-IDF development environment.
2. Connect the ESP32 to your computer via USB.
3. Flash the ESP32 with the provided firmware via the ESP-IDF tools:
   ```bash
   idf.py flash monitor
   ```
4. Ensure the ESP32 is connected to the same network as the Docker containers.

**NOTE**: As the Fingerprint Sensor IRQ pin is not connected (the WAKE pin), the ESP32 will initially state that the sensor is not connected. This is expected behavior, and when a finger is placed on the sensor, it will start functioning correctly.

### Contribution
André Ribeiro
Daniel Pedrinho 


