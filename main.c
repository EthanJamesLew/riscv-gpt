int main() {
    unsigned n = 20;
    unsigned a = 0;
    unsigned b = 1;
    unsigned i;
    unsigned c;

    for (i = 0; i < n; i++) {
        c = a + b;
        a = b;
        b = c;
    }

    return 0;
}

void _start() {
    main();
}

