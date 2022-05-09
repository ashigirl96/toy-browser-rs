pub mod prelude;

use super::*;
use druid::widget::{Container, Flex, Label, LineBreaking};
use druid::{
    AppLauncher, Color, FontDescriptor, FontFamily, FontWeight, Widget, WidgetExt, WindowDesc,
};
use prelude::Browser;

const TEXT_COLOR: Color = Color::rgb8(0x00, 0x00, 0x00);

const H1_FONT: FontDescriptor = FontDescriptor::new(FontFamily::SYSTEM_UI)
    .with_weight(FontWeight::BOLD)
    .with_size(48.0);

impl Browser {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub fn run(self) {
        eprintln!("self.url = {:#?}", self.url);
        let app = WindowDesc::new(build_ui()).window_size((700.0, 400.0));
        AppLauncher::with_window(app).launch(()).expect("error");
    }
}

fn fetch() -> String {
    let html = r#"
<!doctype html>
<html>
<head>
    <title>Example Domain</title>

    <meta charset="utf-8" />
    <meta http-equiv="Content-type" content="text/html; charset=utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <style type="text/css">
    body {
        background-color: #f0f0f2;
        margin: 0;
        padding: 0;
        font-family: -apple-system, system-ui, BlinkMacSystemFont, "Segoe UI", "Open Sans", "Helvetica Neue", Helvetica, Arial, sans-serif;

    }
    div {
        width: 600px;
        margin: 5em auto;
        padding: 2em;
        background-color: #fdfdff;
        border-radius: 0.5em;
        box-shadow: 2px 3px 7px 2px rgba(0,0,0,0.02);
    }
    a:link, a:visited {
        color: #38488f;
        text-decoration: none;
    }
    @media (max-width: 700px) {
        div {
            margin: 0 auto;
            width: auto;
        }
    }
    </style>
</head>

<body>
<div>
    <h1>Example Domain</h1>
    <p>This domain is for use in illustrative examples in documents. You may use this
    domain in literature without prior coordination or asking for permission.</p>
    <p><a href="https://www.iana.org/domains/example">More information...</a></p>
</div>
</body>
</html>
"#;
    html.to_string()
}

fn build_ui() -> impl Widget<()> {
    let html = fetch();
    let dom = DocumentObjectParser::new(html.as_str()).parse();
    let style = dom.extract_style();
    let css = StyleSheetParser::new(&style).parse();
    let render_object = RenderObject::build(dom, css).unwrap();
    build_layout(&render_object).fix_height(1000.0)
}

fn build_layout(render_object: &RenderObject) -> impl Widget<()> {
    let parent = Flex::column();
    let parent = render_object
        .children
        .iter()
        .map(|child_object| (child_object, build_layout(child_object)))
        .fold(parent, |parent, (child_object, child)| {
            parent.with_child(to_child(box child, render_object, child_object))
        });
    let parent = with_margin(box parent, render_object);
    let parent = with_color(box parent, render_object);
    with_fixed_width(box parent, render_object)
}

fn with_color(parent: Box<dyn Widget<()>>, render_object: &RenderObject) -> impl Widget<()> {
    use super::Color as CssColor;
    let bg_color = match render_object.value(&DeclarationProperty::BackgroundColor) {
        Some(DeclarationValue::Color(CssColor { r, g, b, .. })) => {
            Some(Color::rgb8(*r as u8, *g as u8, *b as u8))
        }
        _ => None,
    };
    if let Some(bg_color_) = bg_color {
        parent.background(bg_color_)
    } else {
        // TODO: impl better
        Container::new(parent)
    }
}

// TODO: impl better
fn with_margin(parent: Box<dyn Widget<()>>, render_object: &RenderObject) -> impl Widget<()> {
    let margin_left = render_object.get_length(&DeclarationProperty::PaddingLeft);
    let margin_top = render_object.get_length(&DeclarationProperty::PaddingTop);
    let margin_right = render_object.get_length(&DeclarationProperty::PaddingRight);
    let margin_bottom = render_object.get_length(&DeclarationProperty::PaddingBottom);
    parent.padding((margin_left, margin_top, margin_right, margin_bottom))
}

fn with_fixed_width(parent: Box<dyn Widget<()>>, render_object: &RenderObject) -> impl Widget<()> {
    let parent: Box<dyn Widget<()>> = if let Some(width) = render_object.get_width() {
        box parent.fix_width(width)
    } else {
        box parent
    };
    parent
}

fn to_child(
    child: Box<dyn Widget<()>>,
    parent_object: &RenderObject,
    child_object: &RenderObject,
) -> impl Widget<()> {
    let padding_left = child_object.get_length(&DeclarationProperty::MarginLeft);
    let padding_top = child_object.get_length(&DeclarationProperty::MarginTop);
    let padding_right = child_object.get_length(&DeclarationProperty::MarginRight);
    let padding_bottom = child_object.get_length(&DeclarationProperty::MarginBottom);
    (match &child_object.node {
        Node::Text(s) => match &parent_object.node {
            Node::Element(ref elem) => match elem.tag_name {
                ElementTagName::H1 => Label::new(s.to_string())
                    .with_font(H1_FONT)
                    .with_text_size(24.0)
                    .with_text_color(TEXT_COLOR)
                    .padding((0.0, 8.0))
                    .align_left(),
                ElementTagName::A => Label::new(s.to_string())
                    .with_text_color(Color::rgb8(0x00, 0x00, 0xff))
                    .padding((0.0, 12.0))
                    .align_left(),
                ElementTagName::P => Label::new(s.to_string())
                    .with_text_color(TEXT_COLOR)
                    .with_line_break_mode(LineBreaking::WordWrap)
                    .padding((0.0, 12.0))
                    .align_left(),
                _ => Label::new(s.to_string())
                    .with_text_color(TEXT_COLOR)
                    .with_line_break_mode(LineBreaking::WordWrap)
                    .align_left(),
            },
            _ => child.align_left(),
        },
        Node::Element(ref elem) => match elem.tag_name {
            ElementTagName::Div => child.center(),
            ElementTagName::Body => child.fix_height(1000.0).center(),
            ElementTagName::Html
            | ElementTagName::Main
            | ElementTagName::Article
            | ElementTagName::P
            | ElementTagName::H1
            | ElementTagName::H2
            | ElementTagName::H3
            | ElementTagName::A => child.align_left(),
            _ => Flex::column().align_left(),
        },
        _ => Flex::column().align_left(),
    })
    .padding((padding_left, padding_top, padding_right, padding_bottom))
}
