package gui;

import hardware.*;

import java.awt.*;
import java.awt.event.*;
import java.io.*;

import java.net.URL;

public class Gui implements WindowListener, ActionListener {

    public Frame guiFrame;
    public String hadfile = null;
    private MenuBar guiMenuBar;
    private Menu guiMenuFile, guiMenuRun, guiMenuReset, guiMenuZoom, guiMenuDebug, guiMenuHelp;
    private MenuItem guiMenuFileSelectK7, guiMenuUrlSelectK7, guiMenuRewindK7, guiMenuFileExit;
    private MenuItem guiMenuRunStop, guiMenuRunGo;
    private MenuItem guiMenuResetSoft, guiMenuResetHard;
    private MenuItem guiMenuZoomx1, guiMenuZoomx2, guiMenuZoomx3;
    private MenuItem guiFilter;
    private MenuItem guiMenuDebugDebug;
    private MenuItem guiMenuHelpAbout;
    private Dialog guiDialog;
    private Dialog debugDialog;
    public Screen screen;
    public Machine machine;
    private boolean appletMode = false;
    private URL appletCodeBase;

    public Gui() {
        appletMode = false;
        initMachine();
        initGui();
    }

    public Gui(URL appletCodeBase) {
        appletMode = true;
        this.appletCodeBase = appletCodeBase;
        initMachine();
        initGui();
    }
    boolean usefram = true;

    public Gui(URL appletCodeBase, boolean useframe) {
        usefram = useframe;
        appletMode = true;
        this.appletCodeBase = appletCodeBase;
        initMachine();
        initGui();
    }
    private String lastK7Dir = null;

    public void setK7FromUrl(String url) {
        machine.setK7FileFromUrl(url);
        hadfile = url;
        machine.testtimer = 1;
    }

    public void setK7(String filename) {
        machine.setK7File(filename);
    }

    public void setK7() {
        Frame dummy = new Frame();
        FileDialog filedia = new FileDialog((Frame) dummy,
                "Load file", FileDialog.LOAD);
        filedia.setVisible(true);
        String filename = filedia.getFile();
        if (filename != null) {
            filename = filedia.getDirectory() + filedia.getFile();
            machine.setK7File(filename);
            hadfile = filename;
        }
    }

    public void actionPerformed(ActionEvent evt) {

        // Menu File
        // select K7 (file mode)
/*	    if (guiMenuFileSelectK7.equals(evt.getSource())) {
        FileDialog fd=new FileDialog(guiFrame,"Select a K7 File for read",FileDialog.LOAD);
        if (lastK7Dir!=null) fd.setDirectory(lastK7Dir);
        fd.show();
        if (fd.getFile()!=null) {
        machine.setK7File(fd.getDirectory()+"/"+fd.getFile());
        lastK7Dir=fd.getDirectory();
        }
        } */

        // select an url for a K7
        if (guiMenuUrlSelectK7.equals(evt.getSource())) {
            Browser b = new Browser("http://perso.orange.fr/gilles.fetis/emu/java/k7/index.htm", this);
        }
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
            machine.resetSoft();
            return;
        }
        // Hard
        if (guiMenuResetHard.equals(evt.getSource())) {
            machine.resetHard();
            return;
        }

        // Screen filter
        //

