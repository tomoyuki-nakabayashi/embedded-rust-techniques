#include <stdio.h>
#include <rustlib.h>

FILE* stdout_as_ptr_mut() {
	return stdout;
}

int main(void) {
	rust_main();
}
