#ifndef __HW_GPIO__
#define __HW_GPIO__

#ifdef __cplusplus
extern "C" {
#endif

typedef enum hw_gpio_ports_e
{
    HW_GPIO_PORTA = 0,
    HW_GPIO_PORTB,
    HW_GPIO_PORTC,
    HW_GPIO_PORTD,
    HW_GPIO_PORTE,
    HW_GPIO_PORTF,
    HW_GPIO_PORTG,
    HW_GPIO_PORTH,
    HW_GPIO_PORTI,
    HW_GPIO_PORT_MAX
} hw_gpio_ports_t;

typedef enum hw_gpio_pins_e
{
    HW_GPIO_PIN0  = 0,
    HW_GPIO_PIN1,
    HW_GPIO_PIN2,
    HW_GPIO_PIN3,
    HW_GPIO_PIN4,
    HW_GPIO_PIN5,
    HW_GPIO_PIN6,
    HW_GPIO_PIN7,
    HW_GPIO_PIN8,
    HW_GPIO_PIN9,
    HW_GPIO_PIN10,
    HW_GPIO_PIN11,
    HW_GPIO_PIN12,
    HW_GPIO_PIN13,
    HW_GPIO_PIN14,
    HW_GPIO_PIN15,
    HW_GPIO_PIN_MAX
} hw_gpio_pins_t;

typedef enum hw_gpio_pin_values_e
{
    HW_GPIO_PIN_RESET = 0,
    HW_GPIO_PIN_SET = 1,
    HW_GPIO_PIN_UNDEF = 2,
} hw_gpio_pin_values_t;

void hw_gpio_pin_set(hw_gpio_ports_t port, hw_gpio_pins_t pin);
void hw_gpio_pin_reset(hw_gpio_ports_t port, hw_gpio_pins_t pin);
hw_gpio_pin_values_t hw_gpio_pin_read(hw_gpio_ports_t port, hw_gpio_pins_t pin);
void hw_gpio_pin_toggle(hw_gpio_ports_t port, hw_gpio_pins_t pin);

#ifdef __cplusplus
}
#endif

#endif /* __HW_GPIO__ */
