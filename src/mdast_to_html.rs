use markdown::mdast;

fn wrap(val: &mut String, elm: &str, attrs: &[(&str, &str)]) {
    let attrs = attrs
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join(" ");
    *val = format!("<{elm} {attrs}>{val}</{elm}>");
}

fn warn(node: &mdast::Node) {
    eprintln!("Warning: Unhandled node: {node:?}");
}

fn escape(text: &str) -> String {
    text.replace("<", "&lt;").replace(">", "&gt;")
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
                output.push_str(&escape(&raw[cursor..child_pos.start.offset]));
            }
            cursor = child_pos.end.offset;
            let out = process(raw, child);
            output.push_str(&out);
        }
    }
    if cursor < pos.end.offset {
        output.push_str(&escape(&raw[cursor..pos.end.offset]));
    }

    match node {
        mdast::Node::Root(_) => (),

        mdast::Node::Blockquote(_) => wrap(&mut output, "blockquote", &[]),
        mdast::Node::FootnoteDefinition(_) => warn(node),
        mdast::Node::MdxJsxFlowElement(_) => warn(node),
        mdast::Node::List(data) => match data.ordered {
            true => wrap(&mut output, "ol", &[]),
            false => wrap(&mut output, "ul", &[]),
        },

        mdast::Node::MdxjsEsm(_) => warn(node),
        mdast::Node::Toml(_) => warn(node),
        mdast::Node::Yaml(_) => warn(node),

        mdast::Node::Break(_) => (),
        mdast::Node::InlineCode(_) => wrap(&mut output, "kbd", &[]),
        mdast::Node::InlineMath(_) => warn(node),
        mdast::Node::Delete(_) => wrap(&mut output, "del", &[]),
        mdast::Node::Emphasis(_) => wrap(&mut output, "em", &[]),
        mdast::Node::MdxTextExpression(_) => warn(node),
        mdast::Node::FootnoteReference(_) => warn(node),
        mdast::Node::Html(_) => warn(node),
        mdast::Node::Image(_) => warn(node),
        mdast::Node::ImageReference(_) => warn(node),
        mdast::Node::MdxJsxTextElement(_) => warn(node),
        mdast::Node::Link(data) => wrap(&mut output, "a", &[("href", &data.url)]),
        mdast::Node::LinkReference(_) => warn(node),
        mdast::Node::Strong(_) => wrap(&mut output, "strong", &[]),
        mdast::Node::Text(_) => (),

        mdast::Node::Code(_) => wrap(&mut output, "code", &[]),
        mdast::Node::Math(_) => warn(node),
        mdast::Node::MdxFlowExpression(_) => warn(node),
        mdast::Node::Heading(data) => wrap(&mut output, &format!("h{}", data.depth), &[]),
        mdast::Node::Table(_) => warn(node),
        mdast::Node::ThematicBreak(_) => warn(node),

        mdast::Node::TableRow(_) => warn(node),
        mdast::Node::TableCell(_) => warn(node),

        mdast::Node::ListItem(_) => wrap(&mut output, "li", &[]),

        mdast::Node::Definition(_) => warn(node),
        mdast::Node::Paragraph(_) => wrap(&mut output, "p", &[]),
    }

    output
}
