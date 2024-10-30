use core::panic;

use super::{master_playlist::{self, ExtXIFrameStreamInf, ExtXMediaType, ExtXStreamInf}, media_playlist, tag::{attribute::attributes, Tag, TagMasterPlayList, TagMediaPlayList, TagMediaSegment}, M3u8FileResult};

impl TryFrom<&str> for master_playlist::HdcpLevel {
    type Error = super::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "TYPE-0" => {
                Self::TYPE_0
            }
            "NONE" => {
                Self::NONE
            }
            _ => {
                return Err(super::Error::UnknownEnumValue(value.to_owned()));
            }
        })
    }
}

impl Into<&'static str> for master_playlist::HdcpLevel {
    fn into(self) -> &'static str {
        match self {
            Self::TYPE_0 => "TYPE-0",
            Self::NONE => "NONE",
        }
    }
}

impl TryFrom<&str> for master_playlist::VideoRange {
    type Error = super::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "SDR" => {
                Self::SDR
            }
            "PQ" => {
                Self::PQ
            }
            _ => {
                return Err(super::Error::UnknownEnumValue(value.to_owned()));
            }
        })
    }
}

impl Into<&'static str> for master_playlist::VideoRange {
    fn into(self) -> &'static str {
        match self {
            Self::SDR => "SDR",
            Self::PQ=> "PQ",
        }
    }
}

impl TryFrom<&str> for ExtXMediaType {
    type Error = super::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "AUDIO" => {
                Self::AUDIO
            }
            "VIDEO" => {
                Self::VIDEO
            }
            "SUBTITLES" => {
                Self::SUBTITLES
            }
            "CLOSED-CAPTIONS" => {
                Self::CLOSED_CAPTIONS
            }
            _ => {
                return Err(super::Error::UnknownEnumValue(value.to_owned()));
            }
        })
    }
}

impl Into<&'static str> for master_playlist::ExtXMediaType {
    fn into(self) -> &'static str {
        match self {
            Self::AUDIO => "AUDIO",
            Self::VIDEO => "VIDEO",
            Self::SUBTITLES => "SUBTITLES",
            Self::CLOSED_CAPTIONS => "CLOSED-CAPTIONS",
        }
    }
}


impl TryFrom<&TagMasterPlayList> for master_playlist::ExtXMedia {
    type Error = super::Error;

    fn try_from(value: &TagMasterPlayList) -> Result<Self, Self::Error> {

        if let TagMasterPlayList::EXT_X_MEDIA(mut ext_x_media) = value.clone() {
            let mut ret = Self::default();

            ret.r#type = ext_x_media.remove(attributes::TYPE).map(|v| v.unwrap_enum_string().try_into()).ok_or(super::Error::MissingNonOptionAttribute(attributes::TYPE))??;
            ret.uri = ext_x_media.remove(attributes::URI).map(|v| v.unwrap_string().to_owned());
            ret.group_id = ext_x_media.remove(attributes::GROUP_ID).map(|v| v.unwrap_string().to_owned()).ok_or(super::Error::MissingNonOptionAttribute(attributes::GROUP_ID))?;
            ret.language = ext_x_media.remove(attributes::LANGUAGE).map(|v| v.unwrap_string().to_owned());
            ret.assoc_language = ext_x_media.remove(attributes::ASSOC_LANGUAGE).map(|v| v.unwrap_string().to_owned());
            ret.name = ext_x_media.remove(attributes::NAME).map(|v| v.unwrap_string().to_owned()).ok_or(super::Error::MissingNonOptionAttribute(attributes::NAME))?;
            ret.default = ext_x_media.remove(attributes::DEFAULT).map(|v| v.unwrap_enum_string() == "YES").unwrap_or_default();
            ret.auto_select = ext_x_media.remove(attributes::AUTOSELECT).map(|v| v.unwrap_enum_string() == "YES").unwrap_or_default();
            ret.forced = ext_x_media.remove(attributes::FORCED).map(|v| v.unwrap_enum_string() == "YES").unwrap_or_default();
            ret.instream_id = ext_x_media.remove(attributes::INSTREAM_ID).map(|v| v.unwrap_string().to_owned());
            ret.characteristics = ext_x_media.remove(attributes::CHARACTERISTICS).map(|v| {
                let mut characteristics = Vec::new();
                for characteristic in v.unwrap_string().split(',') {
                    characteristics.push(characteristic.to_string());
                }
                characteristics
            });
            ret.channels = ext_x_media.remove(attributes::CHANNELS).map(|v| {
                let mut channels = Vec::new();
                for channel in v.unwrap_string().split('/') {
                    channels.push(channel.to_string());
                }
                channels
            });

            ret._other = ext_x_media;

            Ok(ret)
        } else {
            Err(super::Error::MisMatchTag)
        }


    }
}

