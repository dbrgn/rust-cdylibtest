crate a contains two `no_mangle` functions (`make_world` and `print_world`)

crate b depends on a and re-exports those functions, and adds an additional function (`goodbye_world`)

when checking with `objdump`:

    $ objdump -T a/target/debug/liba.so  | grep world
    00000000000030c0 g    DF .text  0000000000000303  Base        print_world
    0000000000002f20 g    DF .text  000000000000019a  Base        make_world
    $ objdump -T b/target/debug/libb.so  | grep world
    00000000000034a0 g    DF .text  0000000000000086  Base        goodbye_world

the `libb.so` file does not expose those symbols.

creating wrapper functions works kind-of (see crate `b2`), but it's quite ugly.
also, for some reason, `test.c` in `b2` segfaults while `test.c` in `a` works fine.

    $ cd b2
    $ cc test.c -L target/debug -l b2 -o test
    $ LD_LIBRARY_PATH=target/debug ./test
    Test start

    From b2 crate:
    Output: Goodbye, world!

    From a crate:
    Segmentation fault (core dumped)
