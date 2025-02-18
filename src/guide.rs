use egui::text::LayoutJob;
use egui::{Color32, ScrollArea, TextFormat};
use egui_extras::Size;

pub const GUIDE_VERSION: &str = "0.2";

const SPACE_IN_MENU: f32 = 14.0;
const SPACE_BEFORE_PARAGRAPH: f32 = 10.0;

#[derive(Default)]
pub struct GuideReader {
    contents: Contents,
}

impl eframe::App for GuideReader {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.contents.ui(ctx);
    }
}

#[derive(Default)]
pub struct Contents {
    article: Article,
    about: AboutWorkbook,
    intro: Introduction,
    structure: Structure,
    editing: EditingProject,
    saving: SavingProject,
    help: Help,
}

#[derive(Default)]
enum Article {
    #[default]
    AboutWorkbook,
    Introduction,
    Structure,
    EditingProject,
    SavingProject,
    Help,
}

// Table of contents entries in menu
pub trait Menu {
    // `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;
}

pub trait Reader {
    fn read(&self, ui: &mut egui::Ui, text_format: TextFormat);
}

impl Contents {
    // Show the app ui (table of contents panel and central panel).
    pub fn ui(&mut self, ctx: &egui::Context) {
        self.table_of_contents(ctx);
        self.reader_panel(ctx);
    }
}

impl Contents {
    fn table_of_contents(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("guide_contents_panel").show(ctx, |ui| {
            ScrollArea::vertical()
                .id_source("guide_outline")
                .show(ui, |ui| {
                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                        ui.add_space(SPACE_IN_MENU);
                        let button_rounding = egui::Rounding::ZERO;
                        let button_color = Color32::WHITE.gamma_multiply(0.01);

                        let about_button = egui::Button::new(self.about.name())
                            .fill(button_color)
                            .rounding(button_rounding);
                        let intro_button = egui::Button::new(self.intro.name())
                            .fill(button_color)
                            .rounding(button_rounding);
                        let editing_button = egui::Button::new(self.editing.name())
                            .fill(button_color)
                            .rounding(button_rounding);
                        let structure_button = egui::Button::new(self.structure.name())
                            .fill(button_color)
                            .rounding(button_rounding);
                        let saving_button = egui::Button::new(self.saving.name())
                            .fill(button_color)
                            .rounding(button_rounding);
                        let help_button = egui::Button::new(self.help.name())
                            .fill(button_color)
                            .rounding(button_rounding);

                        let mut about_response = ui.add(about_button);
                        let mut intro_response = ui.add(intro_button);
                        let mut structure_response = ui.add(structure_button);
                        let mut editing_response = ui.add(editing_button);
                        let mut saving_response = ui.add(saving_button);
                        let mut help_response = ui.add(help_button);

                        match self.article {
                            Article::AboutWorkbook => {
                                about_response = about_response.highlight();
                            }
                            Article::Introduction => {
                                intro_response = intro_response.highlight();
                            }
                            Article::Structure => {
                                structure_response = structure_response.highlight();
                            }
                            Article::EditingProject => {
                                editing_response = editing_response.highlight();
                            }
                            Article::SavingProject => {
                                saving_response = saving_response.highlight();
                            }
                            Article::Help => {
                                help_response = help_response.highlight();
                            }
                        }

                        if about_response.clicked() {
                            self.article = Article::AboutWorkbook
                        }
                        if intro_response.clicked() {
                            self.article = Article::Introduction
                        }
                        if structure_response.clicked() {
                            self.article = Article::Structure
                        }
                        if editing_response.clicked() {
                            self.article = Article::EditingProject
                        }
                        if saving_response.clicked() {
                            self.article = Article::SavingProject
                        }
                        if help_response.clicked() {
                            self.article = Article::Help
                        }
                    });
                });
        });
    }

    fn reader_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let body_text_size = egui::TextStyle::Body.resolve(ui.style()).size;
            egui_extras::StripBuilder::new(ui)
                .size(Size::exact(3. * body_text_size))
                .size(Size::remainder())
                .size(Size::exact(body_text_size))
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("Workbook Guide");
                        });
                    });
                    strip.strip(|builder| {
                        builder
                            .size(Size::remainder())
                            .size(Size::relative(0.68).at_least(200.0).at_most(750.0))
                            .size(Size::remainder())
                            .horizontal(|mut strip| {
                                strip.empty();
                                strip.cell(|ui| {
                                    ScrollArea::vertical().id_source("guide_reader").show(
                                        ui,
                                        |ui| {
                                            ui.with_layout(
                                                egui::Layout::top_down_justified(egui::Align::LEFT),
                                                |ui| {
                                                    let text_format: TextFormat =
                                                        if ui.visuals().dark_mode {
                                                            TextFormat {
                                                                line_height: Some(
                                                                    1.68 * body_text_size,
                                                                ),
                                                                ..Default::default()
                                                            }
                                                        } else {
                                                            TextFormat {
                                                                line_height: Some(
                                                                    1.68 * body_text_size,
                                                                ),
                                                                color: Color32::from_gray(85), // -> Self([l, l, l, 255]) with l=0..255
                                                                ..Default::default()
                                                            }
                                                        };

                                                    match self.article {
                                                        Article::AboutWorkbook => {
                                                            self.about.read(ui, text_format);
                                                        }
                                                        Article::Introduction => {
                                                            self.intro.read(ui, text_format);
                                                        }
                                                        Article::Structure => {
                                                            self.structure.read(ui, text_format);
                                                        }
                                                        Article::EditingProject => {
                                                            self.editing.read(ui, text_format);
                                                        }
                                                        Article::SavingProject => {
                                                            self.saving.read(ui, text_format);
                                                        }
                                                        Article::Help => {
                                                            self.help.read(ui, text_format);
                                                        } // _ => {},
                                                    }
                                                },
                                            );
                                        },
                                    );
                                });
                                strip.empty();
                            });
                    });
                    strip.cell(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.hyperlink_to("OpenCall", "https://opencallworks.com")
                                .on_hover_text("https://opencallworks.com");
                        });
                    });
                });
        });
    }
}

