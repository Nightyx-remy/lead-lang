@cInclude("stdio.h");

fn main(): i32 {
    @call("c") printf("Hello world!\n");
}