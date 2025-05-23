use markdown::mdast;

pub fn process(raw: &str, node: &mdast::Node) -> String {
    let mut output = String::new();

    let pos = node.position().unwrap();

    let mut cursor = pos.start.offset;
    if let Some(children) = node.children() {
        for child in children {
            let child_pos = child.position().unwrap();
            println!(
                "child{child:?}: {child_pos:?}, {:?}",
                &raw[child_pos.start.offset..child_pos.end.offset]
            );
            if child_pos.start.offset > cursor {
                output.push_str(dbg!(&raw[cursor..child_pos.start.offset]));
            }
            cursor = child_pos.end.offset;
            let out = process(raw, child);
            output.push_str(&out);
        }
    }
    if cursor < pos.end.offset {
        output.push_str(dbg!(&raw[cursor..pos.end.offset]));
    }

    match node {
        mdast::Node::Heading(data) => {
            let depth = data.depth;
            output = format!("<h{depth}>{output}</h{depth}>");
        }
        mdast::Node::Paragraph(_) => output = format!("<p>{output}</p>"),
        mdast::Node::Emphasis(_) => output = format!("<em>{output}</em>"),
        mdast::Node::Strong(_) => output = format!("<strong>{output}</strong>"),
        mdast::Node::Code(_) => output = format!("<code>{output}</code>"),
        mdast::Node::InlineCode(_) => output = format!("<kbd>{output}</kbd>"),
        mdast::Node::Delete(_) => output = format!("<del>{output}</del>"),
        _ => (),
    }

    output
}
