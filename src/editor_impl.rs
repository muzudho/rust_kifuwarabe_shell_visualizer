use *;
use kifuwarabe_shell::graph::*;


pub fn do_edit(
    _shell_var: &mut ShellVar,
    _request: &dyn Request,
    _response: &mut dyn Response,
){
    println!("Edit.");

}

pub fn do_reload(
    _shell_var: &mut ShellVar,
    _req: &Request,
    res: &mut dyn Response,
) {
    println!("Reload.");
    res.set_reloads(GRAPH_JSON_FILE);
}
