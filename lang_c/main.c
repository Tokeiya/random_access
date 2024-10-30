#include <stdint.h>
#include <stdio.h>

struct xorshift64_state {
    uint64_t a;
};

uint64_t xorshift64(struct xorshift64_state *state)
{
    uint64_t x = state->a;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    return state->a = x;
}


int main(void) {
    struct xorshift64_state state={609302558241040659};

    for (int i = 0; i < 20; i++) {
        printf("%llu,\n", xorshift64(&state));
    }

}