impl TryFrom<&TagMasterPlayList> for master_playlist::ExtXStreamInf {
    type Error = super::Error;

    fn try_from(value: &TagMasterPlayList) -> Result<Self, Self::Error> {

        if let TagMasterPlayList::EXT_X_STREAM_INF(mut ext_x_stream_inf) = value.clone() {
            let mut ret = Self::default();

            ret.bandwidth = ext_x_stream_inf.remove(attributes::BANDWIDTH).map(|v| v.unwrap_int()).ok_or(super::Error::MissingNonOptionAttribute(attributes::BANDWIDTH))?;
            ret.average_bandwidth = ext_x_stream_inf.remove(attributes::AVERAGE_BANDWIDTH).map(|v| v.unwrap_int());
            ext_x_stream_inf.remove(attributes::CODECS).map(|v| {
                for codec in v.unwrap_string().split(',') {
                    ret.codecs.push(codec.to_string());
                }
            });

            ret.resolution = ext_x_stream_inf.remove(attributes::RESOLUTION).map(|v| v.unwrap_resolution()).ok_or(super::Error::MissingNonOptionAttribute(attributes::RESOLUTION))?;
            ret.frame_rate = ext_x_stream_inf.remove(attributes::FRAME_RATE).map(|v| v.unwrap_float());
            if let Some(v) = ext_x_stream_inf.remove(attributes::HDCP_LEVEL).map(|v| v.unwrap_enum_string().try_into()) {
                ret.hdcp_level = Some(v?);
            }
            if let Some(v) = ext_x_stream_inf.remove(attributes::VIDEO_RANGE).map(|v| v.unwrap_enum_string().try_into()) {
                ret.video_range = Some(v?);
            }
            ret.audio = ext_x_stream_inf.remove(attributes::AUDIO).map(|v| v.unwrap_string().to_owned());
            ret.video = ext_x_stream_inf.remove(attributes::VIDEO).map(|v| v.unwrap_string().to_owned());
            ret.subtitles = ext_x_stream_inf.remove(attributes::SUBTITLES).map(|v| v.unwrap_string().to_owned());
            ret.closed_captions = ext_x_stream_inf.remove(attributes::CLOSED_CAPTIONS).map(|v| v.unwrap_string().to_owned());
            ret._other = ext_x_stream_inf;
            Ok(ret)
        } else {
            Err(super::Error::MisMatchTag)
        }


    }
}

impl TryFrom<&TagMasterPlayList> for master_playlist::ExtXIFrameStreamInf {
    type Error = super::Error;

    fn try_from(value: &TagMasterPlayList) -> Result<Self, Self::Error> {

        if let TagMasterPlayList::EXT_X_I_FRAME_STREAM_INF(mut ext_x_i_frame_stream_inf) = value.clone() {
            let mut ret = Self::default();

            ret.bandwidth = ext_x_i_frame_stream_inf.remove(attributes::BANDWIDTH).map(|v| v.unwrap_int()).ok_or(super::Error::MissingNonOptionAttribute(attributes::BANDWIDTH))?;
            ret.average_bandwidth = ext_x_i_frame_stream_inf.remove(attributes::AVERAGE_BANDWIDTH).map(|v| v.unwrap_int());
            if let Some(v) = ext_x_i_frame_stream_inf.remove(attributes::VIDEO_RANGE).map(|v| v.unwrap_enum_string().try_into()) {
                ret.video_range = Some(v?);
            }
            ext_x_i_frame_stream_inf.remove(attributes::CODECS).map(|v| {
                for codec in v.unwrap_string().split(',') {
                    ret.codecs.push(codec.to_string());
                }
            });

            ret.resolution = ext_x_i_frame_stream_inf.remove(attributes::RESOLUTION).map(|v| v.unwrap_resolution()).ok_or(super::Error::MissingNonOptionAttribute(attributes::RESOLUTION))?;
            ret.uri = ext_x_i_frame_stream_inf.remove(attributes::URI).map(|v| v.unwrap_string().to_owned());
            ret._other = ext_x_i_frame_stream_inf;
            Ok(ret)
        } else {
            Err(super::Error::MisMatchTag)
        }


    }
}


