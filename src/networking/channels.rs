use bevy_renet::renet::{ChannelConfig, ConnectionConfig, SendType};
use std::time::Duration;

pub enum Channels { 
    Fast,
    Garanteed,
}

impl From<Channels> for u8 {
    fn from(channel_id: Channels) -> Self {
        match channel_id {
            Channels::Fast => 0,
            Channels::Garanteed => 1,
        }
    }
}

impl Channels {
    pub fn channels_config() -> Vec<ChannelConfig> {
        vec![
            ChannelConfig {
                channel_id: Self::Fast.into(),
                max_memory_usage_bytes: 2 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::ZERO,
                },
            },
            ChannelConfig {
                channel_id: Self::Garanteed.into(),
                max_memory_usage_bytes: 2 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::from_millis(200),
                },
            },
        ]
    }
}

pub fn connection_config() -> ConnectionConfig {
    ConnectionConfig {
        available_bytes_per_tick: 2 * 1024 * 1024,
        client_channels_config: Channels::channels_config(),
        server_channels_config: Channels::channels_config(),
    }
}

