# Generates the bindings using bindgen
# Pass the header paths as parameters.

# Example (windows):
# ./gen_bindings.sh \
#     "C:/Program Files (x86)/Windows Kits/10/Include/10.0.17763.0/ucrt" \
#     "C:/Program Files (x86)/Microsoft Visual Studio/2017/Community/VC/Tools/MSVC/14.15.26726/include" \
#     "D:/Program Files/LLVM/lib/clang/9.0.0/include"

bindgen \
    --ctypes-prefix libc --whitelist-function ^lua.* \
    wrapper.h -o lua_bindings.rs -- \
    -Isrc/embedded \
    "${@/#/-I}"
