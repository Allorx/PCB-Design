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
// define debounce FLIP and MAX states in cycles
#define MAX 1000
#define FLIP 500

int main() {
    stdio_init_all();

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
                        key_out[i][j] = KEY_PRESSED;
                        key_debounce[i][j] = MAX;
                        // fetch key value at key_out location and send to event queue that it has been pressed

                        printf("pressed\n");
                    }
                    else{
                        // increment debounce state
                        key_debounce[i][j] ++;
                    }
                }
                else if(read == KEY_RELEASED && key_out[i][j] == KEY_PRESSED){
                    if(key_debounce[i][j] < FLIP){
                        // key has been confirmed to be released
                        key_out[i][j] = KEY_RELEASED;
                        key_debounce[i][j] = 0;
                        // fetch key value at key_out location and send to event queue that it has been released

                        printf("released\n");
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
                else if(key_debounce[i][j] < MAX && key_out[i][j] == KEY_PRESSED){
                    // current read is same as key_out for i,j - return towards the MAX debounce counts
                    key_debounce[i][j] ++;
                }
            }
            // turn off signal from column i
            gpio_set_dir(cols[i], false);
        }
        // send queud keys to HID
    }
}