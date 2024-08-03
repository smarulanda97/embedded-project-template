/* vim: set sw=2 expandtab tw=80: */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <led.h> 
#include <timer.h>
#include <humidity.h>
#include <temperature.h>

#include <console.h>

char hello[] = "Application running on top of Tockos!\r\n";

int temperature = 0;
unsigned humidity = 0;

char temperature_text[50];

static void nop(
  int a __attribute__((unused)),
  int b __attribute__((unused)),
  int c __attribute__((unused)),
  void* d __attribute__((unused))
) {}

int main(void) {
  putnstr_async(hello, strlen(hello), nop, NULL);

  gpio_enable_output(25);

  while (1) {
    gpio_set(25);
    delay_ms(1000);

    gpio_clear(25);
    delay_ms(1000);

     if (driver_exists(DRIVER_NUM_TEMPERATURE)) {
      temperature_read_sync(&temperature);
      sprintf(temperature_text, "Temperature: %d C\n", temperature / 100);
      putnstr(temperature_text, strlen(temperature_text));
    }
  }

  return 0;
}