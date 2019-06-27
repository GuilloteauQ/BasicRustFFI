#include <stdint.h>
#include <stdio.h>

int32_t add(int32_t, int32_t);

int main() {
    int32_t x = 4;
    int32_t y = 2;
    int32_t z = add(x, y);
    printf("According to Rust, %d + %d = %d\n", x, y, z);
    return 0;
}
