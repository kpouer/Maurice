package hardware;

import java.awt.*;
import java.awt.event.*;

public class Keyboard implements KeyListener {

    private Canvas screen;
    private Memory mem;
    // translation table from scancode to java keycodes VK_
    private int[] ftable;
    final public int EVENT = 0x8000;

    public Keyboard(Canvas screen, Memory mem) {
        this.screen = screen;
        this.mem = mem;
        screen.addKeyListener(this);
        int i;
        
        this.ftable = new int[128];

        for (i = 0; i < 128; i++) {
            ftable[i] = 0;
        }
        /* STOP */
        //ftable[0x6E]=0x29;
  /* 1 .. ACC */
        ftable[0x5E] = '1';
        ftable[0x4E] = '2';
        ftable[0x3E] = '3';
        ftable[0x2E] = '4';
        ftable[0x1E] = '5';
        ftable[0x0E] = '6';
        ftable[0x0C] = '7';
        ftable[0x1C] = '8';
        ftable[0x2C] = '9';
        ftable[0x3C] = '0';
        ftable[0x4C] = '-';
        ftable[0x5C] = '+';
        ftable[0x6C] = KeyEvent.VK_BACK_SPACE + EVENT;
        /* A .. --> */
        ftable[0x5A] = 'a';
        ftable[0x4A] = 'z';
        ftable[0x3A] = 'e';
        ftable[0x2A] = 'r';
        ftable[0x1A] = 't';
        ftable[0x0A] = 'y';
        ftable[0x08] = 'u';
        ftable[0x18] = 'i';
        ftable[0x28] = 'o';
        ftable[0x38] = 'p';
        ftable[0x48] = '/';
        ftable[0x58] = ')';
        /* Q .. enter */
        ftable[0x56] = 'q';
        ftable[0x46] = 's';
        ftable[0x36] = 'd';
        ftable[0x26] = 'f';
        ftable[0x16] = 'g';
        ftable[0x06] = 'h';
        ftable[0x04] = 'j';
        ftable[0x14] = 'k';
        ftable[0x24] = 'l';
        ftable[0x34] = 'm';
        ftable[0x68] = KeyEvent.VK_ENTER + EVENT;
        /* W .. , */
        ftable[0x60] = 'w';
        ftable[0x50] = 'x';
        ftable[0x64] = 'c';
        ftable[0x54] = 'v';
        ftable[0x44] = 'b';
        ftable[0x00] = 'n';
        ftable[0x10] = ',';
        
        ftable[0x20] = '.';
        ftable[0x30] = '@';
        ftable[0x6E] = 145 + EVENT;//STOP
        ftable[0x58] = '*';
        
        /* Specials keys */
        ftable[0x12] = KeyEvent.VK_INSERT+ EVENT;
        ftable[0x02] = KeyEvent.VK_DELETE+ EVENT;
        ftable[0x22] = 36+ EVENT;// Back to top
        ftable[0x62] = KeyEvent.VK_UP+ EVENT;
        ftable[0x52] = KeyEvent.VK_LEFT+ EVENT;
        ftable[0x32] = KeyEvent.VK_RIGHT+ EVENT;
        ftable[0x42] = KeyEvent.VK_DOWN+ EVENT;
        /* espace */
        ftable[0x40] = ' ';
        /* SHIFT + BASIC */
        ftable[0x70] = KeyEvent.VK_F12 + EVENT;//Shift
        ftable[0x72] = KeyEvent.VK_F11 + EVENT;//Basic
        
        /* CNT RAZ */
        ftable[0x6A] = 17+ EVENT;//CTRL
        ftable[0x66] = 27+ EVENT;//ECHAP = raz

    }

    public void keyTyped(KeyEvent e) {
    }

    public void keyMemory(int key, boolean press)
    {
    	if(press)
    		mem.setKey(key);
    	else
    		mem.remKey(key);
    }
    
