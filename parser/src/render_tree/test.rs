#[cfg(test)]
mod tests {
    use crate::prelude::DeclarationProperty::{MarginLeft, PaddingBottom, PaddingRight};
    use crate::prelude::Length;
    use crate::prelude::Unit::{Em, Px};
    use crate::{DocumentObjectParser, RenderObject, StyleSheetParser};

    #[test]
    fn test_new() {
        let html = r#"
<html>
<head>
    <title>Example Domain</title>

    <meta charset="utf-8" />
    <meta http-equiv="Content-type" content="text/html; charset=utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <script>console.log("Hello")</script>
    <style type="text/css">
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
        let css = r#"
body {
    background-color: #f0f0f2;
    margin: 0;
    padding: 0;
}
div {
    width: 600px;
    margin: 5em auto;
    padding: 2em;
    background-color: #fdfdff;
    border-radius: 0.5em;
}
"#;

        let dom = DocumentObjectParser::new(html).parse();
        let css = StyleSheetParser::new(css).parse();
        let render_tree = RenderObject::new(&dom, &css);

        println!("{}", render_tree);
    }

    #[test]
    fn test_lookup_length() {
        let dom = DocumentObjectParser::new(r#"<div />"#).parse();
        let css = StyleSheetParser::new(r#"div { margin: 10em; padding-right: 3px; }"#).parse();
        let tree = RenderObject::new(&dom, &css);

        let tests = vec![
            (MarginLeft, Some(Length::Actual(10.0, Em))),
            (PaddingRight, Some(Length::Actual(3.0, Px))),
            (PaddingBottom, None),
        ];
        for (prop, length) in tests {
            assert_eq!(tree.lookup_length(&prop), length);
        }
    }
}
