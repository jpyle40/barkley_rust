use crate::game::SoundEvent;
use macroquad::audio::{PlaySoundParams, Sound, load_sound_from_bytes, play_sound};

pub struct Audio {
    click: Option<Sound>,
    step: Option<Sound>,
    success: Option<Sound>,
    stop: Option<Sound>,
}
impl Audio {
    pub async fn new() -> Self {
        Self {
            click: load_sound_from_bytes(&tone(&[(640., 0.04)], 0.12))
                .await
                .ok(),
            step: load_sound_from_bytes(&tone(&[(180., 0.045)], 0.08))
                .await
                .ok(),
            success: load_sound_from_bytes(&tone(
                &[
                    (523.25, 0.12),
                    (659.25, 0.12),
                    (783.99, 0.12),
                    (1046.5, 0.30),
                ],
                0.18,
            ))
            .await
            .ok(),
            stop: load_sound_from_bytes(&tone(&[(392., 0.12), (329.63, 0.2)], 0.12))
                .await
                .ok(),
        }
    }
    pub fn play(&self, event: SoundEvent, volume: f32) {
        if volume <= 0. {
            return;
        }
        let sound = match event {
            SoundEvent::Click => &self.click,
            SoundEvent::Step => &self.step,
            SoundEvent::Success => &self.success,
            SoundEvent::Stop => &self.stop,
        };
        if let Some(sound) = sound {
            play_sound(
                sound,
                PlaySoundParams {
                    looped: false,
                    volume,
                },
            );
        }
    }
}
/// Original, synthesized PCM cues. No external audio files or network needed.
fn tone(notes: &[(f32, f32)], gain: f32) -> Vec<u8> {
    let rate = 22050u32;
    let mut samples = Vec::<i16>::new();
    for &(frequency, seconds) in notes {
        let count = (seconds * rate as f32) as usize;
        for i in 0..count {
            let t = i as f32 / rate as f32;
            let envelope = (t / 0.008).min(1.) * ((seconds - t) / 0.04).clamp(0., 1.);
            let wave = (std::f32::consts::TAU * frequency * t).sin()
                + 0.2 * (std::f32::consts::TAU * frequency * 2. * t).sin();
            samples.push((wave * envelope * gain * i16::MAX as f32) as i16);
        }
    }
    let size = (samples.len() * 2) as u32;
    let mut wav = Vec::new();
    wav.extend(b"RIFF");
    wav.extend((36 + size).to_le_bytes());
    wav.extend(b"WAVEfmt ");
    wav.extend(16u32.to_le_bytes());
    wav.extend(1u16.to_le_bytes());
    wav.extend(1u16.to_le_bytes());
    wav.extend(rate.to_le_bytes());
    wav.extend((rate * 2).to_le_bytes());
    wav.extend(2u16.to_le_bytes());
    wav.extend(16u16.to_le_bytes());
    wav.extend(b"data");
    wav.extend(size.to_le_bytes());
    for sample in samples {
        wav.extend(sample.to_le_bytes());
    }
    wav
}
