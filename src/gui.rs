use std::{fs, thread};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use eframe::{App, Frame};
use eframe::epaint::ColorImage;
use egui::{Context, ImageData, TextureHandle};
use log::{info, warn};
use rfd::FileDialog;

use crate::hardware::machine::Machine;
use crate::hardware::screen::{HEIGHT, WIDTH};
use crate::user_input::UserInput;

//use crate::hardware::keyboard::vkey::map_virtual_key_code;

pub(crate) struct Gui {
    hadfile: Option<String>,
    texture: Option<TextureHandle>,
    image_data_receiver: Receiver<ImageData>,
    user_input_sender: Sender<UserInput>,
    show_about: Arc<AtomicBool>,
}

impl Gui {
    pub(crate) fn new(image_data_receiver: Receiver<ImageData>, user_input_sender: Sender<UserInput>) -> Self {
        Gui {
            hadfile: None,
            texture: None,
            image_data_receiver,
            user_input_sender,
            show_about: Arc::new(AtomicBool::new(false)),
        }
    }

    fn select_k7(&mut self) {
        info!("select_k7");
        let sender = self.user_input_sender.clone();
        thread::spawn(move || {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("K7", &["k7"])
                .set_directory("./")
                .pick_file() {
                let path = path.as_path().to_owned();
                sender.send(UserInput::SetK7(path)).unwrap();
            }
        });
        /*
        let files = FileDialog::new()
            .add_filter("k7", &["k7"])
            .set_directory("./")
            .pick_file();
        if let Some(filename) = files {
            let name = filename.as_path();
            self.set_k7(name);
            self.hadfile = Some(fs::canonicalize(name).unwrap().to_string_lossy().into_owned());
        }*/
    }
}


impl App for Gui {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        if let Ok(image_data) = self.image_data_receiver.try_recv() {
            let texture_options = egui::TextureOptions::default();
            let texture = ctx.load_texture("Screen".to_string(),
                                           image_data,
                                           texture_options);
            self.texture = Some(texture);
        }
        self.build_menu_panel(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(texture) = self.texture.as_ref() {
                ui.image((texture.id(), ui.available_size()));
            } else {
                ui.spinner();
            }
        });
        if self.show_about.load(Ordering::Relaxed) {
            self.about(ctx);
        }
        ctx.request_repaint_after(Duration::from_millis(1000 / 60));
    }
}

impl Gui {
    fn build_menu_panel(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Select K7").clicked() {
                            self.select_k7();
                        }
                        if ui.button("Rewind tape").clicked() {
                            warn!("Not yet implemented");
                        }
                        if ui.button("Quit").clicked() {
                            info!("Quit");
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                        egui::widgets::global_dark_light_mode_buttons(ui);
                    });
                    ui.menu_button("Run", |ui| {
                        if ui.button("Stop").clicked() {
                            self.user_input_sender.send(UserInput::Stop).unwrap();
                        }
                        if ui.button("Go").clicked() {
                            self.user_input_sender.send(UserInput::Start).unwrap();
                        }
                    });
                    ui.menu_button("Reset", |ui| {
                        if ui.button("Soft Reset").clicked() {
                            self.user_input_sender.send(UserInput::SoftReset).unwrap();
                        }
                        if ui.button("Hard Reset").clicked() {
                            self.user_input_sender.send(UserInput::HardReset).unwrap();
                        }
                    });
                    ui.menu_button("Image", |ui| {
                        if ui.button("Zoom x 1").clicked() {
                            warn!("Not yet implemented");
                        }
                        if ui.button("Zoom x 2").clicked() {
                            warn!("Not yet implemented");
                        }
                        if ui.button("Zoom x 3").clicked() {
                            warn!("Not yet implemented");
                        }
                        if ui.button("Filter").clicked() {
                            warn!("Not yet implemented");
                        }
                    });
                    ui.menu_button("Debug", |ui| {
                        if ui.button("Debug").clicked() {
                            warn!("Not yet implemented");
                        }
                    });
                    ui.menu_button("Help", |ui| {
                        if ui.button("About").clicked() {
                            self.show_about.store(true, Ordering::Relaxed);
                        }
                    });
                }
            });
        });
    }

    fn about(&self, ctx: &Context) {
        info!("about");
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
        let show_deferred_viewport = self.show_about.clone();
        ctx.show_viewport_deferred(
            egui::ViewportId::from_hash_of("about_viewport"),
            egui::ViewportBuilder::default()
                .with_title("About Bernard")
                .with_inner_size([400.0, 500.0]),
            move |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Deferred,
                    "This egui backend doesn't support multiple viewports"
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label(about_text);
                });
                if ctx.input(|i| i.viewport().close_requested()) {
                    // Tell parent to close us.
                    show_deferred_viewport.store(false, Ordering::Relaxed);
                }
            },
        );
    }
}

/*
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
*/
impl Gui {
   /* fn set_k7(&mut self, filename: &Path) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            self.machine.set_k7_file(filename.display().to_string());
        }
    }*/

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

    // fn zoom(&self, ps: int) {
    // self.screen.setPixelSize(ps);
    // var insets = guiFrame.getInsets(&mut self, mem: &Memory);
    // guiFrame.setSize((int) (320 * ps + (insets.left + insets.right)), (int) (200 * ps + (insets.top + insets.bottom)));
    // screen.repaint(&mut self, mem: &Memory);
    // }


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

    // fn init_gui(&mut self) {
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
    // }

    // fn debug(&mut self) {
    //     // var t1 = new JTextArea(machine.dumpRegisters(&mut self, mem: &Memory), 2, 40);
    //     // t1.setEditable(false);
    //     // t1.setBackground(Color.WHITE);
    //     // var t2 = new JTextArea(machine.unassembleFromPC(10), 10, 40);
    //     // t2.setEditable(false);
    //     // t2.setBackground(Color.WHITE);
    //     // var debugDialog = new JDialog(guiFrame, true);
    //     // debugDialog.add(t1);
    //     // debugDialog.add(t2);
    //     // debugDialog.setLayout(new FlowLayout(&mut self, mem: &Memory));
    //     // debugDialog.setSize(400, 400);
    //     // debugDialog.setVisible(true);
    // }
}