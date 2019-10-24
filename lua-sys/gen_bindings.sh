# Generates the bindings using bindgen
# Pass the input header as the first parameter
# and the output file as the second one.

bindgen --ctypes-prefix libc --whitelist-function ^lua.* $1 -o $2