#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

/*
 * An opaque pointer to an `OpaqueWorld` instance.
 * Used for C type safety.
 */
typedef struct world_pointer_t world_pointer_t;

const char *goodbye_world(void);

world_pointer_t *make_world(void);

void print_world(world_pointer_t *ptr);