    public void keyTranslator(KeyEvent e, boolean press)
    {
       	int tmp_char = e.getKeyChar();
        int tmp_code = e.getKeyCode()+ EVENT;
        
        //System.out.println("key " + tmp_code); // Debug
        int i;
        
     
        switch(tmp_char)
        {
        	case '!':
        		keyMemory(0x70,press);//Shift
        		keyMemory(0x5E,press);//1
        		return;
        	case '"':
        		keyMemory(0x70,press);//Shift
        		keyMemory(0x4E,press);//2
        		return;   
        	case '#':
        		keyMemory(0x70,press);//Shift
        		keyMemory(0x3E,press);//3
        		return;   
        	case '$':
        		keyMemory(0x70,press);//Shift
        		keyMemory(0x2E,press);//4
        		return;      
        	case '%':
        		keyMemory(0x70,press);//Shift
        		keyMemory(0x1E,press);//5
        		return;    
        	case '&':
        		keyMemory(0x70,press);//Shift
        		keyMemory(0x0E,press);//6
        		return;         
        	case 39://'
        		keyMemory(0x70,press);//Shift
        		keyMemory(0x0C,press);//7
        		return;    
        	case '(':
        		keyMemory(0x70,press);//Shift
        		keyMemory(0x1C,press);//8
        		return;  
        	case ')':
        		keyMemory(0x70,press);//Shift
        		keyMemory(0x2C,press);//9
        		return;   
        	case '=':
        		keyMemory(0x70,press);//Shift
        		keyMemory(0x4C,press);//-
        		return;  
        	case ';':
        		keyMemory(0x70,press);//Shift
        		keyMemory(0x5C,press);//+
        		return;         
        	case '?':
        		keyMemory(0x70,press);//Shift
        		keyMemory(0x48,press);// /
        		return;  
        	case ':':
        		keyMemory(0x70,press);//Shift
        		keyMemory(0x58,press);//*
        		return;      
        	case '<':
        		keyMemory(0x70,press);//Shift
        		keyMemory(0x10,press);//,
        		return;  
        	case '>':
        		keyMemory(0x70,press);//Shift
        		keyMemory(0x20,press);//.
        		return;          
        	case '^':
        		keyMemory(0x70,press);//Shift
        		keyMemory(0x30,press);//@
        		return;   
        	case '©':
        		keyMemory(0x6A,press);//Ctrl
        		keyMemory(0x64,press);//C
        		return;
        		
        	default:
        		break;
        }

        for (i = 0; i < 127; i++) {
        	if (ftable[i] >= EVENT) // Specials keys test
        	{
	            if (ftable[i] == tmp_code) { // Match the lookup table
	            	keyMemory(i,press); // Press or release the key
	                return;
	            }
        	}
        	else
        	{
        		if((tmp_char >= 'A') && (tmp_char <= 'Z')) // Uppercase test
        			tmp_char = tmp_char + 'a' - 'A'; // Convert to lowercase
	            if (ftable[i] == tmp_char) { // Match the lookup table
	            	keyMemory(i,press); // Press or release the key
	                return;
	            }
        	}       		
        }
            
    }
    
    public void keyPressed(KeyEvent e) {
    	
        int i;
        for (i = 0; i < 127; i++) 
                mem.remKey(i);
    	keyTranslator(e, true);
        
    }

    public void keyReleased(KeyEvent e) {

        
    	keyTranslator(e, false);
    }
    
    int shiftpressed = 0;

    public void press(int tmp) {
        if (tmp == (int) 'z') {
            shiftpressed++;
            tmp = 16;
        }
        if (tmp == (int) 'x') {
            tmp = 50;
        }
        if (shiftpressed == 2){
            shiftpressed = 0;
            return;
        }

        int i;
        for (i = 0; i < 127; i++) {
            if (ftable[i] == tmp) {
                mem.setKey(i);
                return;
            }
        }
    }
    boolean presstwice;
    boolean releasetwice;

  public void release(int tmp) {
        if (tmp == (int) 'z') {
        if (shiftpressed == 1)
            return;
            tmp = 16;
        }
        if (tmp == (int) 'x') {
            tmp = 50;
        }
        int i;
        for (i = 0; i < 127; i++) {
            if (ftable[i] == tmp) {
                mem.remKey(i);
                return;
            }
        }
    }
}
