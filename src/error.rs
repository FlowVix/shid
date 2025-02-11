macro_rules! make_error {
    (

        @kind: $report_type:ident;

        $struct_name:ident {
            $(
                @title: $title:expr;
                @msgs: [
                    $(
                        $area:expr => $fmt:literal $(: $($args:expr),+)?;
                    )*
                ];
                $variant:ident {
                    $(
                        $field:ident: $typ:ty
                    ),* $(,)?
                } $(,)?
            )*
        }
    ) => {

        #[derive(Debug, Clone)]
        pub enum $struct_name {
            $(
                $variant { $($field: $typ),* },
            )*
        }

        impl $struct_name {
            pub fn into_report(self) -> crate::error::Report {

                use owo_colors::OwoColorize;
                match self {
                    $(
                        Self::$variant { $($field),* } => crate::error::Report {
                            title: String::from($title),
                            typ: crate::error::ReportType::$report_type,
                            messages: Box::new([$(
                                ($area, format!($fmt $( , $( ($args).bright_white() )* )? )),
                            )*])
                        }
                    )*
                }
            }
        }

    };
}

pub(crate) use make_error;

use ahash::AHashMap;
use owo_colors::OwoColorize;
use palette::{FromColor, Okhsv, ShiftHue, Srgb};

use crate::sources::{span::CodeArea, SourceMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReportType {
    Error,
    Warning,
}
impl ReportType {
    pub fn display_str(self) -> String {
        match self {
            ReportType::Error => "error:".bright_red().bold().to_string(),
            ReportType::Warning => "warning:".bright_yellow().bold().to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Report {
    pub title: String,
    pub typ: ReportType,
    pub messages: Box<[(CodeArea, String)]>,
}

impl Report {
    pub fn display(&self, source_map: &SourceMap) {
        let mut files = AHashMap::new();
        for (area, msg) in &self.messages {
            files
                .entry(area.src)
                .or_insert_with(Vec::new)
                .push((area.span, msg));
        }

        let mut hsv = Okhsv::from_color(Srgb::new(228u8, 55, 90).into_format::<f32>());

        eprintln!("\n{} {}", self.typ.display_str(), self.title.bright_white());
        for (src, msgs) in files {
            eprintln!(
                "{}",
                lyneate::Report::new_byte_spanned(
                    &source_map[src].content,
                    msgs.iter().map(|(span, msg)| {
                        let color = Srgb::from_color(hsv).into_format::<u8>();
                        hsv = hsv.shift_hue(40.0);
                        (
                            span.start..span.end,
                            (*msg).clone(),
                            (color.red, color.green, color.blue),
                        )
                    }),
                )
                .display_str()
            )
        }
    }
}
