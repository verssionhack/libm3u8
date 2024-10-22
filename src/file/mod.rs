mod r#error;
pub mod r#trait;
mod r#impl;
mod r#type;
pub use r#trait::*;
pub use r#type::*;
pub use r#error::*;

pub mod tag;


#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::m3u8::file::tag::Tag;

    use super::{master_playlist, tag};



    #[test]
    fn test_read_tags() {
        let txt = include_str!("../../../resources/prog_index.m3u8");

        //let mut tags = Vec::new();

        for line in txt.split('\n') {
            if line.starts_with("#") {
                println!("{}", line);
                let tag = Tag::try_from(line).unwrap();
                match tag {
                    Tag::MasterPlayList(tag) => {
                        match tag {
                            tag::TagMasterPlayList::EXT_X_MEDIA(_) => {
                                let ext_x_media = master_playlist::ExtXMedia::try_from(&tag).unwrap();

                                println!("{:#?}", ext_x_media)
                            }
                            tag::TagMasterPlayList::EXT_X_STREAM_INF(_) => {
                                let ext_x_media = master_playlist::ExtXStreamInf::try_from(&tag).unwrap();

                                println!("{:#?}", ext_x_media)
                            }
                            _ => {
                                println!("{:?}", tag);
                            }
                        }
                    }
                    _ => {
                        println!("{:?}", tag);
                    }
                }
            }
        }
        
        //println!("{:#?}", tags);

    }


    fn test_parse_ext_x_stream_inf() {
        let tag_s: tag::TagMasterPlayList = "#EXT-X-STREAM-INF:BANDWIDTH=800000,AVERAGE-BANDWIDTH=800000,RESOLUTION=1080x608,CODECS=\"avc1.42e00a.mp4a.40.2\",FRAME-RATE=1.1111,HDCP-LEVEL=NONE".try_into().unwrap();

        let ext_x_stream_inf = master_playlist::ExtXStreamInf::try_from(&tag_s).unwrap();

        println!("{:?}", ext_x_stream_inf)

    }

    fn test_parse_ext_x_media() {
        let tag_s: tag::TagMasterPlayList = "#EXT-X-MEDIA:TYPE=VIDEO,URI=\"test\",GROUP-ID=\"group-id\",LANGUAGE=\"jp\",NAME=\"name\",DEFAULT=NO".try_into().unwrap();

        let ext_x_media = master_playlist::ExtXMedia::try_from(&tag_s).unwrap();

        println!("{:?}", ext_x_media)

    }
    
    
}
