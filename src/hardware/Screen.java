package hardware;

import java.awt.*;
import java.net.URL;
import java.awt.image.*;

public class Screen extends Canvas {

    BufferedImage BuffImg;
    WritableRaster raster;
    int[] pixels;
    protected Memory mem;
    private int pixelSize;
    private Graphics og;
    public boolean filter = false;
    private final int palette[] = {
        0x000000,
        0xFF0000,
        0x00FF00,
        0xFFFF00,

        0x0000FF,
        0xFF00FF,
        0x00FFFF,
        0xFFFFFF,

        0x636363,
        0xFF6363,
        0x63FF63,
        0xFFFF63,

        0x0063FF,
        0xFF63FF,
        0x63FFFF,
        0xFF6300,
    };

    public static int led = 0;
    public static int show_led = 0;
    final URL monito = getClass().getResource("mask.png");
    final Image monitor = getToolkit().getImage(monito);

    public Screen() {
        this.pixelSize = 2;
        BuffImg = new BufferedImage(320, 200, BufferedImage.TYPE_INT_RGB);
        raster = BuffImg.getRaster();
        pixels = new int[320 * 200];
        for (int i = 0; i < pixels.length; i++) {
            pixels[i] = 0xff000000;
        }
        BuffImg.setData(raster);
    }

    public void init(Memory memory) {
        this.mem = memory;
    }

    public void setPixelSize(int ps) {
        pixelSize = ps;
        mem.setAllDirty();
    }

    public int getPixelSize() {
        return pixelSize;
    }

    public void update(Graphics gc) {
        paint(gc);
    }

    public void paint(Graphics gc) {
        raster.setDataElements(0, 0, 320, 200, pixels);
        if (filter) {
            Graphics2D g2 = (Graphics2D) gc;
            g2.setRenderingHint(RenderingHints.KEY_INTERPOLATION,
                    RenderingHints.VALUE_INTERPOLATION_BILINEAR);
            gc = g2;
        }
            og = BuffImg.getGraphics();
            if (show_led > 0) {
                show_led--;
                if (led != 0)
                og.setColor(Color.red);
                else
                og.setColor(Color.BLACK);
                og.fillRect(320 - 16, 0, 16, 8);
            }
        dopaint(og);
        if (filter)
            og.drawImage(monitor, 0, 0, 320,200,this);
        gc.drawImage(BuffImg, 0, 0, 320 * pixelSize, 200 * pixelSize, this);
    }
    int i;
    int j;
    int xPosition;
    int yPosition;
    int col;
    int pt;
    int c1;
    int c2;
    int w;
    int p;
    int xx;
    int x;
    int y;
    int offset;

    public void dopaint(Graphics gc) {
        p = 0;
        i = 0;

        for (y = 0; y < 200; y++) {
            offset = y*320;
            x = 0;
            if (!mem.isDirty(y)) {
                i += 40;
            } else {
                for (xx = 0; xx < 40; xx++) {
                    int cc1;
                    int cc2;
                    col = mem.COLOR(i);
                    c2 = col & 0x0F;
                    c1 = col >> 4;
                    cc2 = palette[c1];
                    cc1 = palette[c2];

                    pt = mem.POINT(i);
                    if ((0x80 & pt) != 0) {
                        pixels[x+offset] = cc2;
                    } else {
                        pixels[x+offset] = cc1;
                    }
                    x++;
                    if ((0x40 & pt) != 0) {
                        pixels[x+offset] = cc2;
                    } else {
                        pixels[x+offset] = cc1;
                    }
                    x++;
                    if ((0x20 & pt) != 0) {
                        pixels[x+offset] = cc2;
                    } else {
                        pixels[x+offset] = cc1;
                    }
                    x++;
                    if ((0x10 & pt) != 0) {
                        pixels[x+offset] = cc2;
                    } else {
                        pixels[x+offset] = cc1;
                    }
                    x++;
                    if ((0x08 & pt) != 0) {
                        pixels[x+offset] = cc2;
                    } else {
                        pixels[x+offset] = cc1;
                    }
                    x++;
                    if ((0x04 & pt) != 0) {
                        pixels[x+offset] = cc2;
                    } else {
                        pixels[x+offset] = cc1;
                    }
                    x++;
                    if ((0x02 & pt) != 0) {
                        pixels[x+offset] = cc2;
                    } else {
                        pixels[x+offset] = cc1;
                    }
                    x++;
                    if ((0x01 & pt) != 0) {
                        pixels[x+offset] = cc2;
                    } else {
                        pixels[x+offset] = cc1;
                    }
                    x++;
                    i++;
                }
            }
        }


    }
}
