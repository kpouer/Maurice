package marcel.hardware;

import java.awt.*;
import java.net.URL;
import java.awt.event.MouseEvent;
import java.awt.event.MouseListener;
import java.awt.event.MouseMotionListener;
import java.awt.image.*;

public class Screen extends Canvas {
	private static final long serialVersionUID = 1L;
	BufferedImage BuffImg;
    WritableRaster raster;
	public boolean mouse_clic = false ;
    public int mouse_X = -1,mouse_Y = -1;	
    int[] pixels;
    protected Memory mem;
    public double pixelSize;
    private Graphics og;
    public boolean filter = false;
    private final int palette[] = {
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
        
        // Mouse Event use for Lightpen emulation
        MouseListener _Click = new MouseListener() {
            public void mouseClicked(MouseEvent e) {
            }

            public void mouseEntered(MouseEvent e) {
            }

            public void mouseExited(MouseEvent e) {
            }

            public void mousePressed(MouseEvent e) {
                mouse_X = e.getX();
                mouse_Y = e.getY();
                mouse_X = (int)((mouse_X ) / pixelSize);
                mouse_Y = (int)((mouse_Y ) / pixelSize);
               
                mouse_clic = true;
                
            }
            
            public void mouseReleased(MouseEvent e) {
            	mouse_clic = false;
            	
              }
        };
            
            MouseMotionListener _Motion = new MouseMotionListener() {
                public void mouseDragged(MouseEvent e) {
                  mouse_X = e.getX();
                  mouse_Y = e.getY();
                  mouse_X = (int)((mouse_X ) / pixelSize);
                  mouse_Y = (int)((mouse_Y ) / pixelSize);
                }

                public void mouseMoved(MouseEvent e) {
                	
                }
              };

              
        this.addMouseMotionListener(_Motion);
        this.addMouseListener(_Click);

    }

    public void init(Memory memory) {
        this.mem = memory;
    }

    public void setPixelSize(double ps) {
        pixelSize = ps;
        mem.setAllDirty();
    }

    public double getPixelSize() {
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
        //if (filter)
        //    og.drawImage(monitor, 0, 0, 320,200,this);
        gc.drawImage(BuffImg, 0, 0, (int)(320 * pixelSize), (int)(200 * pixelSize), this);
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
