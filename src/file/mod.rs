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

    use crate::file::{media_playlist, tag::Tag};

    use super::{master_playlist::{self, MasterPlayList}, tag};

    #[test]
    fn test_media_playlist_from_file() {
        let playlist = media_playlist::MediaPlaylist::from_str(include_str!("../../resources/mixed.m3u8"));


        //println!("{:#?}", playlist);
        assert!(playlist.is_ok())
    }

    #[test]
    fn test_master_playlist_from_file() {
        let master_playlist = MasterPlayList::from_str(include_str!("../../resources/main.m3u8"));


        //println!("{:#?}", master_playlist);
        assert!(master_playlist.is_ok())
    }

    fn test_read_tags() {
        let txt = include_str!("../../resources/prog_index.m3u8");

        //let mut tags = Vec::new();

        for line in txt.split('\n') {
            println!("{}", line);
            if line.starts_with("#") {
                let tag = Tag::try_from(line).unwrap();
                match tag {
                    _ => {
                        println!("{:?}", tag);
                    }
                }
            }
        }
        
        //println!("{:#?}", tags);

    }


    #[test]
    fn test_parse_ext_x_stream_inf() {
        let tag_s: tag::TagMasterPlayList = "#EXT-X-STREAM-INF:BANDWIDTH=800000,AVERAGE-BANDWIDTH=800000,RESOLUTION=1080x608,CODECS=\"avc1.42e00a.mp4a.40.2\",FRAME-RATE=1.1111,HDCP-LEVEL=NONE".try_into().unwrap();

        let ext_x_stream_inf = master_playlist::ExtXStreamInf::try_from(&tag_s);

        assert!(ext_x_stream_inf.is_ok());

        //println!("{:?}", ext_x_stream_inf)

    }

    #[test]
    fn test_parse_ext_x_media() {
        let tag_s: tag::TagMasterPlayList = "#EXT-X-MEDIA:TYPE=VIDEO,URI=\"test\",GROUP-ID=\"group-id\",LANGUAGE=\"jp\",NAME=\"name\",DEFAULT=NO".try_into().unwrap();

        let ext_x_media = master_playlist::ExtXMedia::try_from(&tag_s);

        assert!(ext_x_media.is_ok());
        //println!("{:?}", ext_x_media)

    }
    
    
}
