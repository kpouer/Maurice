package marcel.gui;

import marcel.hardware.Machine;
import marcel.hardware.Screen;

import java.awt.*;
import java.awt.event.ActionEvent;
import java.awt.event.ActionListener;
import java.awt.event.WindowAdapter;
import java.awt.event.WindowEvent;

public class Gui extends WindowAdapter implements ActionListener {

    private Frame guiFrame;
    private String hadfile;

    private MenuItem guiMenuFileSelectK7;
    private MenuItem guiMenuRewindK7;
    private MenuItem guiMenuFileExit;
    private MenuItem guiMenuRunStop;
    private MenuItem guiMenuRunGo;
    private MenuItem guiMenuResetSoft;
    private MenuItem guiMenuResetHard;
    private MenuItem guiMenuZoomx1;
    private MenuItem guiMenuZoomx2;
    private MenuItem guiMenuZoomx3;
    private MenuItem guiFilter;
    private MenuItem guiMenuDebugDebug;
    private MenuItem guiMenuHelpAbout;
    private Dialog guiDialog;
    private Dialog debugDialog;
    private Screen screen;
    private Machine machine;
    private String lastK7Dir;

    public Gui() {
        initMachine();
        initGui();
    }

    private void setK7FromUrl(String url) {
        machine.setK7FileFromUrl(url);
        hadfile = url;
        Machine.setTesttimer(1);
    }

    private void setK7(String filename) {
        machine.setK7File(filename);
    }

    private void setK7() {
        var filedia = new FileDialog(guiFrame, "Load file", FileDialog.LOAD);
        filedia.setVisible(true);
        String filename = filedia.getFile();
        if (filename != null) {
            filename = filedia.getDirectory() + filedia.getFile();
            machine.setK7File(filename);
            hadfile = filename;
        }
    }

    @Override
    public void actionPerformed(ActionEvent evt) {
        if (guiMenuFileSelectK7.equals(evt.getSource())) {
            setK7();
        }
        // Rewind the tape (Reload it)
        if (guiMenuRewindK7.equals(evt.getSource())) {
            if (hadfile != null) {
                if (hadfile.startsWith("http:")) {
                    setK7FromUrl(hadfile);
                } else {
                    setK7(hadfile);
                }
            } else {
                System.err.println("No file!");
            }
        }
        // Exit
        if (guiMenuFileExit.equals(evt.getSource())) {
            System.exit(0);
        }

        // Menu Run
        // Stop
        if (guiMenuRunStop.equals(evt.getSource())) {
            machine.stop();
            return;
        }
        // Go
        if (guiMenuRunGo.equals(evt.getSource())) {
            machine.start();
            return;
        }

        // Menu Reset
        // Soft
        if (guiMenuResetSoft.equals(evt.getSource())) {
            machine.stop();
            machine.resetSoft();
            machine.start();
            return;
        }
        // Hard
        if (guiMenuResetHard.equals(evt.getSource())) {
            machine.stop();
            machine.resetHard();
            machine.start();
            return;
        }

        // Screen filter
        //

        if (guiFilter.equals(evt.getSource())) {
            screen.setFilter(!screen.isFilter());
            return;
        }
        // Menu Zoom
        // X1
        if (guiMenuZoomx1.equals(evt.getSource())) {
            screen.setPixelSize(1);
            Insets i = guiFrame.getInsets();
            guiFrame.setSize((int) (320 * screen.getPixelSize() + (i.left + i.right)), (int) (200 * screen.getPixelSize() + (i.top + i.bottom)));
            screen.repaint();

            return;
        }
        // X2
        if (guiMenuZoomx2.equals(evt.getSource())) {
            screen.setPixelSize(2);
            Insets i = guiFrame.getInsets();
            guiFrame.setSize((int) (320 * screen.getPixelSize() + (i.left + i.right)), (int) (200 * screen.getPixelSize() + (i.top + i.bottom)));
            screen.repaint();

            return;
        }
        // X3
        if (guiMenuZoomx3.equals(evt.getSource())) {
            screen.setPixelSize(4);
            Insets i = guiFrame.getInsets();
            guiFrame.setSize((int) (320.0 * screen.getPixelSize() + (i.left + i.right)), (int) (200.0 * screen.getPixelSize() + (i.top + i.bottom)));
            screen.repaint();

            return;
        }

        if (guiMenuDebugDebug.equals(evt.getSource())) {
            debug();
            return;
        }

        if (guiMenuHelpAbout.equals(evt.getSource())) {
            about();
            return;
        }
    }


