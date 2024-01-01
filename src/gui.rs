use std::fs;
use std::path::Path;
use rfd::FileDialog;
use speedy2d::dimen::Vec2;
use speedy2d::Graphics2D;
use speedy2d::image::ImageHandle;
use speedy2d::window::{KeyScancode, ModifiersState, VirtualKeyCode, WindowHandler, WindowHelper};
use crate::hardware::keyboard::vkey::map_virtual_key_code;

use crate::hardware::machine::Machine;
use crate::hardware::screen::DEFAULT_PIXEL_SIZE;
use crate::int;

#[derive(Debug)]
pub(crate) struct Gui {
    pub(crate) machine: Machine,
    hadfile: Option<String>,
    image: Option<ImageHandle>,
}

impl Default for Gui {
    fn default() -> Self {
        let mut machine = Machine::default();
        machine.screen.set_pixel_size(DEFAULT_PIXEL_SIZE, &mut machine.mem);
        Gui {
            machine,
            hadfile: None,
            image: None,
        }
    }
}

impl Gui {
    fn open_file(&mut self) {
        let files = FileDialog::new()
            .add_filter("k7", &["k7"])
            .set_directory("./")
            .pick_file();
        if let Some(filename) = files {
            let name = filename.as_path();
            self.set_k7(name);
            self.hadfile = Some(fs::canonicalize(name).unwrap().to_string_lossy().into_owned());
        }
    }
}

impl WindowHandler for Gui {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        self.machine.run();
        if self.machine.screen.must_redraw {
            self.machine.screen.must_redraw = false;
            match self.machine.screen.paint(graphics, &mut self.machine.mem) {
                Ok(image) => {self.image = Some(image);}
                Err(err) => {println!("Error: {}", err);}
            }
        }
        if self.image.is_some() {
            let image = self.image.as_ref().unwrap();
            graphics.draw_image(Vec2::ZERO, image);
        }
        helper.request_redraw();
    }

    fn on_key_down(&mut self, _: &mut WindowHelper<()>, virtual_key_code: Option<VirtualKeyCode>, scancode: KeyScancode) {
        match virtual_key_code {
            Some(VirtualKeyCode::F2) => { self.open_file(); }
            Some(VirtualKeyCode::F7) => { self.machine.reset_soft(); }
            Some(VirtualKeyCode::F8) => { self.machine.reset_hard(); }
            _ => {
                self.machine.keyboard.key_pressed(map_virtual_key_code(virtual_key_code, scancode), &mut self.machine.mem);
            }
        }
    }

    fn on_key_up(&mut self, _: &mut WindowHelper<()>, virtual_key_code: Option<VirtualKeyCode>, scancode: KeyScancode) {
        self.machine.keyboard.key_released(map_virtual_key_code(virtual_key_code, scancode), &mut self.machine.mem);
    }

    fn on_keyboard_modifiers_changed(&mut self, _: &mut WindowHelper<()>, state: ModifiersState) {
        self.machine.keyboard.modifiers = state;
    }
}

impl Gui {
    fn set_k7(&mut self, filename: &Path) {
        self.machine.set_k7_file(filename);
    }

    // fn setK7(&mut self, mem: &Memory) {
    //     var filedia = new FileDialog(guiFrame, "Load file", FileDialog.LOAD);
    //     filedia.setVisible(true);
    //     String filename = filedia.getFile(&mut self, mem: &Memory);
    //     if (filename != null) {
    //         filename = filedia.getDirectory(&mut self, mem: &Memory) + filedia.getFile(&mut self, mem: &Memory);
    //         machine.set_k7file(filename);
    //         hadfile = filename;
    //     }
    // }

    fn zoom(&self, ps: int) {
        // self.screen.setPixelSize(ps);
        // var insets = guiFrame.getInsets(&mut self, mem: &Memory);
        // guiFrame.setSize((int) (320 * ps + (insets.left + insets.right)), (int) (200 * ps + (insets.top + insets.bottom)));
        // screen.repaint(&mut self, mem: &Memory);
    }


