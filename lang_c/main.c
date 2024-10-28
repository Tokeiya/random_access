#include <stdint.h>
#include <stdio.h>

struct xorshift64_state {
    uint64_t a;
};

uint64_t xorshift64(struct xorshift64_state *state)
{
    uint64_t x = state->a;
    x ^= x << 7;
    x ^= x >> 9;
    return state->a = x;
}


int main(void) {
    struct xorshift64_state state={42};

    for (int i = 0; i < 10; i++) {
        printf("%llu\n", xorshift64(&state));
    }

}