    @Override
    public void windowClosing(WindowEvent e) {
        if (guiFrame.equals(e.getSource())) {
            System.exit(0);
        }
        if (guiDialog.equals(e.getSource())) {
            guiDialog.dispose();
        }
        if (debugDialog.equals(e.getSource())) {
            debugDialog.dispose();
        }
    }

    @Override
    public void windowActivated(WindowEvent e) {
        try {
            if (guiFrame.equals(e.getSource())) {
                guiFrame.toFront();
            }
            if (guiDialog.equals(e.getSource())) {
                guiDialog.toFront();
            }
            if (debugDialog.equals(e.getSource())) {
                debugDialog.toFront();
            }
        } catch (Exception re) {
        }
    }

    @Override
    public void windowDeiconified(WindowEvent e) {
        if (guiFrame.equals(e.getSource())) {
            guiFrame.toFront();
        }
    }

    @Override
    public void windowOpened(WindowEvent e) {
        if (e == null) {
            return;
        }
        if (guiFrame != null) {
            if (guiFrame.equals(e.getSource())) {
                guiFrame.toFront();
            }
        }
        if (guiDialog != null) {
            if (guiDialog.equals(e.getSource())) {
                guiDialog.toFront();
            }
        }
        if (debugDialog != null) {
            if (debugDialog.equals(e.getSource())) {
                debugDialog.toFront();
            }
        }
    }

    private void initGui() {
        guiFrame = new Frame("Marcel O Cinq 3.1 (Java)");
        guiFrame.setLayout(new BorderLayout());

        var guiMenuBar = new MenuBar();

        var guiMenuFile = new Menu("File");

        guiMenuFileSelectK7 = new MenuItem("Select K7");
        guiMenuFileSelectK7.addActionListener(this);
        guiMenuFile.add(guiMenuFileSelectK7);
        guiMenuRewindK7 = new MenuItem("Rewind tape");
        guiMenuRewindK7.addActionListener(this);
        guiMenuFile.add(guiMenuRewindK7);
        guiMenuFileExit = new MenuItem("Exit");
        guiMenuFileExit.addActionListener(this);
        guiMenuFile.add(guiMenuFileExit);

        guiMenuBar.add(guiMenuFile);

        var guiMenuRun = new Menu("Run");
        guiMenuRunStop = new MenuItem("Stop");
        guiMenuRunStop.addActionListener(this);
        guiMenuRun.add(guiMenuRunStop);
        guiMenuRunGo = new MenuItem("Go");
        guiMenuRunGo.addActionListener(this);
        guiMenuRun.add(guiMenuRunGo);

        guiMenuBar.add(guiMenuRun);

        var guiMenuReset = new Menu("Reset");
        guiMenuResetSoft = new MenuItem("Soft Reset");
        guiMenuResetSoft.addActionListener(this);
        guiMenuReset.add(guiMenuResetSoft);
        guiMenuResetHard = new MenuItem("Hard Reset");
        guiMenuResetHard.addActionListener(this);
        guiMenuReset.add(guiMenuResetHard);

        guiMenuBar.add(guiMenuReset);

        var guiMenuZoom = new Menu("Image");
        guiMenuZoomx1 = new MenuItem("Zoom x 1");
        guiMenuZoomx1.addActionListener(this);
        guiMenuZoom.add(guiMenuZoomx1);
        guiMenuZoomx2 = new MenuItem("Zoom x 2");
        guiMenuZoomx2.addActionListener(this);
        guiMenuZoom.add(guiMenuZoomx2);
        guiMenuZoomx3 = new MenuItem("Zoom x 3");
        guiMenuZoomx3.addActionListener(this);
        guiMenuZoom.add(guiMenuZoomx3);
        guiFilter = new MenuItem("Filter");
        guiFilter.addActionListener(this);
        guiMenuZoom.add(guiFilter);

        guiMenuBar.add(guiMenuZoom);

        var guiMenuDebug = new Menu("Debug");
        guiMenuDebugDebug = new MenuItem("Debug");
        guiMenuDebugDebug.addActionListener(this);
        guiMenuDebug.add(guiMenuDebugDebug);

        guiMenuBar.add(guiMenuDebug);

        var guiMenuHelp = new Menu("Help");
        guiMenuHelpAbout = new MenuItem("About");
        guiMenuHelpAbout.addActionListener(this);
        guiMenuHelp.add(guiMenuHelpAbout);

        guiMenuBar.add(guiMenuHelp);

        guiFrame.addWindowListener(this);
        guiFrame.setMenuBar(guiMenuBar);
        guiFrame.add(screen);


        guiFrame.pack();

        guiDialog = new Dialog(guiFrame, true);
        guiDialog.addWindowListener(this);

        debugDialog = new Dialog(guiFrame, true);
        debugDialog.addWindowListener(this);

        screen.requestFocusInWindow();

        machine.start();

        screen.setPixelSize(1);
        Insets i = guiFrame.getInsets();
        guiFrame.setSize((int) (320 * screen.getPixelSize() + (i.left + i.right) + 10), (int) (200 * screen.getPixelSize() + (i.top + i.bottom) + 15));
        guiFrame.setVisible(true);
        screen.repaint();
        screen.setPixelSize(2);
        i = guiFrame.getInsets();
        guiFrame.setSize((int) (320 * screen.getPixelSize() + (i.left + i.right)), (int) (200 * screen.getPixelSize() + (i.top + i.bottom)));
        screen.repaint();
    }

