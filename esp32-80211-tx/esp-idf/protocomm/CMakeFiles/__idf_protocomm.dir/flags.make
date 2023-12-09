# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.25

# compile C with /home/cdjk/.espressif/tools/xtensa-esp-elf/esp-13.2.0_20230928/xtensa-esp-elf/bin/xtensa-esp32-elf-gcc
C_DEFINES = -DESP_PLATFORM -DIDF_VER=\"v5.3-dev-422-ga7fbf452fa\" -DMBEDTLS_CONFIG_FILE=\"mbedtls/esp_config.h\" -DSOC_MMU_PAGE_SIZE=CONFIG_MMU_PAGE_SIZE -DSOC_XTAL_FREQ_MHZ=CONFIG_XTAL_FREQ -D_GLIBCXX_HAVE_POSIX_SEMAPHORE -D_GLIBCXX_USE_POSIX_SEMAPHORE -D_GNU_SOURCE -D_POSIX_READER_WRITER_LOCKS

C_INCLUDES = -I/home/cdjk/github/esp-items/esp32-80211-tx/config -I/home/cdjk/esp/esp-idf/components/protocomm/include/common -I/home/cdjk/esp/esp-idf/components/protocomm/include/security -I/home/cdjk/esp/esp-idf/components/protocomm/include/transports -I/home/cdjk/esp/esp-idf/components/protocomm/proto-c -I/home/cdjk/esp/esp-idf/components/protocomm/src/common -I/home/cdjk/esp/esp-idf/components/protocomm/src/crypto/srp6a/include -I/home/cdjk/esp/esp-idf/components/newlib/platform_include -I/home/cdjk/esp/esp-idf/components/freertos/config/include -I/home/cdjk/esp/esp-idf/components/freertos/config/include/freertos -I/home/cdjk/esp/esp-idf/components/freertos/config/xtensa/include -I/home/cdjk/esp/esp-idf/components/freertos/FreeRTOS-Kernel/include -I/home/cdjk/esp/esp-idf/components/freertos/FreeRTOS-Kernel/portable/xtensa/include -I/home/cdjk/esp/esp-idf/components/freertos/FreeRTOS-Kernel/portable/xtensa/include/freertos -I/home/cdjk/esp/esp-idf/components/freertos/esp_additions/include -I/home/cdjk/esp/esp-idf/components/esp_hw_support/include -I/home/cdjk/esp/esp-idf/components/esp_hw_support/include/soc -I/home/cdjk/esp/esp-idf/components/esp_hw_support/include/soc/esp32 -I/home/cdjk/esp/esp-idf/components/esp_hw_support/port/esp32/. -I/home/cdjk/esp/esp-idf/components/esp_hw_support/port/esp32/private_include -I/home/cdjk/esp/esp-idf/components/heap/include -I/home/cdjk/esp/esp-idf/components/log/include -I/home/cdjk/esp/esp-idf/components/soc/include -I/home/cdjk/esp/esp-idf/components/soc/esp32 -I/home/cdjk/esp/esp-idf/components/soc/esp32/include -I/home/cdjk/esp/esp-idf/components/hal/platform_port/include -I/home/cdjk/esp/esp-idf/components/hal/esp32/include -I/home/cdjk/esp/esp-idf/components/hal/include -I/home/cdjk/esp/esp-idf/components/esp_rom/include -I/home/cdjk/esp/esp-idf/components/esp_rom/include/esp32 -I/home/cdjk/esp/esp-idf/components/esp_rom/esp32 -I/home/cdjk/esp/esp-idf/components/esp_common/include -I/home/cdjk/esp/esp-idf/components/esp_system/include -I/home/cdjk/esp/esp-idf/components/esp_system/port/soc -I/home/cdjk/esp/esp-idf/components/esp_system/port/include/private -I/home/cdjk/esp/esp-idf/components/xtensa/esp32/include -I/home/cdjk/esp/esp-idf/components/xtensa/include -I/home/cdjk/esp/esp-idf/components/xtensa/deprecated_include -I/home/cdjk/esp/esp-idf/components/lwip/include -I/home/cdjk/esp/esp-idf/components/lwip/include/apps -I/home/cdjk/esp/esp-idf/components/lwip/include/apps/sntp -I/home/cdjk/esp/esp-idf/components/lwip/lwip/src/include -I/home/cdjk/esp/esp-idf/components/lwip/port/include -I/home/cdjk/esp/esp-idf/components/lwip/port/freertos/include -I/home/cdjk/esp/esp-idf/components/lwip/port/esp32xx/include -I/home/cdjk/esp/esp-idf/components/lwip/port/esp32xx/include/arch -I/home/cdjk/esp/esp-idf/components/lwip/port/esp32xx/include/sys -I/home/cdjk/esp/esp-idf/components/esp_timer/include -I/home/cdjk/esp/esp-idf/components/esp_wifi/include -I/home/cdjk/esp/esp-idf/components/esp_wifi/wifi_apps/include -I/home/cdjk/esp/esp-idf/components/esp_event/include -I/home/cdjk/esp/esp-idf/components/esp_phy/include -I/home/cdjk/esp/esp-idf/components/esp_phy/esp32/include -I/home/cdjk/esp/esp-idf/components/esp_netif/include -I/home/cdjk/esp/esp-idf/components/protobuf-c/protobuf-c -I/home/cdjk/esp/esp-idf/components/mbedtls/port/include -I/home/cdjk/esp/esp-idf/components/mbedtls/mbedtls/include -I/home/cdjk/esp/esp-idf/components/mbedtls/mbedtls/library -I/home/cdjk/esp/esp-idf/components/mbedtls/esp_crt_bundle/include -I/home/cdjk/esp/esp-idf/components/mbedtls/mbedtls/3rdparty/everest/include -I/home/cdjk/esp/esp-idf/components/mbedtls/mbedtls/3rdparty/p256-m -I/home/cdjk/esp/esp-idf/components/mbedtls/mbedtls/3rdparty/p256-m/p256-m -I/home/cdjk/esp/esp-idf/components/console -I/home/cdjk/esp/esp-idf/components/vfs/include -I/home/cdjk/esp/esp-idf/components/esp_http_server/include -I/home/cdjk/esp/esp-idf/components/http_parser -I/home/cdjk/esp/esp-idf/components/driver/include -I/home/cdjk/esp/esp-idf/components/driver/deprecated -I/home/cdjk/esp/esp-idf/components/driver/analog_comparator/include -I/home/cdjk/esp/esp-idf/components/driver/dac/include -I/home/cdjk/esp/esp-idf/components/driver/i2c/include -I/home/cdjk/esp/esp-idf/components/driver/i2s/include -I/home/cdjk/esp/esp-idf/components/driver/ledc/include -I/home/cdjk/esp/esp-idf/components/driver/parlio/include -I/home/cdjk/esp/esp-idf/components/driver/rmt/include -I/home/cdjk/esp/esp-idf/components/driver/sdio_slave/include -I/home/cdjk/esp/esp-idf/components/driver/sdspi/include -I/home/cdjk/esp/esp-idf/components/driver/sigma_delta/include -I/home/cdjk/esp/esp-idf/components/driver/temperature_sensor/include -I/home/cdjk/esp/esp-idf/components/driver/touch_sensor/include -I/home/cdjk/esp/esp-idf/components/driver/twai/include -I/home/cdjk/esp/esp-idf/components/driver/uart/include -I/home/cdjk/esp/esp-idf/components/driver/usb_serial_jtag/include -I/home/cdjk/esp/esp-idf/components/driver/touch_sensor/esp32/include -I/home/cdjk/esp/esp-idf/components/esp_pm/include -I/home/cdjk/esp/esp-idf/components/esp_ringbuf/include -I/home/cdjk/esp/esp-idf/components/esp_driver_gpio/include -I/home/cdjk/esp/esp-idf/components/esp_driver_pcnt/include -I/home/cdjk/esp/esp-idf/components/esp_driver_gptimer/include -I/home/cdjk/esp/esp-idf/components/esp_driver_spi/include -I/home/cdjk/esp/esp-idf/components/esp_driver_mcpwm/include -I/home/cdjk/esp/esp-idf/components/esp_driver_sdmmc/include -I/home/cdjk/esp/esp-idf/components/sdmmc/include

C_FLAGS = -mlongcalls -Wno-frame-address  -fdiagnostics-color=always -ffunction-sections -fdata-sections -Wall -Werror=all -Wno-error=unused-function -Wno-error=unused-variable -Wno-error=unused-but-set-variable -Wno-error=deprecated-declarations -Wextra -Wno-unused-parameter -Wno-sign-compare -Wno-enum-conversion -gdwarf-4 -ggdb -Og -fno-shrink-wrap -fmacro-prefix-map=/home/cdjk/github/esp-items/esp32-80211-tx=. -fmacro-prefix-map=/home/cdjk/esp/esp-idf=/IDF -fstrict-volatile-bitfields -fno-jump-tables -fno-tree-switch-conversion -std=gnu17 -Wno-old-style-declaration

