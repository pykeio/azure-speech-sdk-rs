use crate::config::Device;
use crate::synthesizer::ssml::Voice;
use crate::synthesizer::AudioFormat;
use crate::synthesizer::Language;
use crate::RequestId;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct Config {
    pub(crate) output_format: AudioFormat,

    pub(crate) device: Device,

    pub(crate) language: Language,
    pub(crate) voice: Option<Voice>,

    pub(crate) bookmark_enabled: bool,
    pub(crate) word_boundary_enabled: bool,
    pub(crate) punctuation_boundary_enabled: bool,
    pub(crate) sentence_boundary_enabled: bool,
    pub(crate) session_end_enabled: bool,
    pub(crate) viseme_enabled: bool,

    pub(crate) auto_detect_language: bool,

    pub(crate) on_session_started: Option<OnSessionStarted>,
    pub(crate) on_session_ended: Option<OnSessionEnded>,
    pub(crate) on_synthesising: Option<OnSynthesising>,
    pub(crate) on_audio_metadata: Option<OnAudioMetadata>,
    pub(crate) on_synthesised: Option<OnSynthesised>,
    pub(crate) on_error: Option<OnError>,
}

pub type OnSessionStarted = Arc<Box<dyn Fn(RequestId) + Send + Sync + 'static>>;
pub type OnSessionEnded = Arc<Box<dyn Fn(RequestId) + Send + Sync + 'static>>;
pub type OnSynthesising = Arc<Box<dyn Fn(RequestId, Vec<u8>) + Send + Sync + 'static>>;
pub type OnAudioMetadata = Arc<Box<dyn Fn(RequestId, String) + Send + Sync + 'static>>;
pub type OnSynthesised = Arc<Box<dyn Fn(RequestId) + Send + Sync + 'static>>;
pub type OnError = Arc<Box<dyn Fn(RequestId, crate::Error) + Send + Sync + 'static>>;

impl Config {
    pub fn new() -> Self {
        Self {
            session_end_enabled: true,
            auto_detect_language: true,
            ..Default::default()
        }
    }

    pub fn with_language(mut self, language: Language) -> Self {
        self.language = language;
        self
    }

    pub fn with_voice(mut self, voice: Voice) -> Self {
        self.voice = Some(voice);
        self
    }

    pub fn with_output_format(mut self, output_format: AudioFormat) -> Self {
        self.output_format = output_format;
        self
    }

    pub fn enable_bookmark(mut self) -> Self {
        self.bookmark_enabled = true;
        self
    }

    pub fn enable_word_boundary(mut self) -> Self {
        self.word_boundary_enabled = true;
        self
    }

    pub fn enable_punctuation_boundary(mut self) -> Self {
        self.punctuation_boundary_enabled = true;
        self
    }

    pub fn enable_sentence_boundary(mut self) -> Self {
        self.sentence_boundary_enabled = true;
        self
    }

    pub fn enable_session_end(mut self) -> Self {
        self.session_end_enabled = true;
        self
    }

    pub fn enable_viseme(mut self) -> Self {
        self.viseme_enabled = true;
        self
    }

    pub fn disable_auto_detect_language(mut self) -> Self {
        self.auto_detect_language = false;
        self
    }

    pub fn set_device(mut self, device: Device) -> Self {
        self.device = device;
        self
    }

    pub fn on_session_start<Func>(mut self, func: Func) -> Self
    where
        Func: Send + Sync + 'static + Fn(RequestId),
    {
        self.on_session_started = Some(Arc::new(Box::new(func)));
        self
    }

    pub fn on_session_end<Func>(mut self, func: Func) -> Self
    where
        Func: Send + Sync + 'static + Fn(RequestId),
    {
        self.on_session_ended = Some(Arc::new(Box::new(func)));
        self
    }

    pub fn on_synthesising<Func>(mut self, func: Func) -> Self
    where
        Func: Send + Sync + 'static + Fn(RequestId, Vec<u8>),
    {
        self.on_synthesising = Some(Arc::new(Box::new(func)));
        self
    }

    pub fn on_audio_metadata<Func>(mut self, func: Func) -> Self
    where
        Func: Send + Sync + 'static + Fn(RequestId, String),
    {
        self.on_audio_metadata = Some(Arc::new(Box::new(func)));
        self
    }

    pub fn on_synthesised<Func>(mut self, func: Func) -> Self
    where
        Func: Send + Sync + 'static + Fn(RequestId),
    {
        self.on_synthesised = Some(Arc::new(Box::new(func)));
        self
    }

    pub fn on_error<Func>(mut self, func: Func) -> Self
    where
        Func: Send + Sync + 'static + Fn(RequestId, crate::Error),
    {
        self.on_error = Some(Arc::new(Box::new(func)));
        self
    }
}