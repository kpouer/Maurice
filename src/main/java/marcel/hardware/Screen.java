package marcel.hardware;

import lombok.Getter;
import lombok.Setter;

import java.awt.*;
import java.awt.event.*;
import java.awt.image.BufferedImage;
import java.awt.image.WritableRaster;
import java.net.URL;
import java.util.Arrays;

public class Screen extends Canvas {
    private final BufferedImage BuffImg;
    private final WritableRaster raster;
    @Getter
    private boolean mouse_clic;
    @Getter
    private int mouse_X = -1;
    @Getter
    private int mouse_Y = -1;
    private final int[] pixels;
    private Memory mem;
    @Getter
    private double pixelSize;
    @Getter
    @Setter
    private boolean filter;
    private int j;
    private int xPosition;
    private int yPosition;
    private int w;
    private final int[] palette = {
        0x000000,
        0xF00000,
        0x00F000,
        0xF0F000,

        0x0000F0,
        0xF000F0,
        0x00F0F0,
        0xF0F0F0,

        0x636363,
        0xF06363,
        0x63F063,
        0xF0F063,

        0x0063F0,
        0xF063F0,
        0x63F0F0,
        0xF06300,
    };

    @Setter
    private static int led;
    @Setter
    private static int show_led;
    private final URL monito = getClass().getResource("mask.png");
    private final Image monitor = getToolkit().getImage(monito);

    public Screen() {
        pixelSize = 2;
        BuffImg = new BufferedImage(320, 200, BufferedImage.TYPE_INT_RGB);
        raster = BuffImg.getRaster();
        pixels = new int[320 * 200];
        Arrays.fill(pixels, 0xff000000);
        BuffImg.setData(raster);

        // Mouse Event use for Lightpen emulation

        MouseMotionListener _Motion = new MouseMotionAdapter() {
            @Override
            public void mouseDragged(MouseEvent e) {
                mouse_X = e.getX();
                mouse_Y = e.getY();
                mouse_X = (int) ((mouse_X) / pixelSize);
                mouse_Y = (int) ((mouse_Y) / pixelSize);
            }
        };

        addMouseMotionListener(_Motion);
        MouseListener _Click = new MouseAdapter() {
            @Override
            public void mousePressed(MouseEvent e) {
                mouse_X = e.getX();
                mouse_Y = e.getY();
                mouse_X = (int) ((mouse_X) / pixelSize);
                mouse_Y = (int) ((mouse_Y) / pixelSize);

                mouse_clic = true;

            }

            @Override
            public void mouseReleased(MouseEvent e) {
                mouse_clic = false;
            }
        };
        addMouseListener(_Click);
    }

    public void init(Memory memory) {
        mem = memory;
    }

    public void setPixelSize(double ps) {
        pixelSize = ps;
        mem.setAllDirty();
    }

    @Override
    public void update(Graphics gc) {
        paint(gc);
    }

    @Override
    public void paint(Graphics gc) {
        raster.setDataElements(0, 0, 320, 200, pixels);
        if (filter) {
            Graphics2D g2 = (Graphics2D) gc;
            g2.setRenderingHint(RenderingHints.KEY_INTERPOLATION,
                                RenderingHints.VALUE_INTERPOLATION_BILINEAR);
            gc = g2;
        }
        Graphics og = BuffImg.getGraphics();
        if (show_led > 0) {
            show_led--;
            if (led != 0) {
                og.setColor(Color.red);
            } else {
                og.setColor(Color.BLACK);
            }
            og.fillRect(320 - 16, 0, 16, 8);
        }
        dopaint(og);
        //if (filter)
        //    og.drawImage(monitor, 0, 0, 320,200,this);
        gc.drawImage(BuffImg, 0, 0, (int) (320 * pixelSize), (int) (200 * pixelSize), this);
    }

    private void dopaint(Graphics gc) {
        int p = 0;
        int i = 0;

        for (int y = 0; y < 200; y++) {
            int offset = y * 320;
            if (!mem.isDirty(y)) {
                i += 40;
            } else {
                int x = 0;
                for (int xx = 0; xx < 40; xx++) {
                    int col = mem.COLOR(i);
                    int c2 = col & 0x0F;
                    int c1 = col >> 4;
                    int cc2 = palette[c1];
                    int cc1 = palette[c2];

                    int pt = mem.POINT(i);
                    if ((0x80 & pt) != 0) {
                        pixels[x + offset] = cc2;
                    } else {
                        pixels[x + offset] = cc1;
                    }
                    x++;
                    if ((0x40 & pt) != 0) {
                        pixels[x + offset] = cc2;
                    } else {
                        pixels[x + offset] = cc1;
                    }
                    x++;
                    if ((0x20 & pt) != 0) {
                        pixels[x + offset] = cc2;
                    } else {
                        pixels[x + offset] = cc1;
                    }
                    x++;
                    if ((0x10 & pt) != 0) {
                        pixels[x + offset] = cc2;
                    } else {
                        pixels[x + offset] = cc1;
                    }
                    x++;
                    if ((0x08 & pt) != 0) {
                        pixels[x + offset] = cc2;
                    } else {
                        pixels[x + offset] = cc1;
                    }
                    x++;
                    if ((0x04 & pt) != 0) {
                        pixels[x + offset] = cc2;
                    } else {
                        pixels[x + offset] = cc1;
                    }
                    x++;
                    if ((0x02 & pt) != 0) {
                        pixels[x + offset] = cc2;
                    } else {
                        pixels[x + offset] = cc1;
                    }
                    x++;
                    if ((0x01 & pt) != 0) {
                        pixels[x + offset] = cc2;
                    } else {
                        pixels[x + offset] = cc1;
                    }
                    x++;
                    i++;
                }
            }
        }
    }
}
