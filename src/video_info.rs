#[derive(Debug,PartialEq, Eq)]
pub enum AdaptiveType
{
    None,
    Audio,
    Video,
    Unknown
}

#[derive(Debug,PartialEq, Eq)]
enum VideoType
{
    Mobile,
    Flash,
    Mp4,
    WebM,
    Unknown
}

#[derive(Debug,PartialEq, Eq)]
enum AudioType
{
    Aac,
    Mp3,
    Vorbis,
    Unknown
}

#[derive(Debug,PartialEq, Eq)]
pub struct VideoInfo
{
    format_code: i32,
    video_type: VideoType,
    resolution: i32,
    is_3d: bool,
    audio_type: AudioType,
    audio_bit_rate: i32,
    pub adaptive_type: AdaptiveType,
    download_url: String
}

// Implementation block
impl VideoInfo {
    fn new(format_code: i32,
           video_type: VideoType,
           resolution: i32,
           is_3d: bool,
           audio_type: AudioType,
           audio_bit_rate: i32,
           adaptive_type: AdaptiveType) -> VideoInfo {
        VideoInfo {
            format_code: format_code,
            video_type: video_type,
            resolution: resolution,
            is_3d: is_3d,
            audio_type: audio_type,
            audio_bit_rate: audio_bit_rate,
            adaptive_type: adaptive_type,
            download_url: String::new()
        }
    }

