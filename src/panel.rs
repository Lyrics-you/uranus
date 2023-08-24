use crate::assemble::Assemble;
use crate::toast::{self, Status, Toast};

use iced::font::{self, Font};
use iced::theme::Palette;
use iced::widget::{
    self, button, checkbox, column, container, image, row, text, tooltip, Image, Text, Tooltip,
};
use iced::{executor, keyboard, subscription, window, Color, Event, Subscription};
use iced::{Application, Command, Element, Length, Settings, Theme};

use rfd::FileDialog;

// font for `icon` and `font`
pub const ICON_FONT: Font = Font::with_name("icons");
pub const YY_FONT: Font = Font {
    family: iced::font::Family::Name("YouYuan"),
    weight: font::Weight::Normal,
    stretch: font::Stretch::Normal,
    monospaced: false,
};
// text font size
const FONT_SIZE: f32 = 24.0;
// tip size and position
const TIP_SIZE: f32 = 16.0;
const TIP_POSITION: iced::widget::tooltip::Position = tooltip::Position::FollowCursor;
// custom color
const RED_COLOR: iced::Color =
    Color::from_rgb(120 as f32 / 255.0, 5 as f32 / 255.0, 0 as f32 / 255.0);
const ORANGE_COLOR: iced::Color =
    Color::from_rgb(255 as f32 / 255.0, 120 as f32 / 255.0, 5 as f32 / 255.0);
const GREEN_COLOR: iced::Color =
    Color::from_rgb(0 as f32 / 255.0, 180 as f32 / 255.0, 150 as f32 / 255.0);

//  run iced
pub fn pannel_main() -> iced::Result {
    let mut settings = Settings::default();

    let mut window_settings: window::Settings = window::Settings::default();
    window_settings.size = (600, 800);
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
    toasts: Vec<Toast>,
}

