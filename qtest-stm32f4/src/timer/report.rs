use super::Channel;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TimerReport {
    pub cr1: u32,
    pub cr2: u32,
    pub smcr: u32,
    pub dier: u32,
    pub sr: u32,
    pub ccmr1: u32,
    pub ccmr2: u32,
    pub ccer: u32,
    pub cnt: u32,
    pub psc: u32,
    pub arr: u32,
    pub ccr1: u32,
    pub ccr2: u32,
    pub ccr3: u32,
    pub ccr4: u32,
    pub dcr: u32,
    pub dmar: u32,
    pub or: u32,
}

impl TimerReport {
    pub fn timer_freq(&self, src_freq: u32) -> u32 {
        src_freq / (self.psc + 1) / (self.arr + 1)
    }

    pub fn channel_report(&self, channel: Channel) -> TimerChannelReport {
        let enabled = self.channel_enabled(channel);
        let ccr = self.ccr(channel);
        let config = self.config(channel);

        TimerChannelReport {
            enabled,
            ccr,
            config,
        }
    }

    pub fn ccmr(&self, channel: Channel) -> u8 {
        let ccmr = match channel {
            Channel::Ch1 => self.ccmr1,
            Channel::Ch2 => self.ccmr1 >> 8,
            Channel::Ch3 => self.ccmr2,
            Channel::Ch4 => self.ccmr2 >> 8,
        };
        (ccmr & 0xFF) as u8
    }

    pub fn ccer(&self, channel: Channel) -> u8 {
        let shift = match channel {
            Channel::Ch1 => 0,
            Channel::Ch2 => 4,
            Channel::Ch3 => 8,
            Channel::Ch4 => 12,
        };
        ((self.ccer >> shift) & 0xB) as u8
    }

    pub fn ccr(&self, channel: Channel) -> u32 {
        match channel {
            Channel::Ch1 => self.ccr1,
            Channel::Ch2 => self.ccr2,
            Channel::Ch3 => self.ccr3,
            Channel::Ch4 => self.ccr4,
        }
    }

    pub fn channel_enabled(&self, channel: Channel) -> bool {
        (self.ccer(channel) & 0x1) != 0
    }

