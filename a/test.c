// Compile: cc test.c -L target/debug -l a -o test
// Run: LD_LIBRARY_PATH=target/debug ./test

#include <stdio.h>
#include "bindings.h"

int main() {
    printf("Test start\n");

    world_pointer_t *world = make_world();
    print_world(world);

    printf("Test end\n");
    return 0;
}
