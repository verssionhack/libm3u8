mod r#error;
pub mod r#trait;
mod r#impl;
mod r#type;
pub use r#trait::*;
pub use r#type::*;
pub use r#error::*;


pub mod attribute;


#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;

    use crate::m3u8::file::tag::{attribute::AttributeList, TagBasic, TagMasterPlayList, TagMediaPlayList, TagMediaSegment};

    #[test]
    fn test_parse_basic_tag() {

        assert_eq!("#EXTM3U".try_into(), Ok(TagBasic::EXTM3U));
        assert_eq!("#EXT-X-VERSION:3".try_into(), Ok(TagBasic::EXT_X_VERSION(3)));
    }


    #[test]
    fn test_parse_media_segment_tag() {

        assert_eq!("#EXTINF:1.833333".try_into(), Ok(TagMediaSegment::EXTINF { duration: 1.833333, title: None }));
        assert_eq!("#EXT-X-BYTERANGE:1@2".try_into(), Ok(TagMediaSegment::EXT_X_BYTERANGE { n: 1, o: Some(2) }));
        assert_eq!("#EXT-X-DISCONTINUITY".try_into(), Ok(TagMediaSegment::EXT_X_DISCONTINUITY));
        /*
        assert_eq!("#EXT-X-KEY:METHOD=NONE,URI=\"path/to/key\",IV=0x1234,KEYFORMAT=\"identity\",KEYFORMATVERSIONS=\"1/2/5\"".try_into(), 
            Ok(TagMediaSegment::EXT_X_KEY(AttributeList::default())));
        */
        //assert_eq!("#EXT-X-MAP:URI=\"path/to/key\",BYTERANGE=\"byterange\"".try_into(), Ok(TagMediaSegment::EXT_X_MAP(AttributeList::default())));
        assert_eq!("#EXT-X-PROGRAM-DATE-TIME:2024-10-17T09:24:07.001Z".try_into(), Ok(TagMediaSegment::EXT_X_PROGRAM_DATE_TIME(NaiveDateTime::parse_from_str("2024-10-17T09:24:07.001", "%Y-%m-%dT%H:%M:%S%.f").unwrap())));
        //assert_eq!("#EXT-X-DATERANGE:ID=\"id\",CLASS=\"class\",START-DATE=\"start-date\",END-DATE=\"end-date\",DURATION=0,PLANNED-DURATION=0,END-ON-NEXT=YES".try_into(), Ok(TagMediaSegment::EXT_X_DATERANGE(AttributeList::default())));
    }

    #[test]
    fn test_parse_media_playlist_type_tag() {

        assert_eq!("#EXT-X-TARGETDURATION:1".try_into(), Ok(TagMediaPlayList::EXT_X_TARGETDURATION(1)));
        assert_eq!("#EXT-X-MEDIA-SEQUENCE:1".try_into(), Ok(TagMediaPlayList::EXT_X_MEDIA_SEQUENCE(1)));
        assert_eq!("#EXT-X-DISCONTINUITY-SEQUENCE:1".try_into(), Ok(TagMediaPlayList::EXT_X_DISCONTINUITY_SEQUENCE(1)));
        assert_eq!("#EXT-X-ENDLIST".try_into(), Ok(TagMediaPlayList::EXT_X_ENDLIST));
        assert_eq!("#EXT-X-PLAYLIST-TYPE:VOD".try_into(), Ok(TagMediaPlayList::EXT_X_PLAYLIST_TYPE("VOD".try_into().unwrap())));
        assert_eq!("#EXT-X-I-FRAMES-ONLY".try_into(), Ok(TagMediaPlayList::EXT_X_I_FRAMES_ONLY));
    }

    #[test]
    fn test_parse_master_playlist_tag() {

        //assert_eq!("#EXT-X-MEDIA:TYPE=VIDEO".try_into(), Ok(TagMasterPlayList::EXT_X_MEDIA(AttributeList::default())));
    }
}