// ----------------------------------------------------------------------------

#[derive(Default)]
pub struct AboutWorkbook {}

impl Menu for AboutWorkbook {
    fn name(&self) -> &'static str {
        "About Workbook"
    }
}

impl Reader for AboutWorkbook {
    fn read(&self, ui: &mut egui::Ui, layout: TextFormat) {
        let title = self.name();
        let paragraph_1 = "\
        Preparing a research project often involves a lot of groundwork—\
        browsing relevant publications, gathering extra information, \
        refining ideas, selecting the right program call, finding strong \
        partners, and collaborating on a range of details. \
        This Workbook is designed to support you through these early stages, \
        acting as an organizer with helpful hints and guides along the way.\
        "
        .to_string();
        let paragraph_2 = "\
        When you've completed this Workbook, filling out any official proposal \
        forms should be much easier—just copy and paste the information you’ve \
        already put together here.\
        "
        .to_string();
        let job_1 = LayoutJob::single_section(paragraph_1, layout.clone());
        let job_2 = LayoutJob::single_section(paragraph_2, layout.clone());

        ui.heading(title);
        ui.add_space(SPACE_BEFORE_PARAGRAPH);
        ui.label(job_1);
        ui.add_space(SPACE_BEFORE_PARAGRAPH);
        ui.label(job_2);
    }
}

// ----------------------------------------------------------------------------

#[derive(Default)]
pub struct Introduction {}

impl Menu for Introduction {
    fn name(&self) -> &'static str {
        "Introduction"
    }
}

impl Reader for Introduction {
    fn read(&self, ui: &mut egui::Ui, layout: TextFormat) {
        let title = self.name();
        let paragraph_1 = "\
        To succeed in a funding competition, having a groundbreaking idea is \
        essential—but building the right team comes first.\
        "
        .to_string();
        let paragraph_2 = "\
        Creating an effective, well-balanced team means finding the right words \
        to share the program call, your idea, and the specific contributions \
        you expect from each partner. It’s a process of listening to their \
        feedback, negotiating differences, refining your ideas, and weaving it \
        all together into a compelling, shared vision.\
        "
        .to_string();
        let paragraph_3 = "\
        This Workbook is designed to make that process smoother by helping you:\
        "
        .to_string();
        let bullet = "•".to_string();
        let par_3_bul_1 = "\
        outline the funding program,\
        "
        .to_string();
        let par_3_bul_2 = "\
        refine and clarify your project idea,\
        "
        .to_string();
        let par_3_bul_3 = "\
        find the best way to inspire each partner toward their unique \
        contribution, and\
        "
        .to_string();
        let par_3_bul_4 = "\
        prepare key sections of the project for final application forms.\
        "
        .to_string();
        let paragraph_4 = "\
        With these tools, bringing your vision to life becomes much more \
        manageable.\
        "
        .to_string();

        let job_1 = LayoutJob::single_section(paragraph_1, layout.clone());
        let job_2 = LayoutJob::single_section(paragraph_2, layout.clone());
        let job_3 = LayoutJob::single_section(paragraph_3, layout.clone());
        let job_bul = LayoutJob::single_section(bullet, layout.clone());
        let job_3_1 = LayoutJob::single_section(par_3_bul_1, layout.clone());
        let job_3_2 = LayoutJob::single_section(par_3_bul_2, layout.clone());
        let job_3_3 = LayoutJob::single_section(par_3_bul_3, layout.clone());
        let job_3_4 = LayoutJob::single_section(par_3_bul_4, layout.clone());
        let job_4 = LayoutJob::single_section(paragraph_4, layout.clone());

        ui.heading(title);
        ui.add_space(SPACE_BEFORE_PARAGRAPH);
        ui.label(job_1);
        ui.add_space(SPACE_BEFORE_PARAGRAPH);
        ui.label(job_2);
        ui.add_space(SPACE_BEFORE_PARAGRAPH);
        ui.label(job_3);
        ui.add_space(SPACE_BEFORE_PARAGRAPH);
        ui.horizontal(|ui| {
            ui.label(job_bul.clone());
            ui.label(job_3_1);
        });
        ui.horizontal(|ui| {
            ui.label(job_bul.clone());
            ui.label(job_3_2);
        });
        ui.horizontal(|ui| {
            ui.label(job_bul.clone());
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                ui.label(job_3_3);
            });
        });
        ui.horizontal(|ui| {
            ui.label(job_bul.clone());
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                ui.label(job_3_4);
            });
        });
        ui.add_space(SPACE_BEFORE_PARAGRAPH);
        ui.label(job_4);
    }
}

