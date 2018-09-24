/// ```
/// ### 以下のコマンドで実行。
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_shell_visualizer
/// cargo run --release
/// ```
extern crate kifuwarabe_shell;
use kifuwarabe_shell::graph::*;
use kifuwarabe_shell::shell::*;
use std::collections::HashSet;

mod editor_impl;
use editor_impl::*;


const GRAPH_JSON_FILE: &str = "graph.json";

#[allow(dead_code)]
const BLACK: &str = "\u{001b}[30m";
#[allow(dead_code)]
const RED: &str = "\u{001b}[31m";
#[allow(dead_code)]
const GREEN: &str = "\u{001b}[32m";
#[allow(dead_code)]
const YELLOW: &str = "\u{001b}[33m";
#[allow(dead_code)]
const BLUE: &str = "\u{001b}[34m";
#[allow(dead_code)]
const MAGENTA: &str = "\u{001b}[35m";
#[allow(dead_code)]
const CYAN: &str = "\u{001b}[36m";
#[allow(dead_code)]
const WHITE: &str = "\u{001b}[37m";
#[allow(dead_code)]
const RESET: &str = "\u{001b}[0m";

// 任意のオブジェクト。
pub struct ShellVar {}
impl ShellVar {
    fn new() -> ShellVar {
        ShellVar {}
    }
}

pub fn make_indent(len: &mut i32, s: &mut String) {
    *len += 4;
    s.clear();
    for _i in 0..*len {
        s.push_str(" ");
    }
}
pub fn unmake_indent(len: &mut i32, s: &mut String) {
    *len -= 4;
    s.clear();
    for _i in 0..*len {
        s.push_str(" ");
    }
}

pub fn expand_exits_map(
    graph: &Graph<ShellVar>,
    principal: &mut HashSet<String>,
    indent_len: &mut i32,
    s: &mut String,
    node_label: &str,
    depth: i32,
) {
    if 0 < depth {
        make_indent(indent_len, s);
        let node = graph.get_node(node_label);
        for (exits_label, exits_vec) in node.get_exits_map().iter() {
            println!("{}| {}", s, exits_label);
            for exits_item in exits_vec.iter() {
                let exist_node = graph.get_node(exits_item);
                if principal.contains(exits_item) {
                    println!("{}+----|LOOP| {}    {}{}{}", s, exits_item, GREEN, exist_node.get_token(), RESET);
                // 循環参照を止める。
                } else {
                    principal.insert(exits_item.to_string());
                    println!("{}+-- {}    {}{}{}", s, exits_item, GREEN, exist_node.get_token(), RESET);
                    expand_exits_map(&graph, principal, indent_len, s, exits_item, depth-1);
                }
            }
        }
        unmake_indent(indent_len, s);
    }
}

/// グラフ ファイル読込。
fn read_contents_graph(graph: &mut Graph<ShellVar>) {
    // グラフのノード構成を読取。
    graph.read_graph_file(GRAPH_JSON_FILE.to_string());

    // 内容確認出力。
    {
        // 循環参照防止のため。
        let mut principal: HashSet<String> = HashSet::new();
        // 循環参照防止のため。
        let max_depth = 20;

        let mut s = "".to_string();
        let mut indent_len = 0;

        println!("entrance");
        make_indent(&mut indent_len, &mut s);
        for node_label in graph.get_entrance_vec().iter() {
            let node = graph.get_node(node_label);
            if principal.contains(node_label) {
                println!("{}|", s);
                println!("{}+----|LOOP| {}    {}{}{}", s, node_label, GREEN, node.get_token(), RESET);
            // 循環参照を止める。
            } else {
                principal.insert(node_label.to_string());
                println!("{}|", s);
                println!("{}+-- {}    {}{}{}", s, node_label, GREEN, node.get_token(), RESET);

                expand_exits_map(
                    &graph,
                    &mut principal,
                    &mut indent_len,
                    &mut s,
                    node_label,
                    max_depth,
                );
            }
        }
    }
}

fn main() {
    // グラフの作成。
    let mut graph: Graph<ShellVar> = Graph::new();
    // コントローラーを登録。
    graph.insert_fn("do_edit", do_edit);
    graph.insert_fn("do_reload", do_reload);

    // グラフ ファイル読込。
    read_contents_graph(&mut graph);

    // 任意のオブジェクト。
    let mut shell_var = ShellVar::new();
    // シェルの作成。
    let mut shell = Shell::new();

    // 実行。
    println!("Please enter command.");
    shell.run(&graph, &mut shell_var);
    println!("Finished.");

}
