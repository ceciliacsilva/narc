#include <stdint.h>
#include <stdbool.h>
#include "hw_api.h"
#include "stm32l052xx.h"

TIM_HandleTypeDef *hw_gpt_map[HW_GPT_TIM_MAX] = 
{
    NULL,
    TIM2,
    NULL,
    NULL,
    NULL,
    TIM6,
    NULL,
    NULL,
    NULL,
    NULL,
    NULL,
    NULL,
    NULL,
    NULL,
    NULL,
    NULL,
    NULL,
    NULL,
    NULL,
    NULL,
    TIM21,
    TIM22,
};

bool hw_gpt_configure(hw_gpt_timers_t timer, hw_gpt_channels_t channel, hw_gpio_ports_t port, hw_gpio_pins_t pin, uint32_t frequency)
{
    // validar a entrada
    // setar o gpio para AF
    // configurar o canal (dependente do timer)
    // setup dos registros de calculo de frequencia
}

void hw_gpt_set_duty_cycle(uint8_t perc)
{

}

/*


uint8_t hw_pwm_calc_presc_period(uint32_t freq, uint16_t *presc, uint16_t *period)
{

    uint32_t tmr_freq = HAL_RCC_GetPCLK2Freq(); // 32000000 
    uint32_t base_freq = tmr_freq;
    uint16_t _pre = 0;
    uint32_t _per = 0;

    if(freq < 1 || freq > tmr_freq)
        return 0;

    do
    {
        _per = base_freq / freq;
        if(_per > 0xFFFF)
        {
            _pre += 1;
            base_freq = tmr_freq / (_pre + 1);
        }
    }
    while(_per > 0xFFFF);

    *presc = _pre;
    *period = _per;

    return 1;
}

uint8_t hw_pwm_set_base_freq(uint32_t freq)
{
    uint16_t presc, period;
    float duty_az, duty_ze;

//    freq = freq >> 1;
    if(hw_pwm_calc_presc_period(freq, &presc, &period) == 0)
        return  0;

    // stop motors using 0 duty cycle
    TIM_PWM_PTR->Instance->CCR1 = 0;
    TIM_PWM_PTR->Instance->CCR2 = 0;

    // reconfigure
    TIM_PWM.Init.Prescaler = presc;
    TIM_PWM.Init.Period = period;
    HAL_TIM_Base_Init(TIM_PWM_PTR);

    hw_pwm_on(HW_MOTOR_ID_AZIMUTH);
    hw_pwm_on(HW_MOTOR_ID_ZENITH);

    return 1;
}

// duty cycle [0.0,1.0]
uint8_t hw_pwm_set_duty_cycle(hw_motor_id_t m_id, float duty_cycle_per)
{
    uint16_t ccr;

    if(duty_cycle_per < 0.0 || duty_cycle_per > 1.0)
        return 0;

    if(m_id != HW_MOTOR_ID_AZIMUTH && m_id != HW_MOTOR_ID_ZENITH)
        return 0;

    // block motor state machine from running
    hw_motor_sm_slope_set(false);

    if(m_id == HW_MOTOR_ID_AZIMUTH)
    {
        ccr = duty_cycle_per * (TIM_PWM_PTR->Init.Period  + 1) - 1;

//        if(ccr != TIM_PWM_PTR->Instance->CCR1)
//        	TIM_PWM_PTR->Instance->CCR1 = ccr;
    }
    else
    {
        ccr = duty_cycle_per * (TIM_PWM_PTR->Init.Period  + 1) - 1;

//        if(ccr != TIM_PWM_PTR->Instance->CCR2)
//        	TIM_PWM_PTR->Instance->CCR2 = ccr;
    }

    motors[m_id].duty_cycle = ccr;
    motors[m_id].state = MOTOR_ON;

    hw_motor_sm_slope_set(true);

    hw_pwm_on(m_id);

    return 1;
}


void hw_pwm_on(hw_motor_id_t m_id)
{
    if(m_id == HW_MOTOR_ID_AZIMUTH)
        HAL_TIM_PWM_Start(TIM_PWM_PTR, TIM_CHANNEL_1);
    else if(m_id == HW_MOTOR_ID_ZENITH)
        HAL_TIM_PWM_Start(TIM_PWM_PTR, TIM_CHANNEL_2);
}

void hw_pwm_off(hw_motor_id_t m_id)
{
    if(m_id == HW_MOTOR_ID_AZIMUTH)
        HAL_TIM_PWM_Stop(TIM_PWM_PTR, TIM_CHANNEL_1);
    else if(m_id == HW_MOTOR_ID_ZENITH)
        HAL_TIM_PWM_Stop(TIM_PWM_PTR, TIM_CHANNEL_2);
}

*/