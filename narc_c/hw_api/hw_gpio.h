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

typedef enum hw_gpio_pin_modes_e
{
    HW_GPIO_PIN_INPUT = 0, 
    HW_GPIO_PIN_OUTPUT,
    HW_GPIO_PIN_ALTERNATE_FUNCTION,
    HW_GPIO_PIN_ANALOG,
	HW_GPIO_PIN_MODE_MAX,
} hw_gpio_pin_modes_t;


typedef enum hw_gpio_pin_topology_e
{
	HW_GPIO_PIN_TOPOLOGY_PUSH_PULL = 0,
    HW_GPIO_PIN_TOPOLOGY_OPEN_DRAIN
} hw_gpio_pin_topology_t;

typedef enum hw_gpio_pin_speed_e
{
	HW_GPIO_PIN_SPEED_LOW = 0,
	HW_GPIO_PIN_SPEED_MEDIUM,
	HW_GPIO_PIN_SPEED_FAST,
	HW_GPIO_PIN_SPEED_HIGH,
	HW_GPIO_PIN_SPEED_MAX
} hw_gpio_pin_speed_t;

typedef enum hw_gpio_pin_pupd_e
{
	HW_GPIO_PIN_PUPD_NOPULL = 0,
	HW_GPIO_PIN_PUPD_UP,
	HW_GPIO_PIN_PUPD_DOWN, 
	HW_GPIO_PIN_RESERVED,
	HW_GPIO_PIN_PUPD_MAX
} hw_gpio_pin_pupd_t;

typedef enum hw_gpio_pin_alternate_function_low_e (
{
	HW_GPIO_PIN_AF0 = 0, 
	HW_GPIO_PIN_AF1,
	HW_GPIO_PIN_AF2,
	HW_GPIO_PIN_AF3,
	HW_GPIO_PIN_AF4,
	HW_GPIO_PIN_AF5,
	HW_GPIO_PIN_AF6,
	HW_GPIO_PIN_AF7,
	HW_GPIO_PIN_AF8,
	HW_GPIO_PIN_AF9,
	HW_GPIO_PIN_AF10,
	HW_GPIO_PIN_AF11,
	HW_GPIO_PIN_AF12,
	HW_GPIO_PIN_AF13,
	HW_GPIO_PIN_AF14,
	HW_GPIO_PIN_AF15,
	HW_GPIO_PIN_ALTERNATE_LOW_MAX
} hw_gpio_pin_alternate_function_low_t;


typedef enum hw_gpio_pin_alternate_function_high_e (
{
	HW_GPIO_PIN_AF0 = 0, 
	HW_GPIO_PIN_AF1,
	HW_GPIO_PIN_AF2,
	HW_GPIO_PIN_AF3,
	HW_GPIO_PIN_AF4,
	HW_GPIO_PIN_AF5,
	HW_GPIO_PIN_AF6,
	HW_GPIO_PIN_AF7,
	HW_GPIO_PIN_AF8,
	HW_GPIO_PIN_AF9,
	HW_GPIO_PIN_AF10,
	HW_GPIO_PIN_AF11,
	HW_GPIO_PIN_AF12,
	HW_GPIO_PIN_AF13,
	HW_GPIO_PIN_AF14,
	HW_GPIO_PIN_AF15,
	HW_GPIO_PIN_ALTERNATE_HIGH_MAX
} hw_gpio_pin_alternate_function_high_t;

void hw_gpio_pin_set(hw_gpio_ports_t port, hw_gpio_pins_t pin);
void hw_gpio_pin_reset(hw_gpio_ports_t port, hw_gpio_pins_t pin);
hw_gpio_pin_values_t hw_gpio_pin_read(hw_gpio_ports_t port, hw_gpio_pins_t pin);
void hw_gpio_pin_toggle(hw_gpio_ports_t port, hw_gpio_pins_t pin);
void hw_gpio_pin_set_mode(hw_gpio_ports_t port, hw_gpio_pins_t pin, hw_gpio_pin_modes_t modes)
void hw_gpio_pin_set_speed(hw_gpio_ports_t port, hw_gpio_pins_t pin, hw_gpio_pin_speed_t speed)
void hw_gpio_pin_set_topology(hw_gpio_ports_t port, hw_gpio_pins_t pin,hw_gpio_pin_topology_t topology)
void hw_gpio_pin_set_pupd(hw_gpio_ports_t port, hw_gpio_pins_t pin, hw_gpio_pin_pupd_t pin_pupd)
void hw_gpio_pin_set_alternate_function_low(hw_gpio_ports_t port, hw_gpio_pins_t pin, hw_gpio_pin_alternate_function_low_t pin_alternate_function_low)
void hw_gpio_pin_set_alternate_function_high(hw_gpio_ports_t port, hw_gpio_pins_t pin, hw_gpio_pin_alternate_function_high_t pin_alternate_function_high)

#ifdef __cplusplus
}
#endif

#endif /* __HW_GPIO__ */