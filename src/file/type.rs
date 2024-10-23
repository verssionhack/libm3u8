pub use master_playlist::MasterPlayList;
pub use media_playlist::MediaPlaylist;


pub mod media_playlist {
    use crate::file::tag::{Tag, TagMediaPlayList, TagMediaSegment};



    #[derive(Debug, Default, PartialEq, Clone)]
    pub struct MediaPlaylist {
        pub(crate) _media_segments: Vec<TagMediaSegment>,
        pub(crate) _media_playlists: Vec<TagMediaPlayList>,
        pub(crate) _urls: Vec<(String, Vec<TagMediaSegment>)>,
        pub _other: Vec<Tag>,
    }
}


pub mod master_playlist {
    use crate::file::tag::{attribute::AttributeList, Tag};



    #[allow(non_camel_case_types)]
    #[derive(Debug, Default, PartialEq, Clone, Copy)]
    pub enum HdcpLevel {
        TYPE_0,
        #[default]
        NONE,
    }

    #[allow(non_camel_case_types)]
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum VideoRange {
        SDR,
        PQ,
    }

    #[derive(Debug, Default)]
    pub struct ExtXStreamInf {
        pub bandwidth: u64,
        pub average_bandwidth: Option<u64>,
        pub codecs: Vec<String>,
        pub resolution: (u64, u64),
        pub frame_rate: Option<f64>,
        pub hdcp_level: Option<HdcpLevel>,
        pub audio: Option<String>,
        pub video: Option<String>,
        pub video_range: Option<VideoRange>,
        pub subtitles: Option<String>,
        pub closed_captions: Option<String>,
        pub _other: AttributeList,
    }

    #[derive(Debug, Default)]
    pub struct ExtXIFrameStreamInf {
        pub bandwidth: u64,
        pub average_bandwidth: Option<u64>,
        pub video_range: Option<VideoRange>,
        pub codecs: Vec<String>,
        pub resolution: (u64, u64),
        pub uri: Option<String>,
        pub _other: AttributeList,
    }

    #[allow(non_camel_case_types)]
    #[derive(Debug, Default, Clone, Copy)]
    pub enum ExtXMediaType {
        AUDIO,
        VIDEO,
        SUBTITLES,
        #[default]
        CLOSED_CAPTIONS,
    }

    #[derive(Debug, Default)]
    pub struct ExtXMedia {
        pub r#type: ExtXMediaType,
        pub uri: Option<String>,
        pub group_id: String,
        pub language: Option<String>,
        pub assoc_language: Option<String>,
        pub name: String,
        pub default: bool,
        pub auto_select: bool,
        pub forced: bool,
        pub instream_id: Option<String>,
        pub characteristics: Option<Vec<String>>,
        pub channels: Option<Vec<String>>,
        pub _other: AttributeList,
    }


    #[derive(Debug, Default)]
    pub struct MasterPlayList {
        pub(crate) _ext_x_medias: Vec<ExtXMedia>,
        pub(crate) _ext_x_i_frame_stream_medias: Vec<ExtXIFrameStreamInf>,
        pub(crate) _urls: Vec<(String, ExtXStreamInf)>,
        pub _other: Vec<Tag>,
    }
}
