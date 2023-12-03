package marcel.hardware;

import javax.sound.sampled.AudioFormat;
import javax.sound.sampled.AudioFormat.Encoding;
import javax.sound.sampled.AudioSystem;
import javax.sound.sampled.DataLine;
import javax.sound.sampled.LineUnavailableException;
import javax.sound.sampled.SourceDataLine;


public class Sound {
	
	private volatile Thread player = null;
	
	static float frameRate = 44100f; // 44100 Hz
	static int channels = 1; // Mono
	static double duration = 0.020; // 20ms
	static int sampleBytes = Short.SIZE / 8; // 8 bits
	static int frameBytes = sampleBytes * channels;
	static AudioFormat format =
	    new AudioFormat(Encoding.PCM_SIGNED,
	                    frameRate,
	                    Short.SIZE,
	                    channels,
	                    frameBytes,
	                    frameRate,
	                    true);
	static int nFrames = (int) Math.ceil(frameRate * duration);
	static int nSamples = nFrames * channels;
	static int nBytes = 1024; // Buffer size
	static byte []data;// = ByteBuffer.allocate(nBytes);
	static byte []datain = null;
	
	static SourceDataLine line = null;


	
    public Sound() {
    	data = new byte[nBytes];
    	// Réservation de la sortie audio, début de la restitution, envoi du tableau
    	DataLine.Info info = new DataLine.Info(SourceDataLine.class, format);
    	
		try { line = (SourceDataLine)AudioSystem.getLine(info); line.open(format); }
    	catch (LineUnavailableException e){e.printStackTrace();}
    	line.start();

    }

    // Copie du buffer de son provenant du 6809 vers le buffer de la carte son
    // Cette fonction est lancée lorsque le buffer 6809 est plein
    
    public static synchronized void playSound(final M6809 cpu) {
    	for ( int i=0; i<nBytes; i++ )                	
    		data[i/4] = cpu.sound_buffer[i];  
        line.write(data, 0, nBytes/4 );
          		
    }

	
	
	


}
