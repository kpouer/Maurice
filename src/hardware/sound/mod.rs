use crate::hardware::M6809::M6809;
use crate::int;

const FRAME_RATE: f64 = 44100.0; // 44100 Hz
const CHANNELS: int = 1; // Mono
const DURATION: f64 = 0.020; // 20ms
const SAMPLE_BYTES: int = 16 / 8; // 8 bits
const FRAME_BYTES: int = SAMPLE_BYTES * CHANNELS;

// const format: AudioFormat =
//  AudioFormat::new(Encoding.PCM_SIGNED,
// FRAME_RATE,
// Short.SIZE,
// CHANNELS,
// FRAME_BYTES,
// FRAME_RATE,
// true);
const N_FRAMES: int = 882;//(FRAME_RATE * DURATION).ceil() as int;
const N_BYTES: usize = 1024; // Buffer size

// fn SourceDataLine line;

#[derive(Debug)]
pub(crate) struct Sound {
    n_samples:int,
    data:Vec<u8>,
}

impl Sound {
    pub(crate) fn new() -> Self {
        Sound {
            n_samples: N_FRAMES * CHANNELS,
            data: vec![0; N_BYTES],
        }
        // Réservation de la sortie audio, début de la restitution, envoi du tableau
        // Info info = new Info(self.SourceDataLine.class, format);
        //
        // try {
        //     line = (self.SourceDataLine) AudioSystem.getLine(info);
        //     line.open(format);
        // } catch (LineUnavailableException e) {
        //     e.printStackTrace(&mut self, mem: &Memory);
        // }
        // line.start(&mut self, mem: &Memory);
    }

    // Copie du buffer de son provenant du 6809 vers le buffer de la carte son
    // Cette fonction est lancée lorsque le buffer 6809 est plein
    fn /*synchronized*/ play_sound(&mut self, cpu: &M6809) {
        for i in 0..N_BYTES {
            self.data[i / 4] = cpu.sound_buffer[i];
        }
        // line.write(data, 0, nBytes / 4);
    }
}

