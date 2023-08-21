use crate::assemble::Assemble;
use crate::components::Charism;

use std::cell::RefCell;
use std::rc::Rc;

use iced::font::{self, Font};
use iced::theme::Palette;
use iced::widget::{
    button, checkbox, column, container, image, row, text, tooltip, Image, Text, Tooltip,
};
use iced::{executor, window, Color};
use iced::{Application, Command, Element, Length, Settings, Theme};

use rfd::FileDialog;

// CONST
const ICON_FONT: Font = Font::with_name("icons");
const YY_FONT: Font = Font {
    family: iced::font::Family::Name("YouYuan"),
    weight: font::Weight::Normal,
    stretch: font::Stretch::Normal,
    monospaced: false,
};

const TIP_SIZE: f32 = 16.0;
const TIP_POSITION: iced::widget::tooltip::Position = tooltip::Position::FollowCursor;

const RED_COLOR: iced::Color =
    Color::from_rgb(120 as f32 / 255.0, 5 as f32 / 255.0, 0 as f32 / 255.0);
const ORANGE_COLOR: iced::Color =
    Color::from_rgb(255 as f32 / 255.0, 120 as f32 / 255.0, 5 as f32 / 255.0);
const GREEN_COLOR: iced::Color =
    Color::from_rgb(0 as f32 / 255.0, 180 as f32 / 255.0, 150 as f32 / 255.0);

// main
pub fn pannel_main() -> iced::Result {
    let mut settings = Settings::default();

    let mut window_settings: window::Settings = window::Settings::default();
    window_settings.size = (400, 600);
    window_settings.resizable = false;
    settings.window = window_settings;

    // setting window icon
    match window::icon::from_file("resources/zagreus-icon-1.jpg") {
        Ok(icon) => settings.window.icon = Some(icon),
        Err(_) => settings.window.icon = None,
    }
    Pannel::run(settings)
}

#[derive(Default)]
struct Pannel<'a> {
    always_fishing_point_checkbox: bool,
    catch_better_fish_checkbox: bool,
    easier_to_pick_up_checkbox: bool,
    gifit_trait_quick_upgrade_checkbox: bool,
    free_store_exchange_checkbox: bool,
    always_hero_raity_trait_checkbox: bool,

    roolback_checkbox: bool,
    pub assemble: Assemble<'a>,
    hades_path: String,
}

