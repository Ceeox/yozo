use mediatype::{
    names::{AV1, AVIF, IMAGE, JPEG, MP4, PNG, VIDEO, VP8, VP9},
    MediaType,
};

pub(crate) struct Converter;

impl Converter {
    pub fn is_mime_supported(mt: MediaType) -> bool {
        let ty = mt.ty;
        let subty = mt.subty;
        if ty.eq(&IMAGE) && subty.eq(&PNG) || subty.eq(&JPEG) || subty.eq(&AVIF) {
            return true;
        }
        if ty.eq(&VIDEO) && subty.eq(&MP4) || subty.eq(&AV1) || subty.eq(&VP8) || subty.eq(&VP9) {
            return true;
        }
        false
    }
}