        if (guiFilter.equals(evt.getSource())) {
            if (screen.filter) {
                screen.filter = false;
            } else {
                screen.filter = true;
            }
            return;
        }
        // Menu Zoom
        // X1
        if (guiMenuZoomx1.equals(evt.getSource())) {
            screen.setPixelSize(1);
            Insets i = guiFrame.getInsets();
            guiFrame.setSize(320 * screen.getPixelSize() + (i.left + i.right), 200 * screen.getPixelSize() + (i.top + i.bottom));
            screen.repaint();
            return;
        }
        // X2
        if (guiMenuZoomx2.equals(evt.getSource())) {
            screen.setPixelSize(2);
            Insets i = guiFrame.getInsets();
            guiFrame.setSize(320 * screen.getPixelSize() + (i.left + i.right), 200 * screen.getPixelSize() + (i.top + i.bottom));
            screen.repaint();
            return;
        }
        // X3
        if (guiMenuZoomx3.equals(evt.getSource())) {
            screen.setPixelSize(3);
            Insets i = guiFrame.getInsets();
            guiFrame.setSize(320 * screen.getPixelSize() + (i.left + i.right), 200 * screen.getPixelSize() + (i.top + i.bottom));
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

    public void windowClosed(WindowEvent e) {
    }

    public void windowDeactivated(WindowEvent e) {
    }

    public void windowDeiconified(WindowEvent e) {
        if (guiFrame.equals(e.getSource())) {
            guiFrame.toFront();
        }
    }

    public void windowIconified(WindowEvent e) {
    }

    public void windowOpened(WindowEvent e) {
        if (guiFrame.equals(e.getSource())) {
            guiFrame.toFront();
        }
        if (guiDialog.equals(e.getSource())) {
            guiDialog.toFront();
        }
        if (debugDialog.equals(e.getSource())) {
            debugDialog.toFront();
        }
    }

    private void initGui() {
        guiFrame = new Frame("Marcel O Cinq 3.0 (Java)");
        guiFrame.setLayout(new BorderLayout());

        guiMenuBar = new MenuBar();

        guiMenuFile = new Menu("File");

        guiMenuFileSelectK7 = new MenuItem("Select K7 (application mode)");
        guiMenuFileSelectK7.addActionListener(this);
        if (!appletMode) {
            guiMenuFile.add(guiMenuFileSelectK7);
        }
        guiMenuUrlSelectK7 = new MenuItem("Select K7 via URL (applet mode)");
        guiMenuUrlSelectK7.addActionListener(this);
        //if(!appletMode)
        guiMenuFile.add(guiMenuUrlSelectK7);
        guiMenuRewindK7 = new MenuItem("Rewind tape");
        guiMenuRewindK7.addActionListener(this);
        guiMenuFile.add(guiMenuRewindK7);
        guiMenuFileExit = new MenuItem("Exit");
        guiMenuFileExit.addActionListener(this);
        guiMenuFile.add(guiMenuFileExit);

        guiMenuBar.add(guiMenuFile);

        guiMenuRun = new Menu("Run");
        guiMenuRunStop = new MenuItem("Stop");
        guiMenuRunStop.addActionListener(this);
        guiMenuRun.add(guiMenuRunStop);
        guiMenuRunGo = new MenuItem("Go");
        guiMenuRunGo.addActionListener(this);
        guiMenuRun.add(guiMenuRunGo);

        guiMenuBar.add(guiMenuRun);

        guiMenuReset = new Menu("Reset");
        guiMenuResetSoft = new MenuItem("Soft Reset");
        guiMenuResetSoft.addActionListener(this);
        guiMenuReset.add(guiMenuResetSoft);
        guiMenuResetHard = new MenuItem("Hard Reset");
        guiMenuResetHard.addActionListener(this);
        guiMenuReset.add(guiMenuResetHard);

        guiMenuBar.add(guiMenuReset);

        guiMenuZoom = new Menu("Image");
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

        guiMenuDebug = new Menu("Debug");
        guiMenuDebugDebug = new MenuItem("Debug");
        guiMenuDebugDebug.addActionListener(this);
        guiMenuDebug.add(guiMenuDebugDebug);

        guiMenuBar.add(guiMenuDebug);

        guiMenuHelp = new Menu("Help");
        guiMenuHelpAbout = new MenuItem("About");
        guiMenuHelpAbout.addActionListener(this);
        guiMenuHelp.add(guiMenuHelpAbout);

        guiMenuBar.add(guiMenuHelp);

        guiFrame.addWindowListener(this);
        guiFrame.setMenuBar(guiMenuBar);
        guiFrame.add(screen);

        guiFrame.pack();
        Insets i = guiFrame.getInsets();
        guiFrame.setSize(320 * screen.getPixelSize() + (i.left + i.right), 200 * screen.getPixelSize() + (i.top + i.bottom));
        if (usefram) {
            guiFrame.setVisible(true);
        }

        guiDialog = new Dialog(guiFrame, true);
        guiDialog.addWindowListener(this);

        debugDialog = new Dialog(guiFrame, true);
        debugDialog.addWindowListener(this);


        machine.start();
    }

    private void initMachine() {
        screen = new Screen();
        if (appletMode) {
            machine = new Machine(screen, appletCodeBase);
        } else {
            machine = new Machine(screen);
        }
    }

    private void about() {


        String aboutText = " Marcel O Cinq 3.0 (java) \n"
                + "(C) G.Fetis 1997-1998-2006\n"
                + "java conversion of my previously C/DOS\n"
                + "based Thomson MO5 emulator \n"
                + "(that was also ported to Unix and Macos)\n"
                + "The basic java design is taken from Pom1\n"
                + "(Apple1 java emulator (that derives from\n"
                + "Microtan java (an obscure british Oric ancestor))\n"
                + "this program is under GPL licence\n"
                + "to load a K7 program:\n"
                + "File->Load a K7 : to select the file (uncompressed)\n"
                + "under Basic interpreter type LOAD then type RUN\n"
                + "or LOADM then EXEC\n"
                + "\n"
                + "Contact gilles.fetis@wanadoo.fr";

        TextArea ta = new TextArea(aboutText, 15, 40, TextArea.SCROLLBARS_NONE);
        ta.setEditable(false);
        guiDialog.removeAll();
        guiDialog.setTitle("About Marcel O Cinq");
        guiDialog.setLayout(new FlowLayout());
        guiDialog.add(ta);
        guiDialog.setSize(375, 280);

        guiDialog.show();
    }

    private void debug() {



        TextArea t1 = new TextArea(this.machine.dumpRegisters(), 2, 40, TextArea.SCROLLBARS_NONE);
        t1.setEditable(false);

        TextArea t2 = new TextArea(this.machine.unassembleFromPC(10), 10, 40, TextArea.SCROLLBARS_NONE);
        t2.setEditable(false);

        debugDialog.removeAll();
        debugDialog.add(t1);
        debugDialog.add(t2);
        debugDialog.setLayout(new FlowLayout());
        debugDialog.setSize(320, 400);

        debugDialog.show();
    }
} // of class

