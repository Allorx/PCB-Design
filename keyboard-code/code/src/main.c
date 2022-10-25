// Aleksas Girenas 15/10/2022
// For controlling OrionsHands (a fully custom keyboard)

#include <stdio.h>
#include "pico/stdlib.h"

// keyboard rotary encoder inputs
#define CLK 0
#define DT 1
// define key presses
#define KEY_PRESSED 0
#define KEY_RELEASED 1
// define debounce FLIP and MAX_DEBOUNCE states in cycles
#define MAX_DEBOUNCE 10
#define FLIP 5
// define FUNC_KEY
#define FUNC_KEY 0xFF

int main() {
    stdio_init_all();

    // keymaps
    const unsigned char keymap[5][14] =
    {
        {0x29,0x1E,0x1F,0x20,0x21,0x22,0x23,0x24,0x25,0x26,0x27,0x2D,0x2E,0x2A},
        {0x2B,0x14,0x1A,0x08,0X15,0X17,0X1C,0X18,0X0C,0X12,0X13,0X2F,0X30,0X48},
        {0x39,0X04,0X16,0X07,0X09,0X0A,0X0B,0X0D,0X0E,0X0F,0X33,0X34,0X28,0X4C},
        {0xE1,0X31,0X1D,0X1B,0X06,0X19,0X05,0X11,0X10,0X36,0X37,0X38,0X32,0X52},
        {0xE0,0XE3,0XE2,0X00,0X00,0X00,0X2C,0X00,0X00,0XE6,FUNC_KEY,0X50,0X51,0X4F}
    };
    const unsigned char fn_keymap[5][14] =
    {
        {0x00,0X3A,0X3B,0X3C,0X3D,0X3E,0X3F,0X40,0X41,0X42,0X43,0X44,0X45,0x00},
        {0x2B,0x14,0x1A,0x08,0X15,0X17,0X1C,0X18,0X0C,0X12,0X13,0X2F,0X30,0X48},
        {0x39,0X04,0X16,0X07,0X09,0X0A,0X0B,0X0D,0X0E,0X0F,0X33,0X34,0X28,0X4C},
        {0xE1,0X31,0X1D,0X1B,0X06,0X19,0X05,0X11,0X10,0X36,0X37,0X38,0X32,0X52},
        {0xE0,0XE3,0XE2,0X00,0X00,0X00,0X2C,0X00,0X00,0XE6,FUNC_KEY,0X50,0X51,0X4F}
    };
    // bool to state whether function key is pressed
    const unsigned char 
    bool fning = false;

    // define key outputs
    uint key_out[14][5] = {KEY_RELEASED};
    // define key debounce counter array and number of checks until a FLIP
    uint key_debounce[14][5] = {0};
    // gpio pin nums of keyboard columns starting from COL_0, COL_1, ...
    uint cols[14] = {13,14,15,12,11,10,9,8,2,3,4,5,6,7};
    // gpio pin nums of keyboard rows starting from ROW_0, ROW_2,...
    uint rows[5] = {20,19,18,17,16};

    // init gpio pins and set directions
    for(int i = 0; i < 14; i++){
        gpio_init(cols[i]);
        gpio_set_dir(cols[i], false);
        gpio_pull_up(cols[i]);
    }
    for(int i = 0; i < 5; i++){
        gpio_init(rows[i]);
        gpio_set_dir(rows[i], false);
        gpio_pull_up(rows[i]);
    }

    while (true) {
        for(int i = 0; i < 14; i++){
            // send signal from column i
            gpio_set_dir(cols[i], true);
            gpio_put(cols[i], 0);

            for(int j = 0; j < 5; j++){
                // read row j: if it is 0 and the debounce on the key exceeds FLIP, then i,j is pressed
                bool read = gpio_get(rows[j]);
                if(read == KEY_PRESSED && key_out[i][j] == KEY_RELEASED){
                    if(key_debounce[i][j] >= FLIP){
                        // key has been confirmed to be pressed
                        // check and set if func key pressed and send to event queue that released all keys
                        // fetch key value at key_out location and send to event queue that it has been pressed
                        if(keymap[j][i] == FUNC_KEY){
                            fning = true;
                            for(int y = 0; y < 14; y++){
                                for(int x = 0; x < 5; x++){
                                    key_out[y][x] = KEY_RELEASED;
                                    printf("%x\n", keymap[x][y]);
                                }
                            }
                        }
                        else if(fning){
                            printf("%x\n", fn_keymap[j][i]);
                        }
                        else{
                            printf("%x\n", keymap[j][i]);
                        }
                        key_out[i][j] = KEY_PRESSED;
                        key_debounce[i][j] = MAX_DEBOUNCE;
                    }
                    else{
                        // increment debounce state
                        key_debounce[i][j] ++;
                    }
                }
                else if(read == KEY_RELEASED && key_out[i][j] == KEY_PRESSED){
                    if(key_debounce[i][j] < FLIP){
                        // key has been confirmed to be released
                        // check and set if func key pressed
                        // fetch key value at key_out location and send to event queue that it has been released
                        if(keymap[j][i] == FUNC_KEY){
                            fning = false;
                        }
                        else if(fning){
                            printf("%x\n", fn_keymap[j][i]);
                        }
                        else{
                            printf("%x\n", keymap[j][i]);
                        }
                        key_out[i][j] = KEY_RELEASED;
                        key_debounce[i][j] = 0;
                    }
                    else{
                        // decrement debounce state
                        key_debounce[i][j] --;
                    }
                }
                else if(key_debounce[i][j] > 0 && key_out[i][j] == KEY_RELEASED){
                    // current read is same as key_out for i,j - return towards 0 debounce counts
                    key_debounce[i][j] --;
                }
                else if(key_debounce[i][j] < MAX_DEBOUNCE && key_out[i][j] == KEY_PRESSED){
                    // current read is same as key_out for i,j - return towards the MAX_DEBOUNCE debounce counts
                    key_debounce[i][j] ++;
                }
            }
            // turn off signal from column i
            gpio_set_dir(cols[i], false);
        }
        // send queued keys to usb

    }
}