    private void initMachine() {
        screen = new Screen();
        machine = new Machine(screen);
    }

    private void about() {
        String aboutText = """
                       Marcel O Cinq 3.1 (java)\s

            (C) G.Fetis 1997-1998-2006
            (C) DevilMarkus http://cpc.devilmarkus.de 2006
            (C) M.Le Goff 2014

            Java conversion of my previously C/DOS
            based Thomson MO5 emulator\s
            (that was also ported to Unix and Macos)
            The basic java design is taken from Pom1
            (Apple1 java emulator (that derives from
            Microtan java (an obscure british Oric ancestor))
            this program is under GPL licence
            to load a K7 program:
            File->Load a K7 : to select the file (uncompressed)
            under Basic interpreter type LOAD then type RUN
            or LOADM then EXEC

            Full keyboard emulation with all symbols
            Sound emulation
            Reset bug solved
            Save K7 emulation
            Lightpen emulation
            AltGr+C = Ctrl+C = Break basic
            F11 = BASIC     F12 = SHIFT

            Contacts :
            gilles.fetis@wanadoo.fr
            marc.le.goff@gmail.fr
            """;

        TextArea ta = new TextArea(aboutText, 30, 40, TextArea.SCROLLBARS_VERTICAL_ONLY);
        ta.setEditable(false);
        ta.setBackground(Color.WHITE);

        guiDialog.removeAll();
        guiDialog.setTitle("About Marcel O Cinq");
        guiDialog.setLayout(new FlowLayout());
        guiDialog.add(ta);
        guiDialog.setSize(400, 500);
        guiDialog.setVisible(true);
    }

    private void debug() {
        TextArea t1 = new TextArea(machine.dumpRegisters(), 2, 40, TextArea.SCROLLBARS_NONE);
        t1.setEditable(false);
        t1.setBackground(Color.WHITE);
        TextArea t2 = new TextArea(machine.unassembleFromPC(10), 10, 40, TextArea.SCROLLBARS_NONE);
        t2.setEditable(false);
        t2.setBackground(Color.WHITE);
        debugDialog.removeAll();
        debugDialog.add(t1);
        debugDialog.add(t2);
        debugDialog.setLayout(new FlowLayout());
        debugDialog.setSize(400, 400);
        debugDialog.setVisible(true);
    }
}