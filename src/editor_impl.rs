use kifuwarabe_shell::graph::*;
use *;

/// ノードを追加するのに使う。
pub fn do_edit_add(
    _shell_var: &mut ShellVar,
    _request: &dyn Request,
    _response: &mut dyn Response,
) {
    println!("!Add.");
}

pub fn do_edit_nodelabelvar(
    shell_var: &mut ShellVar,
    request: &dyn Request,
    _response: &mut dyn Response,
) {
    let node_label = &request.get_groups()[0];
    shell_var.node_label = node_label.to_string();
    println!("!Node label var. {}", shell_var.node_label);
}

/// グラフファイルを上書き保存する。
pub fn do_edit_save(
    _shell_var: &mut ShellVar,
    _request: &dyn Request,
    _response: &mut dyn Response,
) {
    println!("!Save. {}", GRAPH_JSON_FILE);
}

pub fn do_reload(_shell_var: &mut ShellVar, _req: &Request, res: &mut dyn Response) {
    println!("Reload.");
    res.set_reloads(GRAPH_JSON_FILE);
}