// ----------------------------------------------------------------------------

#[derive(Default)]
pub struct Structure {}

impl Menu for Structure {
    fn name(&self) -> &'static str {
        "Structure"
    }
}

impl Reader for Structure {
    fn read(&self, ui: &mut egui::Ui, layout: TextFormat) {
        let title = self.name();
        let paragraph_1 = "\
        While project writers may be familiar with starting from a blank sheet, \
        this Workbook offers a structured framework designed to support \
        collaboration and clarity. By providing a clear structure from the outset, \
        this method helps the project take shape as understanding and alignment \
        grow among team members. \
        We believe this approach not only speeds up the early stages of work but \
        also reduces stress by allowing information to flow in a natural order \
        as it’s gathered, refined, and organized.\
        "
        .to_string();
        let paragraph_2 = "\
        We suggest starting with the Workbook’s simplest view, which contains \
        just four sections: Project Title, Idea, Funding, and Timeline. These sections provide \
        a foundation for organizing your project narrative. As your ideas \
        evolve and the structure of your narrative becomes clearer, you can \
        customize the level of detail by exposing more sections. \
        Use the 'Details' control in the top right to adjust the range \
        of sections in sight. \
        You can also always reveal and conceal whole sections (or certain \
        pieces of your text) by checking and unchecking boxes next to \
        the titles of sections you want to show or hide in the 'Preview' \
        panel.\
        "
        .to_string();
        let paragraph_3 = "\
        This gradual approach helps streamline the early stages, enabling you \
        to organize and expand your narrative as it develops.\
        "
        .to_string();
        let job_1 = LayoutJob::single_section(paragraph_1, layout.clone());
        let job_2 = LayoutJob::single_section(paragraph_2, layout.clone());
        let job_3 = LayoutJob::single_section(paragraph_3, layout.clone());

        ui.heading(title);
        ui.add_space(SPACE_BEFORE_PARAGRAPH);
        ui.label(job_1);
        ui.add_space(SPACE_BEFORE_PARAGRAPH);
        ui.label(job_2);
        ui.add_space(SPACE_BEFORE_PARAGRAPH);
        ui.label(job_3);
    }
}

// ----------------------------------------------------------------------------

#[derive(Default)]
pub struct EditingProject {}

impl Menu for EditingProject {
    fn name(&self) -> &'static str {
        "How to Edit"
    }
}

