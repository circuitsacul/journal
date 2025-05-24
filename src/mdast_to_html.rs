use markdown::mdast;

fn wrap(val: &mut String, elm: &str, class: &str) {
    *val = format!("<{elm} class=\"{class}\">{val}</{elm}>");
}

pub fn render(raw: &str, markdown: &mdast::Node) -> String {
    process(raw, markdown)
}

fn process(raw: &str, node: &mdast::Node) -> String {
    let mut output = String::new();

    let pos = node.position().unwrap();

    let mut cursor = pos.start.offset;
    if let Some(children) = node.children() {
        for child in children {
            let child_pos = child.position().unwrap();
            if child_pos.start.offset > cursor {
                output.push_str(&raw[cursor..child_pos.start.offset]);
            }
            cursor = child_pos.end.offset;
            let out = process(raw, child);
            output.push_str(&out);
        }
    }
    if cursor < pos.end.offset {
        output.push_str(&raw[cursor..pos.end.offset]);
    }

    match node {
        mdast::Node::Blockquote(_) => wrap(&mut output, "blockquote", ""),
        mdast::Node::Heading(data) => {
            let depth = data.depth;
            output = format!("<h{depth}>{output}</h{depth}>");
        }
        mdast::Node::Paragraph(_) => wrap(&mut output, "p", ""),
        mdast::Node::Emphasis(_) => output = format!("<em>{output}</em>"),
        mdast::Node::Strong(_) => output = format!("<strong>{output}</strong>"),
        mdast::Node::Code(_) => output = format!("<code>{output}</code>"),
        mdast::Node::InlineCode(_) => wrap(&mut output, "kbd", ""),
        mdast::Node::Delete(_) => output = format!("<del>{output}</del>"),
        _ => (),
    }

    output
}
