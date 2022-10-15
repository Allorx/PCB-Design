// Aleksas Girenas 15/10/2022
// For controlling OrionsHands (a fully custom keyboard)

#include "pico/stdlib.h"

// keyboard rotary encoder inputs
#define CLK 0
#define DT 1

int main() {
    // keyboard columns starting from COL_0, COL_1, ...
    uint cols[] = {13,14,15,12,11,10,9,8,2,3,4,5,6,7};
    // keyboard rows starting from ROW_0, ROW_2,...
    uint rows[] = {20,19,18,17,16};
    // init gpio pins and set directions
    for(int i = 0; i < 14; i++){
        gpio_init(cols[i]);
        gpio_set_dir(cols[i], true);
    }
    for(int i = 0; i < 5; i++){
        gpio_init(rows[i]);
    }

    while (true) {
        for(int i = 0; i < 14; i++){
            // send signal from pin i
            gpio_put(cols[i], 1);
            // read row pins

            // turn off signal from pin i
            gpio_put(cols[i], 0);
        }
    }
}