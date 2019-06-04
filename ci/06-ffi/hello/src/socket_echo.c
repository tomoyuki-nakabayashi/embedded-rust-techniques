/*
 * Copyright (c) 2017 Linaro Limited
 *
 * SPDX-License-Identifier: Apache-2.0
 */

#include <stdio.h>
#include <bindings.h>

FILE* stdout_as_ptr_mut() {
	return stdout;
}

int main(void)
{
	rust_main();
}
