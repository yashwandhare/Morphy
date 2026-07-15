/// registry.rs - operation registry for dynamic menus and routing

use crate::converters;
use crate::compression;

pub type HandlerFn = fn(&str, Option<&[String]>);

pub struct Operation {
    pub id: &'static str,
    pub display_name: &'static str,
    pub handler: HandlerFn,
}

pub struct Registry {
    pub operations: Vec<Operation>,
}

impl Registry {
    pub fn new() -> Self {
        let mut ops = vec![
            Operation {
                id: "image_conv",
                display_name: "Image Conversion",
                handler: converters::image::conv_image,
            },
            Operation {
                id: "video_gif",
                display_name: "Video to GIF",
                handler: converters::video::conv_video,
            },
            Operation {
                id: "pdf_tools",
                display_name: "PDF Tools",
                handler: converters::pdf::conv_doc,
            },
            Operation {
                id: "image_compress",
                display_name: "Image Compression",
                handler: compression::image::compress_image,
            },
            Operation {
                id: "markdown_pdf",
                display_name: "Markdown to PDF",
                handler: converters::markdown::convert_markdown_to_pdf,
            },
        ];

        #[cfg(feature = "pdf")]
        ops.push(Operation {
            id: "pdf_compress",
            display_name: "PDF Compression",
            handler: compression::pdf::compress_pdf,
        });

        Registry { operations: ops }
    }
}
