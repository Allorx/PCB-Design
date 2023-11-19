// Aleksas Girenas 15/10/2022
// OrionsHands Firmware for rp pico w

#include "hardware/gpio.h"
#include "pico/stdlib.h"
#include "bt_hid.c"
#include "pico/cyw43_arch.h"
#include "pico/multicore.h"

/*
// keyboard rotary encoder inputs
#define CLK 0
#define DT 1
*/

// for debounce
#define CONFIRMED_PRESS 5
// bluetooth reporting rate
#define REPORT_DELAY_MS 1000

// ?keymaps
const uint8_t keymap[5][14] =
{
    {0x29,0x1E,0x1F,0x20,0x21,0x22,0x23,0x24,0x25,0x26,0x27,0x2D,0x2E,0x2A},
    {0x2B,0x14,0x1A,0x08,0X15,0X17,0X1C,0X18,0X0C,0X12,0X13,0X2F,0X30,0X48},
    {0x39,0X04,0X16,0X07,0X09,0X0A,0X0B,0X0D,0X0E,0X0F,0X33,0X34,0X28,0X4C},
    {0xE1,0X31,0X1D,0X1B,0X06,0X19,0X05,0X11,0X10,0X36,0X37,0X38,0X32,0X52},
    {0xE0,0XE3,0XE2,0X00,0X00,0X00,0X2C,0X00,0X00,0XE6,0x00,0X50,0X51,0X4F}
};
const uint8_t fn_keymap[5][14] =
{
    {0x00,0X3A,0X3B,0X3C,0X3D,0X3E,0X3F,0X40,0X41,0X42,0X43,0X44,0X45,0x00},
    {0x2B,0x14,0x1A,0x08,0X15,0X17,0X1C,0X18,0X0C,0X12,0X13,0X2F,0X30,0X48},
    {0x39,0X04,0X16,0X07,0X09,0X0A,0X0B,0X0D,0X0E,0X0F,0X33,0X34,0X28,0X4C},
    {0xE1,0X31,0X1D,0X1B,0X06,0X19,0X05,0X11,0X10,0X36,0X37,0X38,0X32,0X52},
    {0xE0,0XE3,0XE2,0X00,0X00,0X00,0X2C,0X00,0X00,0XE6,0x00,0X50,0X51,0X4F}
};

// define key outputs i.e. pressed or released
uint8_t pressed_keys[5][14] = {0};
// define key debounce counter array
int8_t debounce_keys[5][14] = {0};
// gpio pin nums of keyboard columns starting from COL_0, COL_1, ...
uint8_t cols[14] = {13,14,15,12,11,10,9,8,2,3,4,5,6,7};
// gpio pin nums of keyboard rows starting from ROW_0, ROW_2,...
uint8_t rows[5] = {20,19,18,17,16};

// ?timers
bool timer_fired = false;
int64_t report_alarm_callback(alarm_id_t id, void *user_data){
    // alarm is completed 
    timer_fired = true;
    return 0;
}

void scan_keys(){
    //? Scan all keys to assign which are pressed/released to pressed_keys
    for(int col = 0; col < 14; col++){
        // send signal from this column
        gpio_set_dir(cols[col], true);
        gpio_put(cols[col], 0);
        for(int row = 0; row < 5; row++){
            // read row row: if it is 0 and the debounce on the key exceeds FLIP, then col,row is pressed
            if(gpio_get(rows[row]) == 0){
                if(debounce_keys[row][col] > CONFIRMED_PRESS){
                    // key has been confirmed to be pressed
                    pressed_keys[row][col] = 1;
                    debounce_keys[row][col] = 0;
                }
                else{
                    // increment debounce state
                    debounce_keys[row][col] ++;
                }
            }
            else {
                if(debounce_keys[row][col] < -CONFIRMED_PRESS){
                    // key has been confirmed to be released
                    pressed_keys[row][col] = 0;
                    debounce_keys[row][col] = 0;
                }
                else{
                    // decrement debounce state
                    debounce_keys[row][col] --;
                }
            }
        }
        // turn off signal from this column
        gpio_set_dir(cols[col], false);
    }
}

void core1_main(){
    //? testing
    while(true){
        cyw43_arch_gpio_put(CYW43_WL_GPIO_LED_PIN, 1);
        sleep_ms(1000);
        cyw43_arch_gpio_put(CYW43_WL_GPIO_LED_PIN, 0);
        sleep_ms(1000);
    }
}

int main()
{
    //? Initialise
    stdio_init_all();
    if (cyw43_arch_init())
    {
        // failed to init cyw43
        return -1;
    }

    bt_init();
    multicore_launch_core1(core1_main);
    
    //? set up report alarm
    add_alarm_in_ms(REPORT_DELAY_MS, report_alarm_callback, NULL, false);

    //? init gpio pins and set directions
    for(int col = 0; col < 14; col++){
        gpio_init(cols[col]);
        gpio_set_dir(cols[col], false);
        gpio_pull_up(cols[col]);
    }
    for(int row = 0; row < 5; row++){
        gpio_init(rows[row]);
        gpio_set_dir(rows[row], false);
        gpio_pull_up(rows[row]);
    }

    while (true)
    {   
        scan_keys();

        if(timer_fired){
            //? queue the chars that are currently pressed and ready the report
            for(int col = 0; col < 14; col++){
                for(int row = 0; row < 5; row++){
                    if(pressed_keys[row][col] == 1){
                        if(pressed_keys[4][10]){
                            queue_character(fn_keymap[row][col]);
                        }
                        else{
                            queue_character(keymap[row][col]);
                        }
                    }
                }
            }
            // queue_character(0x09);
            report_ready();
            //? add a new alarm
            add_alarm_in_ms(REPORT_DELAY_MS, report_alarm_callback, NULL, false);
        }
    }

    return 0;
}

//! scanning column then row is not the most efficient (not looking at memory sequentially)