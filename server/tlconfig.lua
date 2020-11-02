return {
    src_dir = "lua",
    include_dir = {
        "lua","lua/engine","lua/types"
    },
    build_dir = "lua_build",
    preload_modules = {
        "lua/types/Rust",
        "lua/types/rune",
        "lua/types/Card",
     }
  
}