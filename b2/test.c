// Compile: cc test.c -L target/debug -l b2 -o test
// Run: LD_LIBRARY_PATH=target/debug ./test
//
#include <stdio.h>
#include "bindings.h"

int main() {
    printf("Test start\n");

    printf("\nFrom b2 crate:\n");
    printf("Output: %s\n", goodbye_world());

    printf("\nFrom a crate:\n");
    world_pointer_t *world = make_world();
    print_world(world);

    printf("Test end\n");
    return 0;
}
