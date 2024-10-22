
use chrono::NaiveDateTime;

use super::attribute::AttributeList;


#[allow(non_camel_case_types)]
pub mod tags {
    pub const EXTM3U: &'static str = "EXTM3U";
    pub const EXT_X_VERSION: &'static str = "EXT-X-VERSION";


    pub const EXTINF: &'static str = "EXTINF";
    pub const EXT_X_BYTERANGE: &'static str = "EXT-X-BYTERANGE";
    pub const EXT_X_DISCONTINUITY: &'static str = "EXT-X-DISCONTINUITY";
    pub const EXT_X_KEY: &'static str = "EXT-X-KEY";
    pub const EXT_X_MAP: &'static str = "EXT-X-MAP";
    pub const EXT_X_PROGRAM_DATE_TIME: &'static str = "EXT-X-PROGRAM-DATE-TIME";
    pub const EXT_X_DATERANGE: &'static str = "EXT-X-DATERANGE";
    pub const EXT_X_BITRATE: &'static str = "EXT-X-BITRATE";

    pub const EXT_X_TARGETDURATION: &'static str = "EXT-X-TARGETDURATION";
    pub const EXT_X_MEDIA_SEQUENCE: &'static str = "EXT-X-MEDIA-SEQUENCE";
    pub const EXT_X_DISCONTINUITY_SEQUENCE: &'static str = "EXT-X-DISCONTINUITY-SEQUENCE";
    pub const EXT_X_ENDLIST: &'static str = "EXT-X-ENDLIST";
    pub const EXT_X_PLAYLIST_TYPE: &'static str = "EXT-X-PLAYLIST-TYPE";
    pub const EXT_X_I_FRAMES_ONLY: &'static str = "EXT-X-I-FRAMES-ONLY";

    pub const EXT_X_MEDIA: &'static str = "EXT-X-MEDIA";
    pub const EXT_X_STREAM_INF: &'static str = "EXT-X-STREAM-INF";
    pub const EXT_X_I_FRAME_STREAM_INF: &'static str = "EXT-X-I-FRAME-STREAM-INF";
    pub const EXT_X_SESSION_DATA: &'static str = "EXT-X-SESSION-DATA";
    pub const EXT_X_SESSION_KEY: &'static str = "EXT-X-SESSION-KEY";
    pub const EXT_X_INDEPENDENT_SEGMENTS: &'static str = "EXT-X-INDEPENDENT-SEGMENTS";
    pub const EXT_X_START: &'static str = "EXT-X-START";
}


#[allow(non_camel_case_types)]
pub mod tag_matcher {
    use regex::Regex;
    use lazy_static::lazy_static;
    use super::tags;

