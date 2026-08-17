use crate::{parser::make_url_from_fragment, CORE_TELEGRAM_URL};
use html2md::{common::get_tag_attr, Handle, StructuredPrinter, TagHandler, TagHandlerFactory};
use std::collections::HashMap;

pub(crate) enum TagsHandlerFactory {
    Anchor,
    Image,
}

impl TagsHandlerFactory {
    pub(crate) fn new_in_map() -> HashMap<String, Box<dyn TagHandlerFactory>> {
        let mut map = HashMap::new();
        map.insert("a".to_string(), Box::new(TagsHandlerFactory::Anchor) as _);
        map.insert("img".to_string(), Box::new(TagsHandlerFactory::Image) as _);
        map
    }
}

impl TagHandlerFactory for TagsHandlerFactory {
    fn instantiate(&self) -> Box<dyn TagHandler> {
        match self {
            TagsHandlerFactory::Anchor => Box::<AnchorHandler>::default(),
            TagsHandlerFactory::Image => Box::new(ImageHandler),
        }
    }
}

#[derive(Default)]
struct AnchorHandler {
    inner: Option<(usize, String)>,
}

impl TagHandler for AnchorHandler {
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        self.inner = get_tag_attr(tag, "href")
            .map(|value| {
                if value.starts_with('#') {
                    make_url_from_fragment(value)
                } else if value.starts_with('/') {
                    [CORE_TELEGRAM_URL, &value].concat()
                } else {
                    value
                }
            })
            .map(|value| (printer.data.len(), value))
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        let (pos, value) = self.inner.as_ref().unwrap();
        if *pos != printer.data.len() {
            printer.insert_str(*pos, "[");
            printer.append_str(&format!("]({})", value));
        }
    }
}

struct ImageHandler;

impl TagHandler for ImageHandler {
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        let alt = get_tag_attr(tag, "alt");

        if let Some(alt) = alt {
            printer.append_str(&alt);
        } else {
            html2md::images::ImgHandler::default().handle(tag, printer)
        }
    }

    fn after_handle(&mut self, _printer: &mut StructuredPrinter) {}
}