    pub fn config(&self, channel: Channel) -> ChannelConfig {
        let ccmr = self.ccmr(channel);
        let ccer = self.ccer(channel);
        let ccr = self.ccr(channel);

        let ccs = ccmr & 0b11;

        if ccs == 0 {
            let config = OutputConfig {
                fast_enable: (ccmr & 0x4) != 0,
                preload_enable: (ccmr & 0x8) != 0,
                mode: OutputMode::from((ccmr >> 4) & 0x7, self.arr, ccr).unwrap(),
                clear_enable: (ccmr & 0x80) != 0,
            };
            let polarity = match ccer & 0x2 {
                0 => OutputPolarity::ActiveHigh,
                _ => OutputPolarity::ActiveLow,
            };
            ChannelConfig::OutputCompare { config, polarity }
        } else {
            let source = match ccs {
                1 => match channel {
                    Channel::Ch1 | Channel::Ch2 => InputSource::Ti1,
                    Channel::Ch3 | Channel::Ch4 => InputSource::Ti3,
                },
                2 => match channel {
                    Channel::Ch1 | Channel::Ch2 => InputSource::Ti2,
                    Channel::Ch3 | Channel::Ch4 => InputSource::Ti4,
                },
                3 => InputSource::Trc,
                _ => unreachable!(),
            };
            let config = InputConfig {
                prescaler: InputPrescaler::from((ccmr >> 2) & 0x3).unwrap(),
                filter: InputFilter::from((ccmr >> 4) & 0xF).unwrap(),
            };
            let ccxp = (ccer & 0x2) != 0;
            let ccxnp = (ccer & 0x4) != 0;
            let polarity = match (ccxnp, ccxp) {
                (false, false) => InputPolarity::RisingEdge,
                (false, true) => InputPolarity::FallingEdge,
                (true, true) => InputPolarity::BothEdges,
                (true, false) => unreachable!(),
            };
            ChannelConfig::InputCapture {
                config,
                polarity,
                source,
            }
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TimerChannelReport {
    pub enabled: bool,
    pub ccr: u32,
    pub config: ChannelConfig,
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ChannelConfig {
    OutputCompare {
        config: OutputConfig,
        polarity: OutputPolarity,
    },
    InputCapture {
        config: InputConfig,
        polarity: InputPolarity,
        source: InputSource,
    },
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OutputConfig {
    pub mode: OutputMode,
    pub fast_enable: bool,
    pub preload_enable: bool,
    pub clear_enable: bool,
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OutputMode {
    Frozen,
    ActiveOnMatch,
    InactiveOnMatch,
    Toggle,
    ForceInactive,
    ForceActive,
    PwmMode1 { duty_cycle: u32 },
    PwmMode2 { duty_cycle: u32 },
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum OutputPolarity {
    ActiveHigh = 0,
    ActiveLow = 1,
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputConfig {
    pub prescaler: InputPrescaler,
    pub filter: InputFilter,
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum InputPrescaler {
    NoPrescaler = 0,
    Div2 = 1,
    Div4 = 2,
    Div8 = 3,
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum InputFilter {
    NoFilter = 0,
    FckIntN2 = 1,
    FckIntN4 = 2,
    FckIntN8 = 3,
    FdtsDiv2N6 = 4,
    FdtsDiv2N8 = 5,
    FdtsDiv4N6 = 6,
    FdtsDiv4N8 = 7,
    FdtsDiv8N6 = 8,
    FdtsDiv8N8 = 9,
    FdtsDiv16N5 = 10,
    FdtsDiv16N6 = 11,
    FdtsDiv16N8 = 12,
    FdtsDiv32N5 = 13,
    FdtsDiv32N6 = 14,
    FdtsDiv32N8 = 15,
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum InputPolarity {
    RisingEdge = 0,
    FallingEdge = 1,
    BothEdges = 3,
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InputSource {
    Ti1,
    Ti2,
    Ti3,
    Ti4,
    Trc,
}

impl From<OutputConfig> for u8 {
    fn from(config: OutputConfig) -> Self {
        let mut value: u8 = 0;
        if config.fast_enable {
            value |= 0x4;
        }
        if config.preload_enable {
            value |= 0x8;
        }
        value |= (u8::from(config.mode)) << 4;
        if config.clear_enable {
            value |= 0x80;
        }
        value
    }
}

impl OutputMode {
    pub fn from(value: u8, arr: u32, ccr: u32) -> Option<Self> {
        let duty_cycle = match arr {
            0 => 0,
            _ => ((ccr as f64 * 100.0) / arr as f64).round() as u32,
        };
        match value {
            0 => Some(OutputMode::Frozen),
            1 => Some(OutputMode::ActiveOnMatch),
            2 => Some(OutputMode::InactiveOnMatch),
            3 => Some(OutputMode::Toggle),
            4 => Some(OutputMode::ForceInactive),
            5 => Some(OutputMode::ForceActive),
            6 => Some(OutputMode::PwmMode1 { duty_cycle }),
            7 => Some(OutputMode::PwmMode2 { duty_cycle }),
            _ => None,
        }
    }
}

impl From<OutputMode> for u8 {
    fn from(mode: OutputMode) -> Self {
        match mode {
            OutputMode::Frozen => 0,
            OutputMode::ActiveOnMatch => 1,
            OutputMode::InactiveOnMatch => 2,
            OutputMode::Toggle => 3,
            OutputMode::ForceInactive => 4,
            OutputMode::ForceActive => 5,
            OutputMode::PwmMode1 { duty_cycle: _ } => 6,
            OutputMode::PwmMode2 { duty_cycle: _ } => 7,
        }
    }
}

impl From<InputConfig> for u8 {
    fn from(config: InputConfig) -> Self {
        let mut value: u8 = 0;
        value |= (config.prescaler as u8) << 2;
        value |= (config.filter as u8) << 4;
        value
    }
}

impl InputPrescaler {
    pub fn division_factor(&self) -> u8 {
        match self {
            InputPrescaler::NoPrescaler => 1,
            InputPrescaler::Div2 => 2,
            InputPrescaler::Div4 => 4,
            InputPrescaler::Div8 => 8,
        }
    }

    pub fn from(value: u8) -> Option<Self> {
        match value {
            0 => Some(InputPrescaler::NoPrescaler),
            1 => Some(InputPrescaler::Div2),
            2 => Some(InputPrescaler::Div4),
            3 => Some(InputPrescaler::Div8),
            _ => None,
        }
    }
}

impl InputFilter {
    pub fn from(value: u8) -> Option<Self> {
        match value {
            0 => Some(InputFilter::NoFilter),
            1 => Some(InputFilter::FckIntN2),
            2 => Some(InputFilter::FckIntN4),
            3 => Some(InputFilter::FckIntN8),
            4 => Some(InputFilter::FdtsDiv2N6),
            5 => Some(InputFilter::FdtsDiv2N8),
            6 => Some(InputFilter::FdtsDiv4N6),
            7 => Some(InputFilter::FdtsDiv4N8),
            8 => Some(InputFilter::FdtsDiv8N6),
            9 => Some(InputFilter::FdtsDiv8N8),
            10 => Some(InputFilter::FdtsDiv16N5),
            11 => Some(InputFilter::FdtsDiv16N6),
            12 => Some(InputFilter::FdtsDiv16N8),
            13 => Some(InputFilter::FdtsDiv32N5),
            14 => Some(InputFilter::FdtsDiv32N6),
            15 => Some(InputFilter::FdtsDiv32N8),
            _ => None,
        }
    }
}

impl From<InputSource> for u8 {
    fn from(source: InputSource) -> Self {
        match source {
            InputSource::Ti1 | InputSource::Ti3 => 1,
            InputSource::Ti2 | InputSource::Ti4 => 2,
            InputSource::Trc => 3,
        }
    }
}
