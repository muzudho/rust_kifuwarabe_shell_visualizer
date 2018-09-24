/// ```
/// ### 以下のコマンドで実行。
/// cd C:\MuzudhoDrive\projects_rust\rust_kifuwarabe_shell_visualizer
/// cargo run --release
/// ```
extern crate kifuwarabe_shell;
use kifuwarabe_shell::graph::*;

const GRAPH_JSON_FILE: &str = "graph.json";

// 任意のオブジェクト。
pub struct ShellVar {}

pub fn make_indent(len: &mut i32, s: &mut String) {
    *len = *len + 4;
    s.clear();
    for _i in 0..*len {
        s.push_str(" ");
    }
}
pub fn unmake_indent(len: &mut i32, s: &mut String) {
    *len = *len - 4;
    s.clear();
    for _i in 0..*len {
        s.push_str(" ");
    }
}

pub fn make_exits_map(graph: &Graph<ShellVar>, indent_len: &mut i32, s: &mut String, node_label: &str) {
    make_indent(indent_len, s);
    let node = graph.get_node(node_label);
    for (exits_label, exits_vec) in node.get_exits_map().iter() {
        println!("{}| {}", s, exits_label);
        for exits_item in exits_vec.iter() {
            println!("{}+---- {}", s, exits_item);
        }
    }

    unmake_indent(indent_len, s);
}

fn main() {
    println!("Hello, world!");

    // グラフの作成。
    let mut graph: Graph<ShellVar> = Graph::new();

    // ファイルからグラフのノード構成を読取。
    graph.read_graph_file(GRAPH_JSON_FILE.to_string());

    // 内容確認出力。
    {
        let mut s = "".to_string();
        let mut indent_len = 0;

        println!("entrance");
        make_indent(&mut indent_len, &mut s);
        for node_label in graph.get_entrance_vec().iter() {
            println!("{}|", s);
            println!("{}+-- {}", s, node_label);

            make_exits_map(&graph, &mut indent_len, &mut s, node_label);
        }

        /*
        println!("nodes");
        for (node_label, node) in graph.get_node_map().iter() {
            println!("  - {} {}", node_label, node.get_token());
            for (exits_label, exits_vec) in node.get_exits_map().iter() {
                println!("    - {}", exits_label);
                for exits_item in exits_vec.iter() {
                    println!("      - {}", exits_item);
                }
            }
        }
        */
    }
}