impl Pannel<'_> {
    /// According to the value to select the corresponding checkbox for change.
    pub fn checkbox_selector(&mut self, value: bool, message: Message) {
        // roolback checkbox should be false
        self.roolback_checkbox = false;
        // select corresponding checkbox and charism
        let (checkbox_ptr, charism) = match message {
            Message::AlwaysFishingPointChecked(_) => (
                &mut self.always_fishing_point_checkbox,
                self.assemble.always_fishing_point_charism.borrow(),
            ),
            Message::CatchBetterFishChecked(_) => (
                &mut self.catch_better_fish_checkbox,
                self.assemble.catch_better_fish_charism.borrow(),
            ),
            Message::EasierToPickUpCheced(_) => (
                &mut self.easier_to_pick_up_checkbox,
                self.assemble.easier_to_pick_up_charism.borrow(),
            ),
            Message::GifitTraitQuickUpgradeChecked(_) => (
                &mut self.gifit_trait_quick_upgrade_checkbox,
                self.assemble.gifit_trait_quick_upgrade_charism.borrow(),
            ),
            Message::FreeStoreExchangeChecked(_) => (
                &mut self.free_store_exchange_checkbox,
                self.assemble.free_store_exchange_charism.borrow(),
            ),
            Message::AlwaysHeroRaityTraitChecked(_) => (
                &mut self.always_hero_raity_trait_checkbox,
                self.assemble.always_hero_raity_trait_charism.borrow(),
            ),
            _ => {
                return;
            }
        };
        // change checkbox to value
        *checkbox_ptr = value;
        if value {
            // checked, apply
            match charism.apply() {
                Ok(_) => {
                    log::info!("Pannel: {} apply success.", charism.name);
                    self.toasts.push(Toast {
                        title: "Apply".into(),
                        body: "success".into(),
                        status: Status::Success,
                    });
                }
                Err(err) => {
                    log::error!("Pannel: {} apply failed!", err);
                    *checkbox_ptr = false;
                    self.toasts.push(Toast {
                        title: "Apply".into(),
                        body: "failed".into(),
                        status: Status::Danger,
                    });
                }
            }
        } else {
            // uncheck, rollback
            match charism.rollback(true) {
                Ok(_) => {
                    log::info!("Pannel: {} rollback success.", charism.name);
                    self.toasts.push(Toast {
                        title: "RollBack".into(),
                        body: "success".into(),
                        status: Status::Success,
                    });
                }
                Err(err) => {
                    log::error!("Pannel: {} rollback failed!", err);
                    self.toasts.push(Toast {
                        title: "RollBack".into(),
                        body: "failed".into(),
                        status: Status::Danger,
                    });
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
    ToastClose(usize),
    Event(Event),
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

    fn subscription(&self) -> Subscription<Self::Message> {
        subscription::events().map(Message::Event)
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
        // select message
        match message {
            Message::SourceLoaded(_) => {}
            Message::Event(Event::Keyboard(keyboard::Event::KeyPressed {
                key_code: keyboard::KeyCode::Tab,
                modifiers,
            })) if modifiers.shift() => {
                return widget::focus_previous();
            }
            Message::Event(Event::Keyboard(keyboard::Event::KeyPressed {
                key_code: keyboard::KeyCode::Tab,
                ..
            })) => {
                return widget::focus_next();
            }
            Message::Event(_) => {}
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
            Message::ToastClose(_index) => {
                // a little problem in remove(index)
                // self.toasts.remove(_index);
                self.toasts.clear();
            }
            Message::AlwaysFishingPointChecked(value)
            | Message::CatchBetterFishChecked(value)
            | Message::EasierToPickUpCheced(value)
            | Message::GifitTraitQuickUpgradeChecked(value)
            | Message::FreeStoreExchangeChecked(value)
            | Message::AlwaysHeroRaityTraitChecked(value) => {
                // check hades_path
                if self.hades_path == "" {
                    log::warn!("Pick Floder First!");
                    let toast = Toast {
                        title: "Tips".into(),
                        body: "Pick Floder First".into(),
                        status: Status::Primary,
                    };
                    self.toasts.push(toast);
                    return Command::none();
                }
                self.checkbox_selector(value, message);
            }
            Message::RollbackChecked(value) => {
                self.roolback_checkbox = value;
                // rollback the checkbox with a value of ture
                if value {
                    if self.always_fishing_point_checkbox {
                        self.checkbox_selector(false, Message::AlwaysFishingPointChecked(false))
                    }

                    if self.catch_better_fish_checkbox {
                        self.checkbox_selector(false, Message::CatchBetterFishChecked(false));
                    };

                    if self.easier_to_pick_up_checkbox {
                        self.checkbox_selector(false, Message::EasierToPickUpCheced(false));
                    };

                    if self.gifit_trait_quick_upgrade_checkbox {
                        self.checkbox_selector(
                            false,
                            Message::GifitTraitQuickUpgradeChecked(false),
                        );
                    };

                    if self.free_store_exchange_checkbox {
                        self.checkbox_selector(false, Message::FreeStoreExchangeChecked(false));
                    };

                    if self.always_hero_raity_trait_checkbox {
                        self.checkbox_selector(false, Message::AlwaysHeroRaityTraitChecked(false));
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

        // image for banner
        let image =
            Image::<image::Handle>::new("resources/zagreus-icon-2.jpg").width(Length::Fixed(500.0));

        // text "Path"
        let path_text = text("Path")
            .size(FONT_SIZE)
            .font(YY_FONT)
            .style(ORANGE_COLOR);

        // changes button context according to the value of hades_path
        let mut context = "Pick Floder";
        if self.hades_path != "" {
            context = &self.hades_path;
        }

        // pick floder button
        let floder_picker = button(Text::new(context).font(YY_FONT).size(TIP_SIZE))
            .on_press(Message::FloderPickPressed);

        // combine the path and floder_picker in a row
        let line = row![path_text, floder_picker]
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
                .size(FONT_SIZE)
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
                .size(FONT_SIZE)
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
                .size(FONT_SIZE)
                .size(FONT_SIZE)
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
            .size(FONT_SIZE)
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
                .size(FONT_SIZE)
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
                .size(FONT_SIZE)
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

        let rollback_checkbox = checkbox("", self.roolback_checkbox, Message::RollbackChecked)
            .icon(checkbox::Icon {
                font: ICON_FONT,
                code_point: '\u{e901}',
                size: None,
                line_height: text::LineHeight::Relative(1.0),
                shaping: text::Shaping::Basic,
            })
            .font(YY_FONT);
        let rollback_text = Text::new("RollBack")
            .font(YY_FONT)
            .size(FONT_SIZE)
            .style(GREEN_COLOR);
        let rollback = row![rollback_checkbox, rollback_text];

        // toast

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

        let container = container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            // .padding(24)
            ;
        // toast manager
        toast::Manager::new(container, &self.toasts, Message::ToastClose)
            .timeout(1)
            .into()
    }
}
