use rustyscript::deno_core::extension;

extension!(
    init_global_this_backup,
    esm_entry_point = "ext:init_global_this_backup/init_global_this_backup.js",
    esm = [ dir "src/controllers/functions/ext", "init_global_this_backup.js" ],
);