impl Pannel<'_> {
    pub fn checkbox_selector(value: bool, charism: &Rc<RefCell<Charism>>) {
        // todo : Toast
        let charism = charism.borrow();
        if value {
            match charism.apply() {
                Ok(_) => {
                    log::info!("Pannel: {} apply success.", charism.name);
                }
                Err(err) => {
                    log::error!("Pannel: {} apply failed!", err)
                }
            }
        } else {
            match charism.rollback(true) {
                Ok(_) => {
                    log::info!("Pannel: {} rollback success.", charism.name);
                }
                Err(err) => {
                    log::error!("Pannel: {} rollback failed!", err)
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    FloderPickPressed,
    AlwaysFishingPointChecked(bool),
    CatchBetterFishChecked(bool),
    EasierToPickUpCheced(bool),
    GifitTraitQuickUpgradeChecked(bool),
    FreeStoreExchangeChecked(bool),
    AlwaysHeroRaityTraitChecked(bool),
    RollbackChecked(bool),
    SourceLoaded(Result<(), font::Error>),
}

impl Application for Pannel<'_> {
    type Message = Message;
    type Flags = ();
    type Executor = executor::Default;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self::default(),
            Command::batch(vec![
                font::load(include_bytes!("../fonts/icons.ttf").as_slice())
                    .map(Message::SourceLoaded),
                font::load(include_bytes!("../fonts/YouYuan.ttf").as_slice())
                    .map(Message::SourceLoaded),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("Uranus - Hades")
    }

    fn theme(&self) -> Self::Theme {
        let palette = Palette {
            background: Color::from_rgb(32 as f32 / 255.0, 34 as f32 / 255.0, 37 as f32 / 255.0),
            text: Color::from_rgb(0.90, 0.90, 0.90),
            primary: RED_COLOR,
            success: Color::from_rgb(18 as f32 / 255.0, 102 as f32 / 255.0, 79 as f32 / 255.0),
            danger: Color::from_rgb(195 as f32 / 255.0, 66 as f32 / 255.0, 63 as f32 / 255.0),
        };
        Theme::custom(palette)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        // check hades path
        match message {
            Message::FloderPickPressed | Message::SourceLoaded(_) => {}
            _ => {
                if self.hades_path == "" {
                    log::warn!("Pick Floder First!");
                    return Command::none();
                }
            }
        }
        // select message
        match message {
            Message::SourceLoaded(_) => {}
            Message::FloderPickPressed => {
                if let Some(floder_path) = FileDialog::new().pick_folder() {
                    let path = floder_path.display().to_string();
                    self.hades_path = path;

                    //  create a new assemble, otherwise the backup_files is the same as before
                    self.assemble = Assemble::new();
                    self.assemble
                        .set_hades_path(floder_path.display().to_string());
                    self.assemble.assemble_all();

                    log::info!("Change Path to {}", self.assemble.hades_path)
                }
            }
            Message::AlwaysFishingPointChecked(value) => {
                self.always_fishing_point_checkbox = value;
                self.roolback_checkbox = false;
                Self::checkbox_selector(value, &self.assemble.always_fishing_point_charism);
            }
            Message::CatchBetterFishChecked(value) => {
                self.catch_better_fish_checkbox = value;
                self.roolback_checkbox = false;
                Self::checkbox_selector(value, &self.assemble.always_fishing_point_charism);
            }

            Message::EasierToPickUpCheced(value) => {
                self.easier_to_pick_up_checkbox = value;
                self.roolback_checkbox = false;
                Self::checkbox_selector(value, &self.assemble.easier_to_pick_up_charism);
            }
            Message::GifitTraitQuickUpgradeChecked(value) => {
                self.gifit_trait_quick_upgrade_checkbox = value;
                self.roolback_checkbox = false;
                Self::checkbox_selector(value, &self.assemble.gifit_trait_quick_upgrade_charism);
            }
            Message::FreeStoreExchangeChecked(value) => {
                self.free_store_exchange_checkbox = value;
                self.roolback_checkbox = false;
                Self::checkbox_selector(value, &self.assemble.free_store_exchange_charism);
            }
            Message::AlwaysHeroRaityTraitChecked(value) => {
                self.always_hero_raity_trait_checkbox = value;
                self.roolback_checkbox = false;
                Self::checkbox_selector(value, &self.assemble.always_hero_raity_trait_charism);
            }
            Message::RollbackChecked(value) => {
                self.roolback_checkbox = value;
                if value {
                    if self.always_fishing_point_checkbox {
                        self.always_fishing_point_checkbox = false;
                        Self::checkbox_selector(false, &self.assemble.always_fishing_point_charism);
                    }

                    if self.catch_better_fish_checkbox {
                        self.catch_better_fish_checkbox = false;
                        Self::checkbox_selector(false, &self.assemble.catch_better_fish_charism);
                    };

                    if self.easier_to_pick_up_checkbox {
                        self.easier_to_pick_up_checkbox = false;
                        Self::checkbox_selector(false, &self.assemble.easier_to_pick_up_charism);
                    };

                    if self.gifit_trait_quick_upgrade_checkbox {
                        self.gifit_trait_quick_upgrade_checkbox = false;
                        Self::checkbox_selector(
                            false,
                            &self.assemble.gifit_trait_quick_upgrade_charism,
                        );
                    };

                    if self.free_store_exchange_checkbox {
                        self.free_store_exchange_checkbox = false;
                        Self::checkbox_selector(false, &self.assemble.free_store_exchange_charism);
                    };

                    if self.always_hero_raity_trait_checkbox {
                        self.always_hero_raity_trait_checkbox = false;
                        Self::checkbox_selector(
                            false,
                            &self.assemble.always_hero_raity_trait_charism,
                        );
                    };
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        // let handle = image::Handle::from_path(format!(
        //     "{}/resources/hades-icon.jpg",
        //     env!("CARGO_MANIFEST_DIR")
        // ));
        let image =
            Image::<image::Handle>::new("resources/zagreus-icon-2.jpg").width(Length::Fixed(300.0));

        let text = text("Path").size(16).font(YY_FONT).style(ORANGE_COLOR);

        let mut context = "Pick Floder";
        if self.hades_path != "" {
            context = &self.hades_path;
        }

        let floder_picker =
            button(Text::new(context).font(YY_FONT)).on_press(Message::FloderPickPressed);

        let line = row![text, floder_picker]
            .spacing(24)
            .align_items(iced::Alignment::Center);

        // "Always Fishing Point",
        let always_fishing_point_checkbox = checkbox(
            "",
            self.always_fishing_point_checkbox,
            Message::AlwaysFishingPointChecked,
        )
        .font(YY_FONT);

        let always_fishing_point_tip = Tooltip::new(
            Text::new(self.assemble.always_fishing_point_charism.borrow().name)
                .font(YY_FONT)
                .style(ORANGE_COLOR),
            self.assemble
                .always_fishing_point_charism
                .borrow()
                .description,
            TIP_POSITION,
        )
        // .gap(10)
        .font(YY_FONT)
        // .padding(10)
        .size(TIP_SIZE);
        let always_fishing_point = row![always_fishing_point_checkbox, always_fishing_point_tip];

        // "Catch Better Fish",
        let catch_better_fish_checkbox = checkbox(
            "",
            self.catch_better_fish_checkbox,
            Message::CatchBetterFishChecked,
        )
        .font(YY_FONT);
        let catch_better_fish_tip = Tooltip::new(
            Text::new(self.assemble.catch_better_fish_charism.borrow().name)
                .font(YY_FONT)
                .style(ORANGE_COLOR),
            self.assemble.catch_better_fish_charism.borrow().description,
            TIP_POSITION,
        )
        // .gap(10)
        .font(YY_FONT)
        // .padding(10)
        .size(TIP_SIZE);
        let catch_better_fish = row![catch_better_fish_checkbox, catch_better_fish_tip];

        // "Easier To Pickup"
        let easier_to_pick_up_checkbox = checkbox(
            "",
            self.easier_to_pick_up_checkbox,
            Message::EasierToPickUpCheced,
        )
        .font(YY_FONT);
        let easier_to_pick_up_tip = Tooltip::new(
            Text::new(self.assemble.easier_to_pick_up_charism.borrow().name)
                .font(YY_FONT)
                .style(ORANGE_COLOR),
            self.assemble.easier_to_pick_up_charism.borrow().description,
            TIP_POSITION,
        )
        // .gap(10)
        .font(YY_FONT)
        // .padding(10)
        .size(TIP_SIZE);
        let easier_to_pick_up = row![easier_to_pick_up_checkbox, easier_to_pick_up_tip];

        // "GifitTrait Quick Upgrade"
        let gifit_trait_quick_upgrade_checkbox = checkbox(
            "",
            self.gifit_trait_quick_upgrade_checkbox,
            Message::GifitTraitQuickUpgradeChecked,
        )
        .font(YY_FONT);
        let gifit_trait_quick_upgrade_tip = Tooltip::new(
            Text::new(
                self.assemble
                    .gifit_trait_quick_upgrade_charism
                    .borrow()
                    .name,
            )
            .font(YY_FONT)
            .style(ORANGE_COLOR),
            self.assemble
                .gifit_trait_quick_upgrade_charism
                .borrow()
                .description,
            TIP_POSITION,
        )
        // .gap(10)
        .font(YY_FONT)
        // .padding(10)
        .size(TIP_SIZE);
        let gifit_trait_quick_upgrade = row![
            gifit_trait_quick_upgrade_checkbox,
            gifit_trait_quick_upgrade_tip
        ];

        // "Free Store Exchange"
        let free_store_exchange_checkbox = checkbox(
            "",
            self.free_store_exchange_checkbox,
            Message::FreeStoreExchangeChecked,
        )
        .font(YY_FONT);
        let free_store_exchange_tip = Tooltip::new(
            Text::new(self.assemble.free_store_exchange_charism.borrow().name)
                .font(YY_FONT)
                .style(ORANGE_COLOR),
            self.assemble
                .free_store_exchange_charism
                .borrow()
                .description,
            TIP_POSITION,
        )
        // .gap(10)
        .font(YY_FONT)
        // .padding(10)
        .size(TIP_SIZE);
        let free_store_exchange = row![free_store_exchange_checkbox, free_store_exchange_tip];

        // "Always Hero RaityTrait"
        let always_hero_raity_trait_checkbox = checkbox(
            "",
            self.always_hero_raity_trait_checkbox,
            Message::AlwaysHeroRaityTraitChecked,
        )
        .font(YY_FONT);
        let always_hero_raity_trait_tip = Tooltip::new(
            Text::new(self.assemble.always_hero_raity_trait_charism.borrow().name)
                .font(YY_FONT)
                .style(ORANGE_COLOR),
            self.assemble
                .always_hero_raity_trait_charism
                .borrow()
                .description,
            TIP_POSITION,
        )
        // .gap(10)
        .font(YY_FONT)
        // .padding(10)
        .size(TIP_SIZE);
        let always_hero_raity_trait = row![
            always_hero_raity_trait_checkbox,
            always_hero_raity_trait_tip
        ];

        let rollback_checkbox =
            checkbox("", self.roolback_checkbox, Message::RollbackChecked)
                .icon(checkbox::Icon {
                    font: ICON_FONT,
                    code_point: '\u{e901}',
                    size: None,
                    line_height: text::LineHeight::Relative(1.0),
                    shaping: text::Shaping::Basic,
                })
                .font(YY_FONT);
        let rollback_text = Text::new("RollBack").font(YY_FONT).style(GREEN_COLOR);
        let rollback = row![rollback_checkbox, rollback_text];

        let content = column![
            image,
            line,
            always_fishing_point,
            catch_better_fish,
            easier_to_pick_up,
            gifit_trait_quick_upgrade,
            free_store_exchange,
            always_hero_raity_trait,
            rollback,
        ]
        .spacing(24);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            // .padding(24)
            .into()
    }
}
