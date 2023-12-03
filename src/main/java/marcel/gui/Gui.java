package marcel.gui;

import marcel.hardware.Machine;
import marcel.hardware.Screen;

import javax.swing.*;
import java.awt.*;
import java.awt.event.ActionEvent;
import java.awt.event.ActionListener;
import java.awt.event.WindowAdapter;
import java.awt.event.WindowEvent;

public class Gui extends WindowAdapter implements ActionListener {

    private JFrame guiFrame;
    private String hadfile;

    private JMenuItem guiMenuFileSelectK7;
    private JMenuItem guiMenuRewindK7;
    private JMenuItem guiMenuFileExit;
    private JMenuItem guiMenuRunStop;
    private JMenuItem guiMenuRunGo;
    private JMenuItem guiMenuResetSoft;
    private JMenuItem guiMenuResetHard;
    private JMenuItem guiMenuZoomx1;
    private JMenuItem guiMenuZoomx2;
    private JMenuItem guiMenuZoomx3;
    private JMenuItem guiFilter;
    private JMenuItem guiMenuDebugDebug;
    private JMenuItem guiMenuHelpAbout;
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

        guiMenuFileSelectK7 = new JMenuItem("Select K7");
        guiMenuFileSelectK7.addActionListener(this);
        guiMenuFile.add(guiMenuFileSelectK7);
        guiMenuRewindK7 = new JMenuItem("Rewind tape");
        guiMenuRewindK7.addActionListener(this);
        guiMenuFile.add(guiMenuRewindK7);
        guiMenuFileExit = new JMenuItem("Exit");
        guiMenuFileExit.addActionListener(this);
        guiMenuFile.add(guiMenuFileExit);

        guiMenuBar.add(guiMenuFile);

        var guiMenuRun = new JMenu("Run");
        guiMenuRunStop = new JMenuItem("Stop");
        guiMenuRunStop.addActionListener(this);
        guiMenuRun.add(guiMenuRunStop);
        guiMenuRunGo = new JMenuItem("Go");
        guiMenuRunGo.addActionListener(this);
        guiMenuRun.add(guiMenuRunGo);

        guiMenuBar.add(guiMenuRun);

        var guiMenuReset = new JMenu("Reset");
        guiMenuResetSoft = new JMenuItem("Soft Reset");
        guiMenuResetSoft.addActionListener(this);
        guiMenuReset.add(guiMenuResetSoft);
        guiMenuResetHard = new JMenuItem("Hard Reset");
        guiMenuResetHard.addActionListener(this);
        guiMenuReset.add(guiMenuResetHard);

        guiMenuBar.add(guiMenuReset);

        var guiMenuZoom = new JMenu("Image");
        guiMenuZoomx1 = new JMenuItem("Zoom x 1");
        guiMenuZoomx1.addActionListener(this);
        guiMenuZoom.add(guiMenuZoomx1);
        guiMenuZoomx2 = new JMenuItem("Zoom x 2");
        guiMenuZoomx2.addActionListener(this);
        guiMenuZoom.add(guiMenuZoomx2);
        guiMenuZoomx3 = new JMenuItem("Zoom x 3");
        guiMenuZoomx3.addActionListener(this);
        guiMenuZoom.add(guiMenuZoomx3);
        guiFilter = new JMenuItem("Filter");
        guiFilter.addActionListener(this);
        guiMenuZoom.add(guiFilter);

        guiMenuBar.add(guiMenuZoom);

        var guiMenuDebug = new JMenu("Debug");
        guiMenuDebugDebug = new JMenuItem("Debug");
        guiMenuDebugDebug.addActionListener(this);
        guiMenuDebug.add(guiMenuDebugDebug);

        guiMenuBar.add(guiMenuDebug);

        var guiMenuHelp = new JMenu("Help");
        guiMenuHelpAbout = new JMenuItem("About");
        guiMenuHelpAbout.addActionListener(this);
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