    // @Override
    // void windowClosing(WindowEvent e) {
    //     if (guiFrame.equals(e.getSource(&mut self, mem: &Memory))) {
    //         System.exit(0);
    //     }
    // }
    //
    // @Override
    // void windowActivated(WindowEvent e) {
    //     try {
    //         if (guiFrame.equals(e.getSource(&mut self, mem: &Memory))) {
    //             guiFrame.toFront(&mut self, mem: &Memory);
    //         }
    //     } catch (Exception re) {
    //     }
    // }
    //
    // @Override
    // void windowDeiconified(WindowEvent e) {
    //     if (guiFrame.equals(e.getSource(&mut self, mem: &Memory))) {
    //         guiFrame.toFront(&mut self, mem: &Memory);
    //     }
    // }
    //
    // @Override
    // void windowOpened(WindowEvent e) {
    //     if (e == null) {
    //         return;
    //     }
    //     if (guiFrame != null) {
    //         if (guiFrame.equals(e.getSource(&mut self, mem: &Memory))) {
    //             guiFrame.toFront(&mut self, mem: &Memory);
    //         }
    //     }
    // }

    fn init_gui(&mut self) {
        // guiFrame = new JFrame("Marcel O Cinq 3.1 (Java)");
        // guiFrame.setLayout(new BorderLayout(&mut self, mem: &Memory));
        //
        // var guiMenuBar = new JMenuBar(&mut self, mem: &Memory);
        //
        // var guiMenuFile = new JMenu("File");
        //
        // var guiMenuFileSelectK7 = new JMenuItem("Select K7");
        // guiMenuFileSelectK7.addActionListener(e -> setK7(&mut self, mem: &Memory));
        // guiMenuFile.add(guiMenuFileSelectK7);
        // var guiMenuRewindK7 = new JMenuItem("Rewind tape");
        // guiMenuRewindK7.addActionListener(e -> {
        //     if (hadfile != null) {
        //         if (hadfile.startsWith("http:")) {
        //             setK7FromUrl(hadfile);
        //         } else {
        //             setK7(hadfile);
        //         }
        //     } else {
        //         eprintln!("No file!");
        //     }
        // });
        // guiMenuFile.add(guiMenuRewindK7);
        // var guiMenuFileExit = new JMenuItem("Exit");
        // guiMenuFileExit.addActionListener(e -> System.exit(0));
        // guiMenuFile.add(guiMenuFileExit);
        //
        // guiMenuBar.add(guiMenuFile);
        //
        // var guiMenuRun = new JMenu("Run");
        // var guiMenuRunStop = new JMenuItem("Stop");
        // guiMenuRunStop.addActionListener(e ->  machine.stop(&mut self, mem: &Memory));
        // guiMenuRun.add(guiMenuRunStop);
        // var guiMenuRunGo = new JMenuItem("Go");
        // guiMenuRunGo.addActionListener(e -> machine.start(&mut self, mem: &Memory));
        // guiMenuRun.add(guiMenuRunGo);
        //
        // guiMenuBar.add(guiMenuRun);
        //
        // var guiMenuReset = new JMenu("Reset");
        // var guiMenuResetSoft = new JMenuItem("Soft Reset");
        // guiMenuResetSoft.addActionListener(e -> {
        //     machine.stop(&mut self, mem: &Memory);
        //     machine.resetSoft(&mut self, mem: &Memory);
        //     machine.start(&mut self, mem: &Memory);
        // });
        // guiMenuReset.add(guiMenuResetSoft);
        // var guiMenuResetHard = new JMenuItem("Hard Reset");
        // guiMenuResetHard.addActionListener(e -> {
        //     machine.stop(&mut self, mem: &Memory);
        //     machine.resetHard(&mut self, mem: &Memory);
        //     machine.start(&mut self, mem: &Memory);
        // });
        // guiMenuReset.add(guiMenuResetHard);
        //
        // guiMenuBar.add(guiMenuReset);
        //
        // var guiMenuZoom = new JMenu("Image");
        // var guiMenuZoomx1 = new JMenuItem("Zoom x 1");
        // guiMenuZoomx1.addActionListener(e -> zoom(1));
        // guiMenuZoom.add(guiMenuZoomx1);
        // var guiMenuZoomx2 = new JMenuItem("Zoom x 2");
        // guiMenuZoomx2.addActionListener(e -> zoom(2));
        // guiMenuZoom.add(guiMenuZoomx2);
        // var guiMenuZoomx3 = new JMenuItem("Zoom x 3");
        // guiMenuZoomx3.addActionListener(e -> zoom(4));
        // guiMenuZoom.add(guiMenuZoomx3);
        // var guiFilter = new JMenuItem("Filter");
        // guiFilter.addActionListener(e -> screen.setFilter(!screen.isFilter(&mut self, mem: &Memory)));
        // guiMenuZoom.add(guiFilter);
        //
        // guiMenuBar.add(guiMenuZoom);
        //
        // var guiMenuDebug = new JMenu("Debug");
        // var guiMenuDebugDebug = new JMenuItem("Debug");
        // guiMenuDebugDebug.addActionListener(e -> debug(&mut self, mem: &Memory));
        // guiMenuDebug.add(guiMenuDebugDebug);
        //
        // guiMenuBar.add(guiMenuDebug);
        //
        // var guiMenuHelp = new JMenu("Help");
        // var guiMenuHelpAbout = new JMenuItem("About");
        // guiMenuHelpAbout.addActionListener(e -> about(&mut self, mem: &Memory));
        // guiMenuHelp.add(guiMenuHelpAbout);
        //
        // guiMenuBar.add(guiMenuHelp);
        //
        // guiFrame.addWindowListener(this);
        // guiFrame.setJMenuBar(guiMenuBar);
        // guiFrame.add(screen);
        //
        //
        // guiFrame.pack(&mut self, mem: &Memory);
        //
        // screen.requestFocusInWindow(&mut self, mem: &Memory);
        //
        // machine.start(&mut self, mem: &Memory);
        //
        // screen.setPixelSize(1);
        // Insets i = guiFrame.getInsets(&mut self, mem: &Memory);
        // guiFrame.setSize((int) (320 * screen.getPixelSize(&mut self, mem: &Memory) + (i.left + i.right) + 10), (int) (200 * screen.getPixelSize(&mut self, mem: &Memory) + (i.top + i.bottom) + 15));
        // guiFrame.setVisible(true);
        // screen.repaint(&mut self, mem: &Memory);
        // screen.setPixelSize(2);
        // i = guiFrame.getInsets(&mut self, mem: &Memory);
        // guiFrame.setSize((int) (320 * screen.getPixelSize(&mut self, mem: &Memory) + (i.left + i.right)), (int) (200 * screen.getPixelSize(&mut self, mem: &Memory) + (i.top + i.bottom)));
        // screen.repaint(&mut self, mem: &Memory);
    }