impl Reader for EditingProject {
    fn read(&self, ui: &mut egui::Ui, layout: TextFormat) {
        let title = self.name();
        let paragraph_1 = "\
        On the editing panel, you can add details to your project freely.\
        "
        .to_string();
        let paragraph_2 = "\
        Don’t worry if your first notes are just drafts—they can be duplicated, \
        hidden, revealed, or deleted as needed. \
        When you add a note, it appears in view once you tick the checkbox next \
        to it. You can hide a note by unchecking this \
        box, and entire sections can be concealed or exposed in the same way.\
        "
        .to_string();
        let paragraph_3 = "\
        When starting a new project, your preview panel will likely be empty. \
        To view specific sections, just tick the box next to each section title \
        or click directly on the title itself.\
        "
        .to_string();
        let paragraph_4 = "\
        As you develop your project, you’ll want to experiment with copy-pasting \
        and rearranging notes to create different versions of your narrative. \
        Any versions can be temporarily recorded as project 'snapshots'. You can \
        use snapshots to capture and name certain stages of your work to revisit \
        and save them later. Refer to 'How to Save Your Project' for more details \
        on this feature.\
        "
        .to_string();
        let paragraph_6 = "\
        This structured approach gives you flexibility as you build and share \
        your project in the way that best suits your needs.\
        "
        .to_string();
        let job_1 = LayoutJob::single_section(paragraph_1, layout.clone());
        let job_2 = LayoutJob::single_section(paragraph_2, layout.clone());
        let job_3 = LayoutJob::single_section(paragraph_3, layout.clone());
        let job_4 = LayoutJob::single_section(paragraph_4, layout.clone());
        let job_6 = LayoutJob::single_section(paragraph_6, layout.clone());

        ui.heading(title);
        ui.add_space(SPACE_BEFORE_PARAGRAPH);
        ui.label(job_1);
        ui.add_space(SPACE_BEFORE_PARAGRAPH);
        ui.label(job_2);
        ui.add_space(SPACE_BEFORE_PARAGRAPH);
        ui.label(job_3);
        ui.add_space(SPACE_BEFORE_PARAGRAPH);
        ui.label(job_4);
        ui.add_space(SPACE_BEFORE_PARAGRAPH);
        ui.label(job_6);
    }
}

// ----------------------------------------------------------------------------

#[derive(Default)]
pub struct SavingProject {}

impl Menu for SavingProject {
    fn name(&self) -> &'static str {
        "How to Save"
    }
}

impl Reader for SavingProject {
    fn read(&self, ui: &mut egui::Ui, layout: TextFormat) {
        let title = self.name();

        let paragraph_1 = "\
        To keep a quick backup of your work as you go, you can use snapshots. \
        A snapshot captures the current state of your project, including any \
        checkmarks and edits. Snapshots stay available until you close the \
        application.\
        "
        .to_string();
        let paragraph_2 = "\
        To save your work, the Workbook provides two options:\
        "
        .to_string();
        let bullet = "•".to_string();
        let par_2_bul_1 = "\
        'Menu: File – Make a snapshot': This creates a temporary version of \
        your project, saved within the session. Note that snapshots won’t be \
        saved permanently on disk.\
        "
        .to_string();
        let par_2_bul_2 = "\
        'Menu: File – Save': This option saves your project on disk. Your \
        project will be saved as a file with the '.w' extension, and you can \
        store it in any folder or create a new one just for your projects.\
        "
        .to_string();
        let paragraph_3 = "\
        By saving your work regularly, you can keep track of your progress and \
        have peace of mind as your project evolves.\
        "
        .to_string();

        let job_1 = LayoutJob::single_section(paragraph_1, layout.clone());
        let job_2 = LayoutJob::single_section(paragraph_2, layout.clone());
        let job_bul = LayoutJob::single_section(bullet, layout.clone());
        let job_2_1 = LayoutJob::single_section(par_2_bul_1, layout.clone());
        let job_2_2 = LayoutJob::single_section(par_2_bul_2, layout.clone());
        let job_3 = LayoutJob::single_section(paragraph_3, layout.clone());

        ui.heading(title);
        ui.add_space(SPACE_BEFORE_PARAGRAPH);
        ui.label(job_1);
        ui.add_space(SPACE_BEFORE_PARAGRAPH);
        ui.label(job_2);
        ui.add_space(SPACE_BEFORE_PARAGRAPH);
        ui.horizontal(|ui| {
            ui.label(job_bul.clone());
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                ui.label(job_2_1);
            });
        });
        ui.horizontal(|ui| {
            ui.label(job_bul.clone());
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                ui.label(job_2_2);
            });
        });
        ui.add_space(SPACE_BEFORE_PARAGRAPH);
        ui.label(job_3);
    }
}

#[derive(Default)]
pub struct Help {}

impl Menu for Help {
    fn name(&self) -> &'static str {
        "Help"
    }
}

impl Reader for Help {
    fn read(&self, ui: &mut egui::Ui, layout: TextFormat) {
        let title = self.name();
        let text = "\
        In addition to this guide, hints and tips are available through \
        the \u{2139} icons in the editor. These tips come in two forms: \
        brief tooltips for quick reference and detailed explanations for \
        deeper guidance. Hover over an icon to see a short tip, or click on it \
        to open a more comprehensive help message in a separate window. \
        This way, you can get just the level of help you need, whether it’s \
        a quick reminder or an in-depth explanation.\
        "
        .to_string();
        let job = LayoutJob::single_section(text, layout);

        ui.heading(title);
        ui.add_space(SPACE_BEFORE_PARAGRAPH);
        ui.label(job);
    }
}
