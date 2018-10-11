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

uint8_t hw_pin_mode_map[HW_GPIO_PIN_MODE_MAX] = 
{
    0x00, 
    0x01,
    0x02,
    0x03
};

uint8_t hw_pin_speed_map[HW_GPIO_PIN_SPEED_MAX] = 
{
    0x00, 
    0x01,
    0x02,
    0x03
};

uint8_t hw_pin_pupd_map[HW_GPIO_PIN_PUPD_MAX] = 
{
    0x00, 
    0x01,
    0x02,
    0x03
};

uint16_t hw_pin_alternate_function_low_map[HW_GPIO_PIN_ALTERNATE_FUNCTION_LOW_MAX] =
{
    0x00, 
    0x01,
    0x02,
    0x04,
    0x05,
    0x06,
    0x07,
    0x08, 
    0x09,
    0x10,
    0x11,
    0x12,
    0x13,
    0x14,
    0x15,
};

uint16_t hw_pin_alternate_function_high_map[HW_GPIO_PIN_ALTERNATE_FUNCTION_HIGH_MAX] =
{
    0x00, 
    0x01,
    0x02,
    0x04,
    0x05,
    0x06,
    0x07,
    0x08, 
    0x09,
    0x10,
    0x11,
    0x12,
    0x13,
    0x14,
    0x15,
};

static bool hw_gpio_is_valid(hw_gpio_ports_t port, hw_gpio_pins_t pin)
{
    bool ret = false;

    if((port < HW_GPIO_PORT_MAX) && (pin < HW_GPIO_PIN_MAX))
    {
        if(hw_port_map[port])
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

void hw_gpio_pin_set_mode(hw_gpio_ports_t port, hw_gpio_pins_t pin, hw_gpio_pin_modes_t mode)
{
	if(hw_gpio_is_valid(port,pin))
	{	
		hw_port_map[port]->MODER &= ~(0x03 << (pin*2));
		hw_port_map[port]->MODER |= hw_pin_mode_map[mode] << (pin*2);
	}
}

void hw_gpio_pin_set_speed(hw_gpio_ports_t port, hw_gpio_pins_t pin, hw_gpio_pin_speed_t speed)
{
	if(hw_gpio_is_valid(port,pin))
	{
		hw_port_map[port]->OSPEEDR &= ~(0x03 << (pin*2));
		hw_port_map[port]->OSPEEDR |= hw_pin_speed_map[speed] << (pin*2);
	
	}
}

void hw_gpio_pin_set_topology(hw_gpio_ports_t port, hw_gpio_pins_t pin, hw_gpio_pin_topology_t topology)
{
	if(hw_gpio_is_valid(port,pin))
	{
		if(topology == HW_GPIO_PIN_TOPOLOGY_PUSH_PULL)
			hw_port_map[port]->PUPDR &= ~(1 << pin);
		else
			hw_port_map[port]->PUPDR |= (1 << pin);
	}
}

void hw_gpio_pin_set_pupd(hw_gpio_ports_t port, hw_gpio_pins_t pin,hw_gpio_pin_pupd_t pupd)
{
	if(hw_gpio_is_valid(port,pin))
	{
        hw_port_map[port]->OTYPER &= ~(0x03 << (pin*2));
        hw_port_map[port]->OTYPER |= hw_pin_pupd_map[pupd] << (pin*2);
	}
}

static void hw_gpio_pin_set_alternate_function_low(hw_gpio_ports_t port, hw_gpio_pins_t pin,hw_gpio_pin_alternate_function_t pin_alternate_function)
{
	if(hw_gpio_is_valid(port,pin))
	{
	hw_port_map[port]->AFRL &= ~(0x15 << (pin*4));
	hw_port_map[port]->AFRL|= hw_pin_speed_map[pin_alternate_function_low] << (pin*4);
	}
}

static void hw_gpio_pin_set_alternate_function_high(hw_gpio_ports_t port, hw_gpio_pins_t pin,hw_gpio_pin_alternate_function_t pin_alternate_function)
{
	if(hw_gpio_is_valid(port,pin))
	{
	hw_port_map[port]->AFRH &= ~(0x15 << (pin*4));
	hw_port_map[port]->AFRH|= hw_pin_speed_map[pin_alternate_function_high] << (pin*4);
	}
}

void hw_gpio_pin_set_alternate_function(hw_gpio_ports_t port, hw_gpio_pins_t pin, hw_gpio_pin_alternate_function_t pin_alternate_function)
{
    if(hw_gpio_is_valid(port,pin))
    {
        if(pin > HW_GPIO_PIN_AF7)
            hw_gpio_pin_set_alternate_function_high(port,pin,pin_alternate_function);
        else
            hw_gpio_pin_set_alternate_function_low(port,pin,pin_alternate_function);
    }
}
