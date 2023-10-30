use rustyscript::deno_core::extension;

extension!(
    init_handle_response,
    esm_entry_point = "ext:init_handle_response/init_handle_response.js",
    esm = [ dir "src/ext", "init_handle_response.js" ],
);
