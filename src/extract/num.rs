use extract::{Extract, Error, Context, Immediate};
use util::BufStream;

use atoi::{atoi, FromRadix10};

use std::error::Error as E;
use std::str::FromStr;

fn extract_integer<T>(ctx: &Context) -> Immediate<T>
where
    T: FromRadix10 + FromStr,
    <T as FromStr>::Err: E,
{
    use codegen::Source::*;

    match ctx.callsite().source() {
        Capture(idx) => {
            let path = ctx.request().uri().path();
            let capture = ctx.captures().get(*idx, path);

            T::from_str(capture)
                .map_err(|err| Error::invalid_argument(&err.description()))
                .into()
        }
        Header(header_name) => {
            let value = match ctx.request().headers().get(header_name) {
                Some(value) => value,
                None => {
                    return Immediate::err(Error::missing_argument());
                }
            };

            match atoi(value.as_bytes()) {
                Some(s) => Immediate::ok(s),
                None => Immediate::err(Error::invalid_argument(&"invalid integer")),
            }
        }
        QueryString => {
            unimplemented!();
        }
        Body => {
            unimplemented!();
        }
        Unknown => {
            unimplemented!();
        }
    }
}

macro_rules! impl_integer_extract {
    ($number:ty) => {
        impl<B: BufStream> Extract<B> for $number {
            type Future = Immediate<Self>;

            fn extract(ctx: &Context) -> Self::Future {
                extract_integer(ctx)
            }
        }
    };
    ($($number:ty),*) => {
        $( impl_integer_extract!($number); )*
    };
}

impl_integer_extract!(i8, u8, i16, u16, i32, u32, i64, u64);
