use chrono::NaiveDateTime;

use super::{tag_matcher, Tag, TagBasic, TagMasterPlayList, TagMediaPlayList, TagMediaPlayListPlayListType, TagMediaSegment};


impl TryFrom<&str> for Tag {
    type Error = super::Error;


    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(if let Ok(tag) = TagBasic::try_from(value) {
            Self::Basic(tag)
        } else if let Ok(tag) = TagMediaSegment::try_from(value) {
            Self::MediaSegment(tag)
        } else if let Ok(tag) = TagMediaPlayList::try_from(value) {
            Self::MediaPlayList(tag)
        } else if let Ok(tag) = TagMasterPlayList::try_from(value) {
            Self::MasterPlayList(tag)
        } else {
            return Err(super::Error::UnknownTag(value.to_string()));
        })
    }
}

impl Into<Tag> for TagBasic {
    fn into(self) -> Tag {
        Tag::Basic(self)
    }
}
impl Into<Tag> for TagMediaSegment {
    fn into(self) -> Tag {
        Tag::MediaSegment(self)
    }
}
impl Into<Tag> for TagMediaPlayList {
    fn into(self) -> Tag {
        Tag::MediaPlayList(self)
    }
}
impl Into<Tag> for TagMasterPlayList {
    fn into(self) -> Tag {
        Tag::MasterPlayList(self)
    }
}





impl TryFrom<&str> for TagBasic {
    type Error = super::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(
            if tag_matcher::BASIC_EXTM3U.is_match(value) {
                Self::EXTM3U
            } else if tag_matcher::BASIC_EXT_X_VERSION.is_match(value) {
                Self::EXT_X_VERSION(tag_matcher::BASIC_EXT_X_VERSION.captures(value).unwrap().name("version").unwrap().as_str().parse().unwrap())
            } else {
                return Err(super::Error::UnknownTag(value.to_string()));
            }
        )
    }
}

impl TryFrom<&str> for TagMediaSegment {
    type Error = super::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(
            if tag_matcher::MEDIA_SEGMENT_EXTINF.is_match(value) {
                let captures = tag_matcher::MEDIA_SEGMENT_EXTINF.captures(value).unwrap();
                Self::EXTINF {
                    duration: captures.name("duration").unwrap().as_str().parse().unwrap(),
                    title: captures.name("title").map(|v| v.as_str().to_string()),
                }
            } else if tag_matcher::MEDIA_SEGMENT_EXT_X_BYTERANGE.is_match(value) {
                let captures = tag_matcher::MEDIA_SEGMENT_EXT_X_BYTERANGE.captures(value).unwrap();
                Self::EXT_X_BYTERANGE { 
                    n: captures.name("n").unwrap().as_str().parse().unwrap(),
                    o: captures.name("o").map(|v| v.as_str().parse().unwrap()),
                }
            } else if tag_matcher::MEDIA_SEGMENT_EXT_X_DISCONTINUITY.is_match(value) {
                Self::EXT_X_DISCONTINUITY
            } else if tag_matcher::MEDIA_SEGMENT_EXT_X_KEY.is_match(value) {
                Self::EXT_X_KEY(tag_matcher::MEDIA_SEGMENT_EXT_X_KEY.captures(value).unwrap().name("key").unwrap().as_str().try_into()?)
            } else if tag_matcher::MEDIA_SEGMENT_EXT_X_BITRATE.is_match(value) {
                Self::EXT_X_BITRATE(tag_matcher::MEDIA_SEGMENT_EXT_X_BITRATE.captures(value).unwrap().name("bitrate").unwrap().as_str().parse().unwrap())
            } else if tag_matcher::MEDIA_SEGMENT_EXT_X_MAP.is_match(value) {
                Self::EXT_X_MAP(tag_matcher::MEDIA_SEGMENT_EXT_X_MAP.captures(value).unwrap().name("map").unwrap().as_str().try_into()?)
            } else if tag_matcher::MEDIA_SEGMENT_EXT_X_DATERANGE.is_match(value) {
                Self::EXT_X_DATERANGE(tag_matcher::MEDIA_SEGMENT_EXT_X_DATERANGE.captures(value).unwrap().name("daterange").unwrap().as_str().try_into()?)
            } else if tag_matcher::MEDIA_SEGMENT_EXT_X_PROGRAM_DATE_TIME.is_match(value) {
                Self::EXT_X_PROGRAM_DATE_TIME(NaiveDateTime::parse_from_str(tag_matcher::MEDIA_SEGMENT_EXT_X_PROGRAM_DATE_TIME.captures(value).unwrap().name("datetime").unwrap().as_str(), "%Y-%m-%dT%H:%M:%S%.f").unwrap())
            } else {
                return Err(super::Error::UnknownTag(value.to_string()));
            }
        )
    }
}

