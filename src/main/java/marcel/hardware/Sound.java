package marcel.hardware;

import javax.sound.sampled.*;
import javax.sound.sampled.AudioFormat.Encoding;
import javax.sound.sampled.DataLine.Info;

public class Sound {

    private final Thread player = null;

    private static final float frameRate = 44100f; // 44100 Hz
    private static final int channels = 1; // Mono
    private static final double duration = 0.020; // 20ms
    private static final int sampleBytes = Short.SIZE / 8; // 8 bits
    private static final int frameBytes = sampleBytes * channels;
    private static final AudioFormat format =
        new AudioFormat(Encoding.PCM_SIGNED,
                        frameRate,
                        Short.SIZE,
                        channels,
                        frameBytes,
                        frameRate,
                        true);
    private static final int nFrames = (int) Math.ceil(frameRate * duration);
    static int nSamples = nFrames * channels;
    private static final int nBytes = 1024; // Buffer size
    private static byte[] data;// = ByteBuffer.allocate(nBytes);
    static byte[] datain;

    private static SourceDataLine line;


    public Sound() {
        data = new byte[nBytes];
        // Réservation de la sortie audio, début de la restitution, envoi du tableau
        Info info = new Info(SourceDataLine.class, format);

        try {
            line = (SourceDataLine) AudioSystem.getLine(info);
            line.open(format);
        } catch (LineUnavailableException e) {
            e.printStackTrace();
        }
        line.start();

    }

    // Copie du buffer de son provenant du 6809 vers le buffer de la carte son
    // Cette fonction est lancée lorsque le buffer 6809 est plein

    public static synchronized void playSound(M6809 cpu) {
        for (int i = 0; i < nBytes; i++) {
            data[i / 4] = cpu.getSound_buffer(i);
        }
        line.write(data, 0, nBytes / 4);
    }
}