    fn about(&mut self) {
        let about_text =
            "Marcel O Cinq 3.1 (java)\
\
            (C) G.Fetis 1997-1998-2006\
            (C) DevilMarkus http://cpc.devilmarkus.de 2006\
            (C) M.Le Goff 2014\
\
            Java conversion of my previously C/DOS\
            based Thomson MO5 emulator\
            (that was also ported to Unix and Macos)\
            The basic java design is taken from Pom1\
            (self.Apple1 java emulator (that derives from\
            Microtan java (an obscure british Oric ancestor))\
            this program is under GPL licence\
            to load a K7 program:\
            File->Load a K7 : to select the file (uncompressed)\
            under Basic interpreter type LOAD then type RUN\
            or LOADM then EXEC\
\
            Full keyboard emulation with all symbols\
            Sound emulation\
            Reset bug solved\
            Save K7 emulation\
            Lightpen emulation\
            AltGr+C = Ctrl+C = Break basic\
            F11 = BASIC     F12 = SHIFT\
\
            Contacts :\
            gilles.fetis@wanadoo.fr\
            marc.le.goff@gmail.fr\
            ";

        // var ta = new JTextArea(aboutText, 30, 40);
        // ta.setEditable(false);
        // ta.setBackground(Color.WHITE);
        // var guiDialog = new JDialog(guiFrame, true);
        // guiDialog.getContentPane(&mut self, mem: &Memory).add(new JScrollPane(ta));
        // guiDialog.setTitle("About Marcel O Cinq");
        // guiDialog.setSize(400, 500);
        // guiDialog.setVisible(true);
    }

    fn debug(&mut self) {
        // var t1 = new JTextArea(machine.dumpRegisters(&mut self, mem: &Memory), 2, 40);
        // t1.setEditable(false);
        // t1.setBackground(Color.WHITE);
        // var t2 = new JTextArea(machine.unassembleFromPC(10), 10, 40);
        // t2.setEditable(false);
        // t2.setBackground(Color.WHITE);
        // var debugDialog = new JDialog(guiFrame, true);
        // debugDialog.add(t1);
        // debugDialog.add(t2);
        // debugDialog.setLayout(new FlowLayout(&mut self, mem: &Memory));
        // debugDialog.setSize(400, 400);
        // debugDialog.setVisible(true);
    }
}