package hardware;

import java.awt.*;
import java.awt.event.*;

public class Keyboard implements KeyListener {

    private Canvas screen;
    private Memory mem;
    // translation table from scancode to java keycodes VK_
    private int[] ftable;

    public Keyboard(Canvas screen, Memory mem) {
        this.screen = screen;
        this.mem = mem;
        screen.addKeyListener(this);
        int i;
        char c;
        this.ftable = new int[128];

        for (i = 0; i < 128; i++) {
            ftable[i] = 0;
        }
        /* STOP */
        //ftable[0x6E]=0x29;
  /* 1 .. ACC */
        ftable[0x5E] = KeyEvent.VK_1;
        ftable[0x4E] = KeyEvent.VK_2;
        ftable[0x3E] = KeyEvent.VK_3;
        ftable[0x2E] = KeyEvent.VK_4;
        ftable[0x1E] = KeyEvent.VK_5;
        ftable[0x0E] = KeyEvent.VK_6;
        ftable[0x0C] = KeyEvent.VK_7;
        ftable[0x1C] = KeyEvent.VK_8;
        ftable[0x2C] = KeyEvent.VK_9;
        ftable[0x3C] = KeyEvent.VK_0;
        ftable[0x4C] = KeyEvent.VK_MINUS;
        ftable[0x5C] = KeyEvent.VK_EQUALS;
        ftable[0x6C] = KeyEvent.VK_BACK_SPACE;
        /* A .. --> */
        ftable[0x5A] = KeyEvent.VK_A;
        ftable[0x4A] = KeyEvent.VK_Z;
        ftable[0x3A] = KeyEvent.VK_E;
        ftable[0x2A] = KeyEvent.VK_R;
        ftable[0x1A] = KeyEvent.VK_T;
        ftable[0x0A] = KeyEvent.VK_Y;
        ftable[0x08] = KeyEvent.VK_U;
        ftable[0x18] = KeyEvent.VK_I;
        ftable[0x28] = KeyEvent.VK_O;
        ftable[0x38] = KeyEvent.VK_P;
        ftable[0x48] = KeyEvent.VK_BRACELEFT;
        ftable[0x58] = KeyEvent.VK_BRACERIGHT;
        /* Q .. enter */
        ftable[0x56] = KeyEvent.VK_Q;
        ftable[0x46] = KeyEvent.VK_S;
        ftable[0x36] = KeyEvent.VK_D;
        ftable[0x26] = KeyEvent.VK_F;
        ftable[0x16] = KeyEvent.VK_G;
        ftable[0x06] = KeyEvent.VK_H;
        ftable[0x04] = KeyEvent.VK_J;
        ftable[0x14] = KeyEvent.VK_K;
        ftable[0x24] = KeyEvent.VK_L;
        ftable[0x34] = KeyEvent.VK_M;
        ftable[0x68] = KeyEvent.VK_ENTER;
        /* W .. , */
        ftable[0x60] = KeyEvent.VK_Z;
        ftable[0x50] = KeyEvent.VK_X;
        ftable[0x64] = KeyEvent.VK_C;
        ftable[0x54] = KeyEvent.VK_V;
        ftable[0x44] = KeyEvent.VK_B;
        ftable[0x00] = KeyEvent.VK_N;
        ftable[0x10] = KeyEvent.VK_COMMA;
        ftable[0x20] = KeyEvent.VK_COLON;
        ftable[0x30] = KeyEvent.VK_STOP;
        ftable[0x58] = KeyEvent.VK_SLASH;
        /* ins eff curseur */
        ftable[0x12] = KeyEvent.VK_INSERT;
        ftable[0x02] = KeyEvent.VK_DELETE;
        ftable[0x62] = KeyEvent.VK_UP;
        ftable[0x52] = KeyEvent.VK_LEFT;
        ftable[0x32] = KeyEvent.VK_RIGHT;
        ftable[0x42] = KeyEvent.VK_DOWN;
        /* espace */
        ftable[0x40] = KeyEvent.VK_SPACE;
        /* SHIFT + BASIC */
        ftable[0x70] = KeyEvent.VK_SHIFT;
        ftable[0x72] = KeyEvent.VK_CONTROL;
        /* CNT RAZ */
        ftable[0x6A] = KeyEvent.VK_TAB;
        ftable[0x66] = KeyEvent.VK_CAPS_LOCK;

    }

    public void keyTyped(KeyEvent e) {
    }

    public void keyPressed(KeyEvent e) {
        int tmp = e.getKeyCode();
        int i;
        for (i = 0; i < 127; i++) {
            if (ftable[i] == tmp) {
                mem.setKey(i);
                return;
            }
        }
    }

    public void keyReleased(KeyEvent e) {
        int tmp = e.getKeyCode();
        int i;
        for (i = 0; i < 127; i++) {
            if (ftable[i] == tmp) {
                mem.remKey(i);
                return;
            }
        }
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