impl TryFrom<&str> for TagMediaPlayListPlayListType {
    type Error = super::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "VOD" => {
                Self::VOD
            }
            "EVENT" => {
                Self::EVENT
            }
            _ => {
                return Err(super::Error::UnknownTagValue(value.to_owned()));
            }
        })
    }
}

impl TryFrom<&str> for TagMediaPlayList {
    type Error = super::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(
            if tag_matcher::MEDIA_PLAYLIST_EXT_X_TARGETDURATION.is_match(value) {
                Self::EXT_X_TARGETDURATION(tag_matcher::MEDIA_PLAYLIST_EXT_X_TARGETDURATION.captures(value).unwrap().name("targetduration").unwrap().as_str().parse().unwrap())
            } else if tag_matcher::MEDIA_PLAYLIST_EXT_X_MEDIA_SEQUENCE.is_match(value) {
                Self::EXT_X_MEDIA_SEQUENCE(tag_matcher::MEDIA_PLAYLIST_EXT_X_MEDIA_SEQUENCE.captures(value).unwrap().name("media_sequence").unwrap().as_str().parse().unwrap())
            } else if tag_matcher::MEDIA_PLAYLIST_EXT_X_ENDLIST.is_match(value) {
                Self::EXT_X_ENDLIST
            } else if tag_matcher::MEDIA_PLAYLIST_EXT_X_DISCONTINUITY_SEQUENCE.is_match(value) {
                Self::EXT_X_DISCONTINUITY_SEQUENCE(tag_matcher::MEDIA_PLAYLIST_EXT_X_DISCONTINUITY_SEQUENCE.captures(value).unwrap().name("discontinuity_sequence").unwrap().as_str().parse().unwrap())
            } else if tag_matcher::MEDIA_PLAYLIST_EXT_X_PLAYLIST_TYPE.is_match(value) {
                Self::EXT_X_PLAYLIST_TYPE(tag_matcher::MEDIA_PLAYLIST_EXT_X_PLAYLIST_TYPE.captures(value).unwrap().name("playlist_type").unwrap().as_str().try_into()?)
            } else if tag_matcher::MEDIA_PLAYLIST_EXT_X_I_FRAMES_ONLY.is_match(value) {
                Self::EXT_X_I_FRAMES_ONLY
            } else {
                return Err(super::Error::UnknownTag(value.to_string()));
            }
        )
    }
}

impl TryFrom<&str> for TagMasterPlayList {
    type Error = super::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(
            if tag_matcher::MASTER_PLAYLIST_EXT_X_MEDIA.is_match(value) {
                Self::EXT_X_MEDIA(tag_matcher::MASTER_PLAYLIST_EXT_X_MEDIA.captures(value).unwrap().name("media").unwrap().as_str().try_into()?)
            } else if tag_matcher::MASTER_PLAYLIST_EXT_X_STREAM_INF.is_match(value) {
                Self::EXT_X_STREAM_INF(tag_matcher::MASTER_PLAYLIST_EXT_X_STREAM_INF.captures(value).unwrap().name("stream_inf").unwrap().as_str().try_into()?)
            } else if tag_matcher::MASTER_PLAYLIST_EXT_X_I_FRAME_STREAM_INF.is_match(value) {
                Self::EXT_X_I_FRAME_STREAM_INF(tag_matcher::MASTER_PLAYLIST_EXT_X_I_FRAME_STREAM_INF.captures(value).unwrap().name("i_frame_stream_inf").unwrap().as_str().try_into()?)
            } else if tag_matcher::MASTER_PLAYLIST_EXT_X_SESSION_DATA.is_match(value) {
                Self::EXT_X_SESSION_DATA(tag_matcher::MASTER_PLAYLIST_EXT_X_SESSION_DATA.captures(value).unwrap().name("session_data").unwrap().as_str().try_into()?)
            } else if tag_matcher::MASTER_PLAYLIST_EXT_X_SESSION_KEY.is_match(value) {
                Self::EXT_X_SESSION_KEY(tag_matcher::MASTER_PLAYLIST_EXT_X_SESSION_KEY.captures(value).unwrap().name("session_key").unwrap().as_str().try_into()?)
            } else if tag_matcher::MASTER_PLAYLIST_EXT_X_INDEPENDENT_SEGMENTS.is_match(value) {
                Self::EXT_X_INDEPENDENT_SEGMENTS
            } else if tag_matcher::MASTER_PLAYLIST_EXT_X_START.is_match(value) {
                Self::EXT_X_START(tag_matcher::MASTER_PLAYLIST_EXT_X_START.captures(value).unwrap().name("start").unwrap().as_str().try_into()?)
            } else {
                return Err(super::Error::UnknownTag(value.to_string()));
            }
        )
    }
}