    lazy_static! {
        pub static ref BASIC_EXTM3U: Regex = Regex::new(&format!("^#{}$", tags::EXTM3U)).unwrap();
        pub static ref BASIC_EXT_X_VERSION: Regex = Regex::new(&format!("^#{}:(?<version>\\d+)$", tags::EXT_X_VERSION)).unwrap();


        pub static ref MEDIA_SEGMENT_EXTINF: Regex = Regex::new(&format!(r"^#{}:(?<duration>[\d\.]+),?(?<title>.+)?$", tags::EXTINF)).unwrap();
        pub static ref MEDIA_SEGMENT_EXT_X_BYTERANGE: Regex = Regex::new(&format!("^#{}:(?<n>\\d+)(@(?<o>\\d+))?$", tags::EXT_X_BYTERANGE)).unwrap();
        pub static ref MEDIA_SEGMENT_EXT_X_DISCONTINUITY: Regex = Regex::new(&format!("^#{}$", tags::EXT_X_DISCONTINUITY)).unwrap();
        pub static ref MEDIA_SEGMENT_EXT_X_KEY: Regex = Regex::new(&format!("^#{}:(?<key>.*)$", tags::EXT_X_KEY)).unwrap();
        pub static ref MEDIA_SEGMENT_EXT_X_MAP: Regex = Regex::new(&format!("^#{}:(?<map>.*)$", tags::EXT_X_MAP)).unwrap();
        pub static ref MEDIA_SEGMENT_EXT_X_PROGRAM_DATE_TIME: Regex = Regex::new(&format!("^#{}:(?<datetime>.*)Z$", tags::EXT_X_PROGRAM_DATE_TIME)).unwrap();
        pub static ref MEDIA_SEGMENT_EXT_X_DATERANGE: Regex = Regex::new(&format!("^#{}:(?<daterange>.*)$", tags::EXT_X_DATERANGE)).unwrap();
        pub static ref MEDIA_SEGMENT_EXT_X_BITRATE: Regex = Regex::new(&format!("^#{}:(?<bitrate>.*)$", tags::EXT_X_BITRATE)).unwrap();

        pub static ref MEDIA_PLAYLIST_EXT_X_TARGETDURATION: Regex = Regex::new(&format!("^#{}:(?<targetduration>.*)$", tags::EXT_X_TARGETDURATION)).unwrap();
        pub static ref MEDIA_PLAYLIST_EXT_X_MEDIA_SEQUENCE: Regex = Regex::new(&format!("^#{}:(?<media_sequence>.*)$", tags::EXT_X_MEDIA_SEQUENCE)).unwrap();
        pub static ref MEDIA_PLAYLIST_EXT_X_DISCONTINUITY_SEQUENCE: Regex = Regex::new(&format!("^#{}:(?<discontinuity_sequence>.*)$", tags::EXT_X_DISCONTINUITY_SEQUENCE)).unwrap();
        pub static ref MEDIA_PLAYLIST_EXT_X_ENDLIST: Regex = Regex::new(&format!("^#{}$", tags::EXT_X_ENDLIST)).unwrap();
        pub static ref MEDIA_PLAYLIST_EXT_X_PLAYLIST_TYPE: Regex = Regex::new(&format!("^#{}:(?<playlist_type>.*)$", tags::EXT_X_PLAYLIST_TYPE)).unwrap();
        pub static ref MEDIA_PLAYLIST_EXT_X_I_FRAMES_ONLY: Regex = Regex::new(&format!("^#{}$", tags::EXT_X_I_FRAMES_ONLY)).unwrap();

        pub static ref MASTER_PLAYLIST_EXT_X_MEDIA: Regex = Regex::new(&format!("^#{}:(?<media>.*)$", tags::EXT_X_MEDIA)).unwrap();
        pub static ref MASTER_PLAYLIST_EXT_X_STREAM_INF: Regex = Regex::new(&format!("^#{}:(?<stream_inf>.*)$", tags::EXT_X_STREAM_INF)).unwrap();
        pub static ref MASTER_PLAYLIST_EXT_X_I_FRAME_STREAM_INF: Regex = Regex::new(&format!("^#{}:(?<i_frame_stream_inf>.*)$", tags::EXT_X_I_FRAME_STREAM_INF)).unwrap();
        pub static ref MASTER_PLAYLIST_EXT_X_SESSION_DATA: Regex = Regex::new(&format!("^#{}:(?<session_data>.*)$", tags::EXT_X_SESSION_DATA)).unwrap();
        pub static ref MASTER_PLAYLIST_EXT_X_SESSION_KEY: Regex = Regex::new(&format!("^#{}:(?<session_key>.*)$", tags::EXT_X_SESSION_KEY)).unwrap();
        pub static ref MASTER_PLAYLIST_EXT_X_INDEPENDENT_SEGMENTS: Regex = Regex::new(&format!("^#{}$", tags::EXT_X_INDEPENDENT_SEGMENTS)).unwrap();
        pub static ref MASTER_PLAYLIST_EXT_X_START: Regex = Regex::new(&format!("^#{}:(?<start>.*)$", tags::EXT_X_START)).unwrap();

    }
}

#[derive(Debug, PartialEq)]
pub enum Tag {
    Basic(TagBasic),
    MediaSegment(TagMediaSegment),
    MediaPlayList(TagMediaPlayList),
    MasterPlayList(TagMasterPlayList),
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum TagBasic {
    EXTM3U,
    EXT_X_VERSION(u64),
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum TagMediaSegment {
    EXTINF {
        duration: f64,
        title: Option<String>,
    },
    EXT_X_BYTERANGE {
        n: u64,
        o: Option<u64>,
    },
    EXT_X_DISCONTINUITY,
    EXT_X_KEY(AttributeList),
    EXT_X_MAP(AttributeList),
    EXT_X_PROGRAM_DATE_TIME(NaiveDateTime),
    EXT_X_DATE_TIME(NaiveDateTime),
    EXT_X_DATERANGE(AttributeList),
    EXT_X_BITRATE(u64),
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum TagMediaPlayListPlayListType {
    VOD,
    EVENT,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum TagMediaPlayList {
    EXT_X_TARGETDURATION(u64),
    EXT_X_MEDIA_SEQUENCE(u64),
    EXT_X_DISCONTINUITY_SEQUENCE(u64),
    EXT_X_ENDLIST,
    EXT_X_PLAYLIST_TYPE(TagMediaPlayListPlayListType),
    EXT_X_I_FRAMES_ONLY,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum TagMasterPlayList {
    EXT_X_MEDIA(AttributeList),
    EXT_X_STREAM_INF(AttributeList),
    EXT_X_I_FRAME_STREAM_INF(AttributeList),
    EXT_X_SESSION_DATA(AttributeList),
    EXT_X_SESSION_KEY(AttributeList),
    EXT_X_INDEPENDENT_SEGMENTS,
    EXT_X_START(AttributeList),
}
