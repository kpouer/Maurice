package marcel.gui;

import marcel.hardware.Machine;
import marcel.hardware.Screen;

import javax.swing.*;
import java.awt.*;
import java.awt.event.WindowAdapter;
import java.awt.event.WindowEvent;

public class Gui extends WindowAdapter {

    private JFrame guiFrame;
    private String hadfile;

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

    private void zoom(int ps) {
        screen.setPixelSize(ps);
        var insets = guiFrame.getInsets();
        guiFrame.setSize((int) (320 * ps + (insets.left + insets.right)), (int) (200 * ps + (insets.top + insets.bottom)));
        screen.repaint();
    }


    @Override
    public void windowClosing(WindowEvent e) {
        if (guiFrame.equals(e.getSource())) {
            System.exit(0);
        }
    }

    @Override
    public void windowActivated(WindowEvent e) {
        try {
            if (guiFrame.equals(e.getSource())) {
                guiFrame.toFront();
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
    }

    private void initGui() {
        guiFrame = new JFrame("Marcel O Cinq 3.1 (Java)");
        guiFrame.setLayout(new BorderLayout());

        var guiMenuBar = new JMenuBar();

        var guiMenuFile = new JMenu("File");

        var guiMenuFileSelectK7 = new JMenuItem("Select K7");
        guiMenuFileSelectK7.addActionListener(e -> setK7());
        guiMenuFile.add(guiMenuFileSelectK7);
        var guiMenuRewindK7 = new JMenuItem("Rewind tape");
        guiMenuRewindK7.addActionListener(e -> {
            if (hadfile != null) {
                if (hadfile.startsWith("http:")) {
                    setK7FromUrl(hadfile);
                } else {
                    setK7(hadfile);
                }
            } else {
                System.err.println("No file!");
            }
        });
        guiMenuFile.add(guiMenuRewindK7);
        var guiMenuFileExit = new JMenuItem("Exit");
        guiMenuFileExit.addActionListener(e -> System.exit(0));
        guiMenuFile.add(guiMenuFileExit);

        guiMenuBar.add(guiMenuFile);

        var guiMenuRun = new JMenu("Run");
        var guiMenuRunStop = new JMenuItem("Stop");
        guiMenuRunStop.addActionListener(e ->  machine.stop());
        guiMenuRun.add(guiMenuRunStop);
        var guiMenuRunGo = new JMenuItem("Go");
        guiMenuRunGo.addActionListener(e -> machine.start());
        guiMenuRun.add(guiMenuRunGo);

        guiMenuBar.add(guiMenuRun);

        var guiMenuReset = new JMenu("Reset");
        var guiMenuResetSoft = new JMenuItem("Soft Reset");
        guiMenuResetSoft.addActionListener(e -> {
            machine.stop();
            machine.resetSoft();
            machine.start();
        });
        guiMenuReset.add(guiMenuResetSoft);
        var guiMenuResetHard = new JMenuItem("Hard Reset");
        guiMenuResetHard.addActionListener(e -> {
            machine.stop();
            machine.resetHard();
            machine.start();
        });
        guiMenuReset.add(guiMenuResetHard);

        guiMenuBar.add(guiMenuReset);

        var guiMenuZoom = new JMenu("Image");
        var guiMenuZoomx1 = new JMenuItem("Zoom x 1");
        guiMenuZoomx1.addActionListener(e -> zoom(1));
        guiMenuZoom.add(guiMenuZoomx1);
        var guiMenuZoomx2 = new JMenuItem("Zoom x 2");
        guiMenuZoomx2.addActionListener(e -> zoom(2));
        guiMenuZoom.add(guiMenuZoomx2);
        var guiMenuZoomx3 = new JMenuItem("Zoom x 3");
        guiMenuZoomx3.addActionListener(e -> zoom(4));
        guiMenuZoom.add(guiMenuZoomx3);
        var guiFilter = new JMenuItem("Filter");
        guiFilter.addActionListener(e -> screen.setFilter(!screen.isFilter()));
        guiMenuZoom.add(guiFilter);

        guiMenuBar.add(guiMenuZoom);

        var guiMenuDebug = new JMenu("Debug");
        var guiMenuDebugDebug = new JMenuItem("Debug");
        guiMenuDebugDebug.addActionListener(e -> debug());
        guiMenuDebug.add(guiMenuDebugDebug);

        guiMenuBar.add(guiMenuDebug);

        var guiMenuHelp = new JMenu("Help");
        var guiMenuHelpAbout = new JMenuItem("About");
        guiMenuHelpAbout.addActionListener(e -> about());
        guiMenuHelp.add(guiMenuHelpAbout);

        guiMenuBar.add(guiMenuHelp);

        guiFrame.addWindowListener(this);
        guiFrame.setJMenuBar(guiMenuBar);
        guiFrame.add(screen);


        guiFrame.pack();

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

        var ta = new JTextArea(aboutText, 30, 40);
        ta.setEditable(false);
        ta.setBackground(Color.WHITE);
        var guiDialog = new JDialog(guiFrame, true);
        guiDialog.getContentPane().add(new JScrollPane(ta));
        guiDialog.setTitle("About Marcel O Cinq");
        guiDialog.setSize(400, 500);
        guiDialog.setVisible(true);
    }

    private void debug() {
        var t1 = new JTextArea(machine.dumpRegisters(), 2, 40);
        t1.setEditable(false);
        t1.setBackground(Color.WHITE);
        var t2 = new JTextArea(machine.unassembleFromPC(10), 10, 40);
        t2.setEditable(false);
        t2.setBackground(Color.WHITE);
        var debugDialog = new JDialog(guiFrame, true);
        debugDialog.add(t1);
        debugDialog.add(t2);
        debugDialog.setLayout(new FlowLayout());
        debugDialog.setSize(400, 400);
        debugDialog.setVisible(true);
    }
}