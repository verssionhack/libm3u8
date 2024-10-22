


#[derive(Debug)]
pub enum File {
    MasterPlayList(),
}


pub mod master_playlist {


    #[allow(non_camel_case_types)]
    #[derive(Debug, Default)]
    pub enum HdcpLevel {
        TYPE_0,
        #[default]
        NONE,
    }

    #[allow(non_camel_case_types)]
    #[derive(Debug)]
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
    }


    #[allow(non_camel_case_types)]
    #[derive(Debug, Default)]
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
    }


    #[derive(Debug)]
    pub struct MasterPlayList {
        pub ext_x_media: Option<ExtXMedia>,
        pub urls: Vec<(String, ExtXStreamInf)>,
    }

}
