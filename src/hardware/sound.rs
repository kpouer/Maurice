use crate::hardware::M6809::M6809;
use crate::int;

const frameRate: f64 = 44100.0; // 44100 Hz
const channels: int = 1; // Mono
const duration: f64 = 0.020; // 20ms
const sampleBytes: int = 16 / 8; // 8 bits
const frameBytes: int = sampleBytes * channels;

// const format: AudioFormat =
//  AudioFormat::new(Encoding.PCM_SIGNED,
// frameRate,
// Short.SIZE,
// channels,
// frameBytes,
// frameRate,
// true);
const nFrames: int = 882;//(frameRate * duration).ceil() as int;
const n_bytes: usize = 1024; // Buffer size

// fn SourceDataLine line;

#[derive(Debug)]
pub(crate) struct Sound {
    nSamples:int,
    data:Vec<u8>,
}

impl Sound {
    pub(crate) fn new() -> Self {
        Sound {
            nSamples: nFrames * channels,
            data: vec![0; n_bytes],
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
        for i in 0..n_bytes {
            self.data[i / 4] = cpu.sound_buffer[i];
        }
        // line.write(data, 0, nBytes / 4);
    }
}

