use super::{master_playlist::{self, ExtXMediaType}, tag::{self, attribute::attributes}};

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

impl TryFrom<&tag::TagMasterPlayList> for master_playlist::ExtXMedia {
    type Error = super::Error;

    fn try_from(value: &tag::TagMasterPlayList) -> Result<Self, Self::Error> {

        if let tag::TagMasterPlayList::EXT_X_MEDIA(ref ext_x_media) = value {
            let mut ret = Self::default();

            ret.r#type = ext_x_media.get(attributes::TYPE).map(|v| v.unwrap_enum_string()).ok_or(super::Error::MissingNonOptionAttribute(attributes::TYPE))?.try_into()?;
            ret.uri = ext_x_media.get(attributes::URI).map(|v| v.unwrap_string().to_owned());
            ret.group_id = ext_x_media.get(attributes::GROUP_ID).map(|v| v.unwrap_string().to_owned()).ok_or(super::Error::MissingNonOptionAttribute(attributes::GROUP_ID))?;
            ret.language = ext_x_media.get(attributes::LANGUAGE).map(|v| v.unwrap_string().to_owned());
            ret.assoc_language = ext_x_media.get(attributes::ASSOC_LANGUAGE).map(|v| v.unwrap_string().to_owned());
            ret.name = ext_x_media.get(attributes::NAME).map(|v| v.unwrap_string().to_owned()).ok_or(super::Error::MissingNonOptionAttribute(attributes::NAME))?;
            ret.default = ext_x_media.get(attributes::DEFAULT).map(|v| v.unwrap_enum_string() == "YES").unwrap_or_default();
            ret.auto_select = ext_x_media.get(attributes::AUTOSELECT).map(|v| v.unwrap_enum_string() == "YES").unwrap_or_default();
            ret.forced = ext_x_media.get(attributes::FORCED).map(|v| v.unwrap_enum_string() == "YES").unwrap_or_default();
            ret.instream_id = ext_x_media.get(attributes::INSTREAM_ID).map(|v| v.unwrap_string().to_owned());
            ret.characteristics = ext_x_media.get(attributes::CHARACTERISTICS).map(|v| {
                let mut characteristics = Vec::new();
                for characteristic in v.unwrap_string().split(',') {
                    characteristics.push(characteristic.to_string());
                }
                characteristics
            });
            ret.channels = ext_x_media.get(attributes::CHANNELS).map(|v| {
                let mut channels = Vec::new();
                for channel in v.unwrap_string().split('/') {
                    channels.push(channel.to_string());
                }
                channels
            });

            Ok(ret)
        } else {
            Err(super::Error::MisMatchTag)
        }


    }
}

impl TryFrom<&tag::TagMasterPlayList> for master_playlist::ExtXStreamInf {
    type Error = super::Error;

    fn try_from(value: &tag::TagMasterPlayList) -> Result<Self, Self::Error> {

        if let tag::TagMasterPlayList::EXT_X_STREAM_INF(ref ext_x_stream_inf) = value {
            let mut ret = Self::default();

            ret.bandwidth = ext_x_stream_inf.get(attributes::BANDWIDTH).map(|v| v.unwrap_int()).ok_or(super::Error::MissingNonOptionAttribute(attributes::BANDWIDTH))?;
            ret.average_bandwidth = ext_x_stream_inf.get(attributes::AVERAGE_BANDWIDTH).map(|v| v.unwrap_int());
            ext_x_stream_inf.get(attributes::CODECS).map(|v| {
                for codec in v.unwrap_string().split(',') {
                    ret.codecs.push(codec.to_string());
                }
            });

            ret.resolution = ext_x_stream_inf.get(attributes::RESOLUTION).map(|v| v.unwrap_resolution()).ok_or(super::Error::MissingNonOptionAttribute(attributes::RESOLUTION))?;
            ret.frame_rate = ext_x_stream_inf.get(attributes::FRAME_RATE).map(|v| v.unwrap_float());
            if let Some(v) = ext_x_stream_inf.get(attributes::HDCP_LEVEL).map(|v| v.unwrap_enum_string()) {
                ret.hdcp_level = Some(v.try_into()?);
            }
            if let Some(v) = ext_x_stream_inf.get(attributes::VIDEO_RANGE).map(|v| v.unwrap_enum_string()) {
                ret.video_range = Some(v.try_into()?);
            }
            ret.audio = ext_x_stream_inf.get(attributes::AUDIO).map(|v| v.unwrap_string().to_owned());
            ret.video = ext_x_stream_inf.get(attributes::VIDEO).map(|v| v.unwrap_string().to_owned());
            ret.subtitles = ext_x_stream_inf.get(attributes::SUBTITLES).map(|v| v.unwrap_string().to_owned());
            ret.closed_captions = ext_x_stream_inf.get(attributes::CLOSED_CAPTIONS).map(|v| v.unwrap_string().to_owned());
            Ok(ret)
        } else {
            Err(super::Error::MisMatchTag)
        }


    }
}

