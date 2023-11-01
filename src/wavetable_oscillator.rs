use std::time::Duration;

use rodio::Source;

#[derive(Clone)]
pub struct WavetableOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
    total_duration: f32,
    fade_in_duration: f32,
    fade_out_duration: f32,
    elapsed_time: f32,
}

impl WavetableOscillator {
    pub fn new(sample_rate: u32, total_duration: f32, wave_table: Vec<f32>) -> WavetableOscillator {
        return WavetableOscillator {
            sample_rate: sample_rate,
            wave_table: wave_table,
            index: 0.0,
            index_increment: 0.0,
            total_duration: total_duration,
            fade_in_duration: total_duration/3.0,
            fade_out_duration: total_duration/2.0,
            elapsed_time: 0.0,
        };
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    fn get_sample(&mut self) -> f32 {
        let sample = self.lerp() * self.get_amplitude().clamp(0.0, 1.0);
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        return sample;
    }

    fn lerp(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();
        
        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        return truncated_index_weight * self.wave_table[truncated_index] 
               + next_index_weight * self.wave_table[next_index];
    }

    pub fn get_amplitude(&mut self) -> f32 {
        let sample_duration = 1.0 / self.sample_rate as f32;
        let mut envelope_value = 0.0;

        if self.elapsed_time < self.fade_in_duration {
            // Calcular fade-in
            envelope_value = self.elapsed_time / self.fade_in_duration;
        } else if self.elapsed_time >= self.total_duration - self.fade_out_duration {
            // Calcular fade-out
            let fade_out_start = self.total_duration - self.fade_out_duration;
            envelope_value = 1.0 - (self.elapsed_time - fade_out_start) / self.fade_out_duration;
        } else {
            // Nenhum fade
            envelope_value = 1.0;
        }

        self.elapsed_time += sample_duration;

        // if self.elapsed_time >= self.total_duration {
        //     // O som terminou, resetar o envelope
        //     self.elapsed_time = 0.0;
        //     envelope_value = 0.0;
        // }

        return envelope_value;
    }

}

impl Iterator for WavetableOscillator {
    type Item = f32;
    
    fn next(&mut self) -> Option<Self::Item> {
        return Some(self.get_sample());
    }
}

impl Source for WavetableOscillator {
    fn channels(&self) -> u16 {
        return 1;
    }

    fn sample_rate(&self) -> u32 {
        return self.sample_rate;
    }   

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}