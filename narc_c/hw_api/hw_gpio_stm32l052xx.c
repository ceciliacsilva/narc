#include <stdint.h>
#include <stdbool.h>
#include "hw_api.h"
#include "hw_gpio.h"
#include "stm32l052xx.h"

GPIO_TypeDef *hw_port_map[HW_GPIO_PORT_MAX] = 
{
    GPIOA,
    GPIOB,
    GPIOC,
    GPIOD,
    NULL,
    NULL,
    NULL,
    GPIOH,
    NULL
};

uint16_t hw_pin_map[HW_GPIO_PIN_MAX] = 
{
    1 << HW_GPIO_PIN0,
    1 << HW_GPIO_PIN1,
    1 << HW_GPIO_PIN2,
    1 << HW_GPIO_PIN3,
    1 << HW_GPIO_PIN4,
    1 << HW_GPIO_PIN5,
    1 << HW_GPIO_PIN6,
    1 << HW_GPIO_PIN7,
    1 << HW_GPIO_PIN8,
    1 << HW_GPIO_PIN9,
    1 << HW_GPIO_PIN10,
    1 << HW_GPIO_PIN11,
    1 << HW_GPIO_PIN12,
    1 << HW_GPIO_PIN13,
    1 << HW_GPIO_PIN14,
    1 << HW_GPIO_PIN15,
};

static bool hw_gpio_is_valid(hw_gpio_ports_t port, hw_gpio_pins_t pin)
{
    bool ret = false;

    if((port < HW_GPIO_PORT_MAX) && (pin < HW_GPIO_PIN_MAX))
    {
        if(hw_port_map[port]))
            ret = true;
    }

    return ret;
}

void hw_gpio_pin_set(hw_gpio_ports_t port, hw_gpio_pins_t pin)
{
    if(hw_gpio_is_valid(port,pin))
        hw_port_map[port]->BSRR = hw_pin_map[pin];
}

void hw_gpio_pin_reset(hw_gpio_ports_t port, hw_gpio_pins_t pin)
{
    if(hw_gpio_is_valid(port,pin))
        hw_port_map[port]->BRR = hw_pin_map[pin];
}

hw_gpio_pin_values_t hw_gpio_pin_read(hw_gpio_ports_t port, hw_gpio_pins_t pin)
{
    hw_gpio_pin_values_t pin_val = HW_GPIO_PIN_UNDEF;

    if(hw_gpio_is_valid(port,pin))
        pin_val = hw_port_map[port]->IDR & hw_pin_map[pin] ? HW_GPIO_PIN_SET : HW_GPIO_PIN_RESET;

    return pin_val;
}

void hw_gpio_pin_toggle(hw_gpio_ports_t port, hw_gpio_pins_t pin)
{
    if(hw_gpio_is_valid(port,pin))
        hw_port_map[port]->ODR ^= hw_pin_map[pin];
}