impl master_playlist::MasterPlayList {
    pub fn from_str(s: &str) -> M3u8FileResult<Self> {
        let mut ret = Self::default();

        let mut last_ext_x_stream_inf: Option<ExtXStreamInf> = None;

        for line in s.split('\n') {
            if line.len() == 0 {
                continue;
            }
            if !line.starts_with("#") {
                if last_ext_x_stream_inf.is_some() {
                    ret._urls.push((line.to_owned(), last_ext_x_stream_inf.take().unwrap()));
                } else {
                    panic!();
                }
            }
            let Ok(tag) = Tag::try_from(line) else {
                continue;
            };
            match tag {
                Tag::MasterPlayList(master_tag) => {
                    match master_tag {
                        TagMasterPlayList::EXT_X_MEDIA(_) => {
                            ret._ext_x_medias.push((&master_tag).try_into().unwrap());
                        }
                        TagMasterPlayList::EXT_X_STREAM_INF(_) => {
                            if last_ext_x_stream_inf.is_none() {
                                last_ext_x_stream_inf = Some((&master_tag).try_into().unwrap());
                            }
                        }
                        TagMasterPlayList::EXT_X_I_FRAME_STREAM_INF(_) => {
                            ret._ext_x_i_frame_stream_medias.push((&master_tag).try_into().unwrap());
                        }
                        _ => {
                            ret._other.push(master_tag.into());
                        }
                    }
                }
                _ => {
                    ret._other.push(tag);
                }
            }
        }

        Ok(ret)
    }
    pub fn ext_x_medias(&self) -> &[master_playlist::ExtXMedia] {
        &self._ext_x_medias
    }
    pub fn ext_x_i_frame_stream_media_infs(&self) -> &[master_playlist::ExtXIFrameStreamInf] {
        &self._ext_x_i_frame_stream_medias
    }
    pub fn urls(&self) -> &[(String, ExtXStreamInf)] {
        &self._urls
    }
    pub fn group_ids(&self) -> Vec<&str> {
        self.ext_x_medias().iter()
            .fold(Vec::new(), |mut s, v| {
                if !s.contains(&v.group_id.as_str()) {
                    s.push(&v.group_id);
                }
                s
            })
    }
}

impl media_playlist::MediaPlaylist {
    pub fn from_str(s: &str) -> M3u8FileResult<Self> {
        let mut ret = Self::default();

        let mut media_segments: Vec<TagMediaSegment> = Vec::new();
        let mut url: Option<String> = None;

        for line in s.split('\n') {
            if line.len() == 0 {
                continue;
            }
            if !line.starts_with("#") {
                url = Some(line.to_owned());
                continue;
            }
            let Ok(tag) = Tag::try_from(line) else {
                continue;
            };
            match tag {
                Tag::MediaSegment(v) => {
                    if let TagMediaSegment::EXTINF { duration: _, title: _ } = v {
                        if media_segments.len() > 0 {
                            ret._urls.push((url.take().unwrap(), media_segments.clone()));
                            media_segments.clear();
                        }
                        media_segments.push(v);
                    } else {
                        if media_segments.len() > 0 {
                            media_segments.push(v);
                        } else {
                            ret._media_segments.push(v);
                        }
                    }
                }
                Tag::MediaPlayList(v) => {
                    if url.is_some() {
                        ret._urls.push((url.take().unwrap(), media_segments.clone()));
                        media_segments.clear();
                    }
                    ret._media_playlists.push(v);
                }
                _ => {
                    ret._other.push(tag);
                }
            }
        }

        Ok(ret)
    }
    pub fn media_playlist(&self) -> &[TagMediaPlayList] {
        &self._media_playlists
    }
    pub fn urls(&self) -> &[(String, Vec<TagMediaSegment>)] {
        &self._urls
    }
}
