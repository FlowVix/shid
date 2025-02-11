use crate::{error::make_error, sources::span::CodeArea};

make_error! {
    @kind: Warning;

    ParserWarning {

        @title: "Gaga";
        @msgs: [
            v => "{}": sog;
        ];
        Blibby {
            v: CodeArea,
            sog: String,
        }

    }
}