    pub fn defaults() -> Vec<VideoInfo>
    {
        vec![
            // default
            VideoInfo::new(0, VideoType::Unknown, 0, false, AudioType::Unknown, 0, AdaptiveType::Unknown),

            // normal
            VideoInfo::new(5, VideoType::Flash, 240, false, AudioType::Mp3, 64, AdaptiveType::None),
            VideoInfo::new(6, VideoType::Flash, 270, false, AudioType::Mp3, 64, AdaptiveType::None),
            VideoInfo::new(13, VideoType::Mobile, 0, false, AudioType::Aac, 0, AdaptiveType::None),
            VideoInfo::new(17, VideoType::Mobile, 144, false, AudioType::Aac, 24, AdaptiveType::None),
            VideoInfo::new(18, VideoType::Mp4, 360, false, AudioType::Aac, 96, AdaptiveType::None),
            VideoInfo::new(22, VideoType::Mp4, 720, false, AudioType::Aac, 192, AdaptiveType::None),
            VideoInfo::new(34, VideoType::Flash, 360, false, AudioType::Aac, 128, AdaptiveType::None),
            VideoInfo::new(35, VideoType::Flash, 480, false, AudioType::Aac, 128, AdaptiveType::None),
            VideoInfo::new(36, VideoType::Mobile, 240, false, AudioType::Aac, 38, AdaptiveType::None),
            VideoInfo::new(37, VideoType::Mp4, 1080, false, AudioType::Aac, 192, AdaptiveType::None),
            VideoInfo::new(38, VideoType::Mp4, 3072, false, AudioType::Aac, 192, AdaptiveType::None),
            VideoInfo::new(43, VideoType::WebM, 360, false, AudioType::Vorbis, 128, AdaptiveType::None),
            VideoInfo::new(44, VideoType::WebM, 480, false, AudioType::Vorbis, 128, AdaptiveType::None),
            VideoInfo::new(45, VideoType::WebM, 720, false, AudioType::Vorbis, 192, AdaptiveType::None),
            VideoInfo::new(46, VideoType::WebM, 1080, false, AudioType::Vorbis, 192, AdaptiveType::None),
            /* 3d */
            VideoInfo::new(82, VideoType::Mp4, 360, true, AudioType::Aac, 96, AdaptiveType::None),
            VideoInfo::new(83, VideoType::Mp4, 240, true, AudioType::Aac, 96, AdaptiveType::None),
            VideoInfo::new(84, VideoType::Mp4, 720, true, AudioType::Aac, 152, AdaptiveType::None),
            VideoInfo::new(85, VideoType::Mp4, 520, true, AudioType::Aac, 152, AdaptiveType::None),
            VideoInfo::new(100, VideoType::WebM, 360, true, AudioType::Vorbis, 128, AdaptiveType::None),
            VideoInfo::new(101, VideoType::WebM, 360, true, AudioType::Vorbis, 192, AdaptiveType::None),
            VideoInfo::new(102, VideoType::WebM, 720, true, AudioType::Vorbis, 192, AdaptiveType::None),
            /* Adaptive (aka DASH) - Video */
            VideoInfo::new(133, VideoType::Mp4, 240, false, AudioType::Unknown, 0, AdaptiveType::Video),
            VideoInfo::new(134, VideoType::Mp4, 360, false, AudioType::Unknown, 0, AdaptiveType::Video),
            VideoInfo::new(135, VideoType::Mp4, 480, false, AudioType::Unknown, 0, AdaptiveType::Video),
            VideoInfo::new(136, VideoType::Mp4, 720, false, AudioType::Unknown, 0, AdaptiveType::Video),
            VideoInfo::new(137, VideoType::Mp4, 1080, false, AudioType::Unknown, 0, AdaptiveType::Video),
            VideoInfo::new(138, VideoType::Mp4, 2160, false, AudioType::Unknown, 0, AdaptiveType::Video),
            VideoInfo::new(160, VideoType::Mp4, 144, false, AudioType::Unknown, 0, AdaptiveType::Video),
            VideoInfo::new(242, VideoType::WebM, 240, false, AudioType::Unknown, 0, AdaptiveType::Video),
            VideoInfo::new(243, VideoType::WebM, 360, false, AudioType::Unknown, 0, AdaptiveType::Video),
            VideoInfo::new(244, VideoType::WebM, 480, false, AudioType::Unknown, 0, AdaptiveType::Video),
            VideoInfo::new(247, VideoType::WebM, 720, false, AudioType::Unknown, 0, AdaptiveType::Video),
            VideoInfo::new(248, VideoType::WebM, 1080, false, AudioType::Unknown, 0, AdaptiveType::Video),
            VideoInfo::new(264, VideoType::Mp4, 1440, false, AudioType::Unknown, 0, AdaptiveType::Video),
            VideoInfo::new(271, VideoType::WebM, 1440, false, AudioType::Unknown, 0, AdaptiveType::Video),
            VideoInfo::new(272, VideoType::WebM, 2160, false, AudioType::Unknown, 0, AdaptiveType::Video),
            VideoInfo::new(278, VideoType::WebM, 144, false, AudioType::Unknown, 0, AdaptiveType::Video),
            /* Adaptive (aka DASH) - Audio */
            VideoInfo::new(139, VideoType::Mp4, 0, false, AudioType::Aac, 48, AdaptiveType::Audio),
            VideoInfo::new(140, VideoType::Mp4, 0, false, AudioType::Aac, 128, AdaptiveType::Audio),
            VideoInfo::new(141, VideoType::Mp4, 0, false, AudioType::Aac, 256, AdaptiveType::Audio),
            VideoInfo::new(171, VideoType::WebM, 0, false, AudioType::Vorbis, 128, AdaptiveType::Audio),
            VideoInfo::new(172, VideoType::WebM, 0, false, AudioType::Vorbis, 192, AdaptiveType::Audio)
        ]
    }

    pub fn find_videoinfo_for_formatcode<'a>(infos: &'a Vec<VideoInfo>, format_code: i32) -> &'a VideoInfo
    {
        infos.iter().find(|&x| x.format_code == format_code).unwrap_or(&infos[0])
    }
}

#[test]
fn lookup()
{
    let infos: Vec<VideoInfo> = VideoInfo::defaults();
    let f: &VideoInfo = VideoInfo::find_videoinfo_for_formatcode(&infos, 5);
    let y: &VideoInfo = &infos[0];
    assert_eq!(*f, *y);
}