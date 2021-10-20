// $ gcc -S main.c -o a.asm
// 

// void wait_while_0(int *p) {
void wait_while_0(volatile int *p) {
    while (*p == 0) {}
}
