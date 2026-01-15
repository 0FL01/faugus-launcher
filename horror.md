    Checking faugus-launcher-rs v0.0.1 (/home/stfu/ai/refactor-opensource/faugus-launcher)
error[E0433]: failed to resolve: could not find `Text` in `theme`
   --> src/gui/add_game_dialog.rs:658:41
    |
658 |                     .style(iced::theme::Text::Color(iced::Color::new(
    |                                         ^^^^ could not find `Text` in `theme`
    |
help: consider importing one of these items
    |
  4 + use crate::text::Text;
    |
  4 + use iced::advanced::Text;
    |
  4 + use iced::advanced::widget::Text;
    |
  4 + use iced::widget::Text;
    |
help: if you import `Text`, refer to it directly
    |
658 -                     .style(iced::theme::Text::Color(iced::Color::new(
658 +                     .style(Text::Color(iced::Color::new(
    |

error[E0433]: failed to resolve: could not find `Container` in `theme`
  --> src/gui/confirmation_dialog.rs:71:33
   |
71 |             .style(iced::theme::Container::Transparent);
   |                                 ^^^^^^^^^ could not find `Container` in `theme`
   |
help: consider importing one of these structs
   |
 4 + use crate::container::Container;
   |
 4 + use iced::widget::Container;
   |
help: if you import `Container`, refer to it directly
   |
71 -             .style(iced::theme::Container::Transparent);
71 +             .style(Container::Transparent);
   |

error[E0433]: failed to resolve: could not find `Container` in `theme`
   --> src/gui/confirmation_dialog.rs:106:29
    |
106 |         .style(iced::theme::Container::Box);
    |                             ^^^^^^^^^ could not find `Container` in `theme`
    |
help: consider importing one of these structs
    |
  4 + use crate::container::Container;
    |
  4 + use iced::widget::Container;
    |
help: if you import `Container`, refer to it directly
    |
106 -         .style(iced::theme::Container::Box);
106 +         .style(Container::Box);
    |

error[E0433]: failed to resolve: could not find `Container` in `theme`
   --> src/gui/main_window.rs:381:42
    |
381 | ...                   iced::theme::Container::Box
    |                                    ^^^^^^^^^ could not find `Container` in `theme`
    |
help: consider importing one of these structs
    |
  4 + use crate::container::Container;
    |
  4 + use iced::widget::Container;
    |
help: if you import `Container`, refer to it directly
    |
381 -                             iced::theme::Container::Box
381 +                             Container::Box
    |

error[E0433]: failed to resolve: could not find `Container` in `theme`
   --> src/gui/main_window.rs:383:42
    |
383 | ...                   iced::theme::Container::Transparent
    |                                    ^^^^^^^^^ could not find `Container` in `theme`
    |
help: consider importing one of these structs
    |
  4 + use crate::container::Container;
    |
  4 + use iced::widget::Container;
    |
help: if you import `Container`, refer to it directly
    |
383 -                             iced::theme::Container::Transparent
383 +                             Container::Transparent
    |

error[E0433]: failed to resolve: could not find `Text` in `theme`
   --> src/gui/main_window.rs:434:71
    |
434 |                         text(&game.title).size(14).style(iced::theme::Text::Color(
    |                                                                       ^^^^ could not find `Text` in `theme`
    |
help: consider importing one of these items
    |
  4 + use crate::text::Text;
    |
  4 + use iced::advanced::Text;
    |
  4 + use iced::advanced::widget::Text;
    |
  4 + use iced::widget::Text;
    |
help: if you import `Text`, refer to it directly
    |
434 -                         text(&game.title).size(14).style(iced::theme::Text::Color(
434 +                         text(&game.title).size(14).style(Text::Color(
    |

error[E0433]: failed to resolve: could not find `Container` in `theme`
   --> src/gui/main_window.rs:447:42
    |
447 | ...                   iced::theme::Container::Box
    |                                    ^^^^^^^^^ could not find `Container` in `theme`
    |
help: consider importing one of these structs
    |
  4 + use crate::container::Container;
    |
  4 + use iced::widget::Container;
    |
help: if you import `Container`, refer to it directly
    |
447 -                             iced::theme::Container::Box
447 +                             Container::Box
    |

error[E0433]: failed to resolve: could not find `Container` in `theme`
   --> src/gui/main_window.rs:449:42
    |
449 | ...                   iced::theme::Container::Transparent
    |                                    ^^^^^^^^^ could not find `Container` in `theme`
    |
help: consider importing one of these structs
    |
  4 + use crate::container::Container;
    |
  4 + use iced::widget::Container;
    |
help: if you import `Container`, refer to it directly
    |
449 -                             iced::theme::Container::Transparent
449 +                             Container::Transparent
    |

error[E0433]: failed to resolve: could not find `Container` in `theme`
   --> src/gui/main_window.rs:522:38
    |
522 |                         iced::theme::Container::Box
    |                                      ^^^^^^^^^ could not find `Container` in `theme`
    |
help: consider importing one of these structs
    |
  4 + use crate::container::Container;
    |
  4 + use iced::widget::Container;
    |
help: if you import `Container`, refer to it directly
    |
522 -                         iced::theme::Container::Box
522 +                         Container::Box
    |

error[E0433]: failed to resolve: could not find `Container` in `theme`
   --> src/gui/main_window.rs:524:38
    |
524 |                         iced::theme::Container::Transparent
    |                                      ^^^^^^^^^ could not find `Container` in `theme`
    |
help: consider importing one of these structs
    |
  4 + use crate::container::Container;
    |
  4 + use iced::widget::Container;
    |
help: if you import `Container`, refer to it directly
    |
524 -                         iced::theme::Container::Transparent
524 +                         Container::Transparent
    |

error[E0433]: failed to resolve: could not find `Text` in `theme`
   --> src/gui/proton_manager_dialog.rs:331:41
    |
331 |                     .style(iced::theme::Text::Color(iced::Color::new(
    |                                         ^^^^ could not find `Text` in `theme`
    |
help: consider importing one of these items
    |
  4 + use crate::text::Text;
    |
  4 + use iced::advanced::Text;
    |
  4 + use iced::advanced::widget::Text;
    |
  4 + use iced::widget::Text;
    |
help: if you import `Text`, refer to it directly
    |
331 -                     .style(iced::theme::Text::Color(iced::Color::new(
331 +                     .style(Text::Color(iced::Color::new(
    |

error[E0433]: failed to resolve: could not find `Text` in `theme`
   --> src/gui/settings_dialog.rs:345:41
    |
345 |                     .style(iced::theme::Text::Color(iced::Color::new(
    |                                         ^^^^ could not find `Text` in `theme`
    |
help: consider importing one of these items
    |
  4 + use crate::text::Text;
    |
  4 + use iced::advanced::Text;
    |
  4 + use iced::advanced::widget::Text;
    |
  4 + use iced::widget::Text;
    |
help: if you import `Text`, refer to it directly
    |
345 -                     .style(iced::theme::Text::Color(iced::Color::new(
345 +                     .style(Text::Color(iced::Color::new(
    |

error[E0433]: failed to resolve: use of undeclared type `Length`
   --> src/main.rs:536:28
    |
536 |                     .width(Length::Fill)
    |                            ^^^^^^ use of undeclared type `Length`
    |
help: consider importing this enum
    |
 15 + use iced::Length;
    |

error[E0433]: failed to resolve: use of undeclared type `Length`
   --> src/main.rs:537:29
    |
537 |                     .height(Length::Fill)
    |                             ^^^^^^ use of undeclared type `Length`
    |
help: consider importing this enum
    |
 15 + use iced::Length;
    |

error[E0433]: failed to resolve: could not find `Container` in `theme`
   --> src/main.rs:538:41
    |
538 |                     .style(iced::theme::Container::Transparent);
    |                                         ^^^^^^^^^ could not find `Container` in `theme`
    |
help: consider importing one of these structs
    |
 15 + use crate::container::Container;
    |
 15 + use iced::widget::Container;
    |
help: if you import `Container`, refer to it directly
    |
538 -                     .style(iced::theme::Container::Transparent);
538 +                     .style(Container::Transparent);
    |

error[E0433]: failed to resolve: use of undeclared type `Length`
   --> src/main.rs:544:32
    |
544 |                         .width(Length::Fixed(600.0))
    |                                ^^^^^^ use of undeclared type `Length`
    |
help: consider importing this enum
    |
 15 + use iced::Length;
    |

error[E0433]: failed to resolve: use of undeclared type `Length`
   --> src/main.rs:545:33
    |
545 |                         .height(Length::Fill)
    |                                 ^^^^^^ use of undeclared type `Length`
    |
help: consider importing this enum
    |
 15 + use iced::Length;
    |

error[E0433]: failed to resolve: use of undeclared type `Length`
   --> src/main.rs:548:24
    |
548 |                 .width(Length::Fill)
    |                        ^^^^^^ use of undeclared type `Length`
    |
help: consider importing this enum
    |
 15 + use iced::Length;
    |

error[E0433]: failed to resolve: use of undeclared type `Length`
   --> src/main.rs:549:25
    |
549 |                 .height(Length::Fill)
    |                         ^^^^^^ use of undeclared type `Length`
    |
help: consider importing this enum
    |
 15 + use iced::Length;
    |

error[E0433]: failed to resolve: use of undeclared type `Length`
   --> src/main.rs:563:28
    |
563 |                     .width(Length::Fill)
    |                            ^^^^^^ use of undeclared type `Length`
    |
help: consider importing this enum
    |
 15 + use iced::Length;
    |

error[E0433]: failed to resolve: use of undeclared type `Length`
   --> src/main.rs:564:29
    |
564 |                     .height(Length::Fill)
    |                             ^^^^^^ use of undeclared type `Length`
    |
help: consider importing this enum
    |
 15 + use iced::Length;
    |

error[E0433]: failed to resolve: could not find `Container` in `theme`
   --> src/main.rs:565:41
    |
565 |                     .style(iced::theme::Container::Transparent);
    |                                         ^^^^^^^^^ could not find `Container` in `theme`
    |
help: consider importing one of these structs
    |
 15 + use crate::container::Container;
    |
 15 + use iced::widget::Container;
    |
help: if you import `Container`, refer to it directly
    |
565 -                     .style(iced::theme::Container::Transparent);
565 +                     .style(Container::Transparent);
    |

error[E0433]: failed to resolve: use of undeclared type `Length`
   --> src/main.rs:571:32
    |
571 |                         .width(Length::Fixed(700.0))
    |                                ^^^^^^ use of undeclared type `Length`
    |
help: consider importing this enum
    |
 15 + use iced::Length;
    |

error[E0433]: failed to resolve: use of undeclared type `Length`
   --> src/main.rs:572:33
    |
572 |                         .height(Length::Fill)
    |                                 ^^^^^^ use of undeclared type `Length`
    |
help: consider importing this enum
    |
 15 + use iced::Length;
    |

error[E0433]: failed to resolve: use of undeclared type `Length`
   --> src/main.rs:575:24
    |
575 |                 .width(Length::Fill)
    |                        ^^^^^^ use of undeclared type `Length`
    |
help: consider importing this enum
    |
 15 + use iced::Length;
    |

error[E0433]: failed to resolve: use of undeclared type `Length`
   --> src/main.rs:576:25
    |
576 |                 .height(Length::Fill)
    |                         ^^^^^^ use of undeclared type `Length`
    |
help: consider importing this enum
    |
 15 + use iced::Length;
    |

error[E0433]: failed to resolve: use of undeclared type `Length`
   --> src/main.rs:590:28
    |
590 |                     .width(Length::Fill)
    |                            ^^^^^^ use of undeclared type `Length`
    |
help: consider importing this enum
    |
 15 + use iced::Length;
    |

error[E0433]: failed to resolve: use of undeclared type `Length`
   --> src/main.rs:591:29
    |
591 |                     .height(Length::Fill)
    |                             ^^^^^^ use of undeclared type `Length`
    |
help: consider importing this enum
    |
 15 + use iced::Length;
    |

error[E0433]: failed to resolve: could not find `Container` in `theme`
   --> src/main.rs:592:41
    |
592 |                     .style(iced::theme::Container::Transparent);
    |                                         ^^^^^^^^^ could not find `Container` in `theme`
    |
help: consider importing one of these structs
    |
 15 + use crate::container::Container;
    |
 15 + use iced::widget::Container;
    |
help: if you import `Container`, refer to it directly
    |
592 -                     .style(iced::theme::Container::Transparent);
592 +                     .style(Container::Transparent);
    |

error[E0433]: failed to resolve: use of undeclared type `Length`
   --> src/main.rs:598:32
    |
598 |                         .width(Length::Fixed(900.0))
    |                                ^^^^^^ use of undeclared type `Length`
    |
help: consider importing this enum
    |
 15 + use iced::Length;
    |

error[E0433]: failed to resolve: use of undeclared type `Length`
   --> src/main.rs:599:33
    |
599 |                         .height(Length::Fill)
    |                                 ^^^^^^ use of undeclared type `Length`
    |
help: consider importing this enum
    |
 15 + use iced::Length;
    |

error[E0433]: failed to resolve: use of undeclared type `Length`
   --> src/main.rs:602:24
    |
602 |                 .width(Length::Fill)
    |                        ^^^^^^ use of undeclared type `Length`
    |
help: consider importing this enum
    |
 15 + use iced::Length;
    |

error[E0433]: failed to resolve: use of undeclared type `Length`
   --> src/main.rs:603:25
    |
603 |                 .height(Length::Fill)
    |                         ^^^^^^ use of undeclared type `Length`
    |
help: consider importing this enum
    |
 15 + use iced::Length;
    |

error[E0603]: module `proton_manager` is private
  --> src/gui/proton_manager_dialog.rs:12:20
   |
12 | use crate::proton::proton_manager::{
   |                    ^^^^^^^^^^^^^^ private module
   |
note: the module `proton_manager` is defined here
  --> src/proton/mod.rs:4:1
   |
 4 | mod proton_manager;
   | ^^^^^^^^^^^^^^^^^^^

warning: unused import: `GameConfig`
 --> src/config/mod.rs:9:29
  |
9 | pub use game_config::{Game, GameConfig};
  |                             ^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `paths::Paths`
  --> src/config/mod.rs:10:9
   |
10 | pub use paths::Paths;
   |         ^^^^^^^^^^^^

warning: unused import: `Alignment`
 --> src/gui/add_game_dialog.rs:5:12
  |
5 | use iced::{Alignment, Element, Length, Task};
  |            ^^^^^^^^^

warning: unused imports: `Event` and `self`
 --> src/gui/main_window.rs:4:19
  |
4 | use iced::event::{self, Event};
  |                   ^^^^  ^^^^^

warning: unused import: `warn`
  --> src/gui/main_window.rs:12:28
   |
12 | use tracing::{error, info, warn};
   |                            ^^^^

warning: unused import: `text_input`
 --> src/gui/proton_manager_dialog.rs:5:72
  |
5 |     button, column, container, horizontal_rule, row, scrollable, text, text_input, Space,
  |                                                                        ^^^^^^^^^^

warning: unused import: `std::collections::HashMap`
 --> src/gui/proton_manager_dialog.rs:8:5
  |
8 | use std::collections::HashMap;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `container`
 --> src/gui/settings_dialog.rs:4:46
  |
4 | use iced::widget::{button, checkbox, column, container, row, scrollable, text, text_input, Space};
  |                                              ^^^^^^^^^

warning: unused imports: `Alignment` and `Task`
 --> src/gui/settings_dialog.rs:5:12
  |
5 | use iced::{Alignment, Element, Length, Task};
  |            ^^^^^^^^^                   ^^^^

warning: unused import: `ConfigUpdates`
 --> src/gui/settings_dialog.rs:8:32
  |
8 | use crate::config::{AppConfig, ConfigUpdates, InterfaceMode};
  |                                ^^^^^^^^^^^^^

warning: unused imports: `AddGameDialog` and `AddGameMessage`
  --> src/gui/mod.rs:11:27
   |
11 | pub use add_game_dialog::{AddGameDialog, AddGameMessage};
   |                           ^^^^^^^^^^^^^  ^^^^^^^^^^^^^^

warning: unused import: `confirmation_dialog::ConfirmationDialog`
  --> src/gui/mod.rs:12:9
   |
12 | pub use confirmation_dialog::ConfirmationDialog;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `LogViewerDialog` and `LogViewerMessage`
  --> src/gui/mod.rs:13:29
   |
13 | pub use log_viewer_dialog::{LogViewerDialog, LogViewerMessage};
   |                             ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^

warning: unused import: `main_window::MainWindow`
  --> src/gui/mod.rs:14:9
   |
14 | pub use main_window::MainWindow;
   |         ^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `ProtonManagerDialog` and `ProtonManagerMessage`
  --> src/gui/mod.rs:15:33
   |
15 | pub use proton_manager_dialog::{ProtonManagerDialog, ProtonManagerMessage};
   |                                 ^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `SettingsDialog` and `SettingsMessage`
  --> src/gui/mod.rs:16:27
   |
16 | pub use settings_dialog::{SettingsDialog, SettingsMessage};
   |                           ^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^

warning: unexpected `cfg` condition value: `gui`
   --> src/icons/icon_manager.rs:286:11
    |
286 |     #[cfg(feature = "gui")]
    |           ^^^^^^^^^^^^^^^ help: remove the condition
    |
    = note: no expected values for `feature`
    = help: consider adding `gui` as a feature in `Cargo.toml`
    = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration
    = note: `#[warn(unexpected_cfgs)]` on by default

warning: unused import: `IconSize`
 --> src/icons/mod.rs:6:37
  |
6 | pub use icon_manager::{IconManager, IconSize};
  |                                     ^^^^^^^^

warning: unused import: `tokio::sync::mpsc`
 --> src/launcher/launch_controller.rs:7:5
  |
7 | use tokio::sync::mpsc;
  |     ^^^^^^^^^^^^^^^^^

warning: unused import: `warn`
 --> src/launcher/launch_controller.rs:8:28
  |
8 | use tracing::{error, info, warn};
  |                            ^^^^

warning: unused imports: `GameLauncher` and `GameProcess`
 --> src/launcher/mod.rs:7:25
  |
7 | pub use game_launcher::{GameLauncher, GameProcess};
  |                         ^^^^^^^^^^^^  ^^^^^^^^^^^

warning: unused import: `std::fs`
 --> src/locale/i18n.rs:5:5
  |
5 | use std::fs;
  |     ^^^^^^^

warning: unused imports: `detect_language` and `get_system_locale`
 --> src/locale/mod.rs:6:16
  |
6 | pub use i18n::{detect_language, get_system_locale, I18n};
  |                ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^

warning: unused imports: `error` and `warn`
  --> src/proton/proton_manager.rs:13:15
   |
13 | use tracing::{error, info, warn};
   |               ^^^^^        ^^^^

warning: unused imports: `PROTON_CONFIGS` and `ProtonManager`
 --> src/proton/mod.rs:6:26
  |
6 | pub use proton_manager::{ProtonManager, PROTON_CONFIGS};
  |                          ^^^^^^^^^^^^^  ^^^^^^^^^^^^^^

warning: unused import: `DesktopEntry`
 --> src/shortcuts/mod.rs:6:25
  |
6 | pub use desktop_entry::{DesktopEntry, DesktopShortcutManager};
  |                         ^^^^^^^^^^^^

warning: unused import: `SteamShortcut`
 --> src/steam/mod.rs:6:21
  |
6 | pub use shortcuts::{SteamShortcut, SteamShortcuts};
  |                     ^^^^^^^^^^^^^

warning: unused import: `muda::Menu`
 --> src/tray/tray.rs:5:5
  |
5 | use muda::Menu;
  |     ^^^^^^^^^^

warning: unused import: `std::path::PathBuf`
 --> src/tray/tray.rs:6:5
  |
6 | use std::path::PathBuf;
  |     ^^^^^^^^^^^^^^^^^^

warning: unused imports: `error` and `warn`
 --> src/tray/tray.rs:8:15
  |
8 | use tracing::{error, info, warn};
  |               ^^^^^        ^^^^

warning: unused import: `icon::TrayIcon`
 --> src/tray/mod.rs:8:9
  |
8 | pub use icon::TrayIcon;
  |         ^^^^^^^^^^^^^^

warning: unused import: `menu::TrayMenu`
 --> src/tray/mod.rs:9:9
  |
9 | pub use menu::TrayMenu;
  |         ^^^^^^^^^^^^^^

warning: unused import: `warn`
  --> src/utils/components.rs:12:21
   |
12 | use tracing::{info, warn};
   |                     ^^^^

warning: unused import: `components::ComponentManager`
 --> src/utils/mod.rs:6:9
  |
6 | pub use components::ComponentManager;
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `LaunchStatus`
  --> src/main.rs:28:31
   |
28 | use launcher::{LaunchMessage, LaunchStatus};
   |                               ^^^^^^^^^^^^

warning: unused import: `TrayMessage`
  --> src/main.rs:33:47
   |
33 | use tray::{SystemTray, TrayConfig, TrayEvent, TrayMessage};
   |                                               ^^^^^^^^^^^

error[E0072]: recursive types `Message` and `ConfirmationDialog` have infinite size
  --> src/main.rs:39:1
   |
39 | pub enum Message {
   | ^^^^^^^^^^^^^^^^
...
77 |     ShowConfirmationDialog(ConfirmationDialog),
   |                            ------------------ recursive without indirection
   |
  ::: src/gui/confirmation_dialog.rs:12:1
   |
12 | pub struct ConfirmationDialog {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
18 |     on_confirm: Message,
   |                 ------- recursive without indirection
   |

error[E0308]: mismatched types
   --> src/gui/add_game_dialog.rs:197:32
    |
197 |         dialog.lossless_flow = game.lossless_flow;
    |         --------------------   ^^^^^^^^^^^^^^^^^^ expected `u32`, found `bool`
    |         |
    |         expected due to the type of this binding

error[E0308]: mismatched types
   --> src/gui/add_game_dialog.rs:436:28
    |
436 |             lossless_flow: self.lossless_flow,
    |                            ^^^^^^^^^^^^^^^^^^ expected `bool`, found `u32`

error[E0599]: no method named `placeholder` found for struct `iced::widget::TextInput<'a, Message, Theme, Renderer>` in the current scope
   --> src/gui/add_game_dialog.rs:510:18
    |
508 | /             text_input("", &self.game_title)
509 | |                 .on_input(AddGameMessage::TitleChanged)
510 | |                 .placeholder("Game Title"),
    | |                 -^^^^^^^^^^^ private field, not a method
    | |_________________|
    |

error[E0599]: no method named `placeholder` found for struct `iced::widget::TextInput<'a, Message, Theme, Renderer>` in the current scope
   --> src/gui/add_game_dialog.rs:525:22
    |
523 | /                 text_input("", &path_display)
524 | |                     .on_input(AddGameMessage::PathChanged)
525 | |                     .placeholder("/path/to/the/exe"),
    | |                     -^^^^^^^^^^^ private field, not a method
    | |_____________________|
    |

error[E0599]: no method named `placeholder` found for struct `iced::widget::TextInput<'a, Message, Theme, Renderer>` in the current scope
   --> src/gui/add_game_dialog.rs:545:22
    |
543 | /                 text_input("", &prefix_display)
544 | |                     .on_input(AddGameMessage::PrefixChanged)
545 | |                     .placeholder("/path/to/the/prefix"),
    | |                     -^^^^^^^^^^^ private field, not a method
    | |_____________________|
    |

error[E0599]: no method named `placeholder` found for struct `iced::widget::TextInput<'a, Message, Theme, Renderer>` in the current scope
   --> src/gui/add_game_dialog.rs:583:22
    |
581 | /                 text_input("", &self.protonfix)
582 | |                     .on_input(AddGameMessage::ProtonfixChanged)
583 | |                     .placeholder("UMU ID"),
    | |                     -^^^^^^^^^^^ private field, not a method
    | |_____________________|
    |

error[E0599]: no method named `placeholder` found for struct `iced::widget::TextInput<'a, Message, Theme, Renderer>` in the current scope
   --> src/gui/add_game_dialog.rs:602:22
    |
600 | /                 text_input("", &self.launch_arguments)
601 | |                     .on_input(AddGameMessage::LaunchArgumentsChanged)
602 | |                     .placeholder("e.g.: PROTON_USE_WINED3D=1 gamescope -W 2560 -H 1440"),
    | |                     -^^^^^^^^^^^ private field, not a method
    | |_____________________|
    |

error[E0599]: no method named `placeholder` found for struct `iced::widget::TextInput<'a, Message, Theme, Renderer>` in the current scope
   --> src/gui/add_game_dialog.rs:610:22
    |
608 | /                 text_input("", &self.game_arguments)
609 | |                     .on_input(AddGameMessage::GameArgumentsChanged)
610 | |                     .placeholder("e.g.: -d3d11 -fullscreen"),
    | |                     -^^^^^^^^^^^ private field, not a method
    | |_____________________|
    |

error[E0277]: the trait bound `iced::advanced::iced_graphics::iced_core::Element<'_, _, _, _>: From<std::string::String>` is not satisfied
   --> src/gui/add_game_dialog.rs:627:25
    |
627 |             row![button(i18n.t("Lossless Scaling Frame Generation"))
    |                  ------ ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `From<std::string::String>` is not implemented for `iced::advanced::iced_graphics::iced_core::Element<'_, _, _, _>`
    |                  |
    |                  required by a bound introduced by this call
    |
    = help: the following other types implement trait `From<T>`:
              `iced::advanced::iced_graphics::iced_core::Element<'_, Link, Theme, Renderer>` implements `From<Rich<'_, Link, Theme, Renderer>>`
              `iced::advanced::iced_graphics::iced_core::Element<'_, Message, Theme, Renderer>` implements `From<&str>`
              `iced::advanced::iced_graphics::iced_core::Element<'_, Message, Theme, Renderer>` implements `From<Checkbox<'_, Message, Theme, Renderer>>`
              `iced::advanced::iced_graphics::iced_core::Element<'_, Message, Theme, Renderer>` implements `From<ComboBox<'_, T, Message, Theme, Renderer>>`
              `iced::advanced::iced_graphics::iced_core::Element<'_, Message, Theme, Renderer>` implements `From<Container<'_, Message, Theme, Renderer>>`
              `iced::advanced::iced_graphics::iced_core::Element<'_, Message, Theme, Renderer>` implements `From<MouseArea<'_, Message, Theme, Renderer>>`
              `iced::advanced::iced_graphics::iced_core::Element<'_, Message, Theme, Renderer>` implements `From<PaneGrid<'_, Message, Theme, Renderer>>`
              `iced::advanced::iced_graphics::iced_core::Element<'_, Message, Theme, Renderer>` implements `From<PickList<'_, T, L, V, Message, Theme, Renderer>>`
            and 24 others
    = note: required for `std::string::String` to implement `Into<iced::advanced::iced_graphics::iced_core::Element<'_, _, _, _>>`
note: required by a bound in `button`
   --> /home/stfu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/iced_widget-0.13.4/src/helpers.rs:835:19
    |
834 | pub fn button<'a, Message, Theme, Renderer>(
    |        ------ required by a bound in this function
835 |     content: impl Into<Element<'a, Message, Theme, Renderer>>,
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `button`

error[E0308]: arguments to this function are incorrect
   --> src/gui/main_window.rs:169:28
    |
169 |                     return Task::chain(task, |msg| Task::done(Message::LaunchMessage(msg)));
    |                            ^^^^^^^^^^^
    |
note: expected `Task<Message>`, found `Task<LaunchMessage>`
   --> src/gui/main_window.rs:169:40
    |
169 |                     return Task::chain(task, |msg| Task::done(Message::LaunchMessage(msg)));
    |                                        ^^^^
    = note: expected struct `Task<Message>`
               found struct `Task<LaunchMessage>`
note: expected `Task<Message>`, found closure
   --> src/gui/main_window.rs:169:46
    |
169 |                     return Task::chain(task, |msg| Task::done(Message::LaunchMessage(msg)));
    |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: expected struct `Task<Message>`
              found closure `{closure@src/gui/main_window.rs:169:46: 169:51}`
note: method defined here
   --> /home/stfu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/iced_runtime-0.13.2/src/task.rs:113:12
    |
113 |     pub fn chain(self, task: Self) -> Self
    |            ^^^^^
help: use parentheses to call this closure
    |
169 |                     return Task::chain(task, (|msg| Task::done(Message::LaunchMessage(msg)))(/* LaunchMessage */));
    |                                              +                                             ++++++++++++++++++++++

error[E0308]: arguments to this function are incorrect
   --> src/gui/main_window.rs:178:32
    |
178 |                         return Task::chain(task, |msg| Task::done(Message::LaunchMessage(msg)));
    |                                ^^^^^^^^^^^
    |
note: expected `Task<Message>`, found `Task<LaunchMessage>`
   --> src/gui/main_window.rs:178:44
    |
178 |                         return Task::chain(task, |msg| Task::done(Message::LaunchMessage(msg)));
    |                                            ^^^^
    = note: expected struct `Task<Message>`
               found struct `Task<LaunchMessage>`
note: expected `Task<Message>`, found closure
   --> src/gui/main_window.rs:178:50
    |
178 |                         return Task::chain(task, |msg| Task::done(Message::LaunchMessage(msg)));
    |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: expected struct `Task<Message>`
              found closure `{closure@src/gui/main_window.rs:178:50: 178:55}`
note: method defined here
   --> /home/stfu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/iced_runtime-0.13.2/src/task.rs:113:12
    |
113 |     pub fn chain(self, task: Self) -> Self
    |            ^^^^^
help: use parentheses to call this closure
    |
178 |                         return Task::chain(task, (|msg| Task::done(Message::LaunchMessage(msg)))(/* LaunchMessage */));
    |                                                  +                                             ++++++++++++++++++++++

error[E0277]: expected a `Fn(&_)` closure, found `iced::Color`
   --> src/gui/main_window.rs:361:32
    |
361 |                         .style(iced::Color::from_rgb(0.6, 0.6, 0.6))
    |                          ----- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an `Fn(&_)` closure, found `iced::Color`
    |                          |
    |                          required by a bound introduced by this call
    |
    = help: the trait `for<'a> Fn(&'a _)` is not implemented for `iced::Color`
note: required by a bound in `iced::advanced::widget::Text::<'a, Theme, Renderer>::style`
   --> /home/stfu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/iced_core-0.13.2/src/widget/text.rs:170:40
    |
170 |     pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self
    |                                        ^^^^^^^^^^^^^^^^^^^ required by this bound in `Text::<'a, Theme, Renderer>::style`

error[E0599]: no method named `on_double_press` found for struct `MouseArea<'a, Message, Theme, Renderer>` in the current scope
   --> src/gui/main_window.rs:389:22
    |
387 | /                 mouse_area(container)
388 | |                     .on_press(Message::GameClicked(index))
389 | |                     .on_double_press(Message::GameDoubleClicked(index))
    | |_____________________-^^^^^^^^^^^^^^^
    |
help: there is a method `on_press` with a similar name
    |
389 -                     .on_double_press(Message::GameDoubleClicked(index))
389 +                     .on_press(Message::GameDoubleClicked(index))
    |

error[E0599]: no method named `on_double_press` found for struct `MouseArea<'a, Message, Theme, Renderer>` in the current scope
   --> src/gui/main_window.rs:455:26
    |
453 | /                     mouse_area(container)
454 | |                         .on_press(Message::GameClicked(index))
455 | |                         .on_double_press(Message::GameDoubleClicked(index))
    | |_________________________-^^^^^^^^^^^^^^^
    |
help: there is a method `on_press` with a similar name
    |
455 -                         .on_double_press(Message::GameDoubleClicked(index))
455 +                         .on_press(Message::GameDoubleClicked(index))
    |

error[E0277]: expected a `Fn(&_)` closure, found `iced::Color`
   --> src/gui/main_window.rs:502:32
    |
502 |                         .style(iced::Color::from_rgb(0.6, 0.6, 0.6))
    |                          ----- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an `Fn(&_)` closure, found `iced::Color`
    |                          |
    |                          required by a bound introduced by this call
    |
    = help: the trait `for<'a> Fn(&'a _)` is not implemented for `iced::Color`
note: required by a bound in `iced::advanced::widget::Text::<'a, Theme, Renderer>::style`
   --> /home/stfu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/iced_core-0.13.2/src/widget/text.rs:170:40
    |
170 |     pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self
    |                                        ^^^^^^^^^^^^^^^^^^^ required by this bound in `Text::<'a, Theme, Renderer>::style`

error[E0599]: no method named `on_double_press` found for struct `MouseArea<'a, Message, Theme, Renderer>` in the current scope
   --> src/gui/main_window.rs:529:22
    |
527 | /                 mouse_area(container)
528 | |                     .on_press(Message::GameClicked(index))
529 | |                     .on_double_press(Message::GameDoubleClicked(index))
    | |_____________________-^^^^^^^^^^^^^^^
    |
help: there is a method `on_press` with a similar name
    |
529 -                     .on_double_press(Message::GameDoubleClicked(index))
529 +                     .on_press(Message::GameDoubleClicked(index))
    |

error[E0609]: no field `show_hidden_games` on type `AppConfig`
   --> src/gui/main_window.rs:659:25
    |
659 |             self.config.show_hidden_games
    |                         ^^^^^^^^^^^^^^^^^ unknown field
    |
    = note: available fields are: `close_on_launch`, `default_prefix`, `mangohud`, `gamemode`, `disable_hidraw` ... and 19 others

error[E0282]: type annotations needed
   --> src/gui/proton_manager_dialog.rs:156:43
    |
156 | ...                   let release = manager.get_latest_release(config).await.ok();
    |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type

error[E0282]: type annotations needed
   --> src/gui/proton_manager_dialog.rs:191:31
    |
191 |                         .any(|v| v.to_lowercase() == tag_name.to_lowercase());
    |                               ^  - type must be known at this point
    |
help: consider giving this closure parameter an explicit type
    |
191 |                         .any(|v: /* Type */| v.to_lowercase() == tag_name.to_lowercase());
    |                                ++++++++++++

error[E0308]: mismatched types
   --> src/icons/icon_manager.rs:163:63
    |
163 |                 if let Some(largest) = Self::find_largest_png(png_path.parent().unwrap()) {
    |                                        ---------------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&PathBuf`, found `&Path`
    |                                        |
    |                                        arguments to this function are incorrect
    |
    = note: expected reference `&PathBuf`
               found reference `&std::path::Path`
note: associated function defined here
   --> src/icons/icon_manager.rs:199:8
    |
199 |     fn find_largest_png(dir: &PathBuf) -> Option<PathBuf> {
    |        ^^^^^^^^^^^^^^^^ -------------

error[E0369]: binary operation `==` cannot be applied to type `&GameProcess`
  --> src/launcher/launch_controller.rs:18:13
   |
14 | #[derive(Debug, Clone, PartialEq)]
   |                        --------- in this derive macro expansion
...
18 |     Running(GameProcess),
   |             ^^^^^^^^^^^
   |
note: an implementation of `PartialEq` might be missing for `GameProcess`
  --> src/launcher/game_launcher.rs:16:1
   |
16 | pub struct GameProcess {
   | ^^^^^^^^^^^^^^^^^^^^^^ must implement `PartialEq`
help: consider annotating `GameProcess` with `#[derive(PartialEq)]`
  --> src/launcher/game_launcher.rs:16:1
   |
16 + #[derive(PartialEq)]
17 | pub struct GameProcess {
   |

error[E0658]: use of unstable library feature `str_as_str`
   --> src/locale/i18n.rs:607:51
    |
607 |                 if !translations.contains_key(key.as_str()) {
    |                                                   ^^^^^^
    |
    = note: see issue #130366 <https://github.com/rust-lang/rust/issues/130366> for more information

error[E0308]: mismatched types
  --> src/shortcuts/desktop_entry.rs:99:37
   |
99 |             if game.lossless_flow > 0 {
   |                ------------------   ^ expected `bool`, found integer
   |                |
   |                expected because this is `bool`

error[E0599]: no method named `context` found for enum `serde_json::Value` in the current scope
  --> src/steam/shortcuts.rs:86:14
   |
85 |           let shortcuts_value = new_vdf_parser::open_shortcuts_vdf(&vdf_path)
   |  _______________________________-
86 | |             .context("Failed to parse shortcuts.vdf")?;
   | |             -^^^^^^^ method not found in `serde_json::Value`
   | |_____________|
   |

error[E0282]: type annotations needed
  --> src/steam/shortcuts.rs:89:13
   |
89 |             obj.clone()
   |             ^^^ cannot infer type

error[E0599]: no function or associated item named `from_rgba` found for struct `tray_icon::TrayIcon` in the current scope
   --> src/tray/tray.rs:95:46
    |
 95 |         let tray_icon = tray_icon::TrayIcon::from_rgba(icon_bytes, None, None)
    |                                              ^^^^^^^^^ function or associated item not found in `tray_icon::TrayIcon`
    |
note: if you're trying to build a new `tray_icon::TrayIcon` consider using one of the following associated functions:
      tray_icon::TrayIcon::new
      tray_icon::TrayIcon::with_id
   --> /home/stfu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tray-icon-0.21.3/src/lib.rs:338:5
    |
338 |     pub fn new(attrs: TrayIconAttributes) -> Result<Self> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
352 |     pub fn with_id<I: Into<TrayIconId>>(id: I, attrs: TrayIconAttributes) -> Result<Self> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0277]: the `?` operator can only be used on `Result`s, not `Option`s, in a method that returns `Result`
   --> src/tray/tray.rs:120:38
    |
113 |     pub fn handle_message(&mut self, message: TrayMessage) -> Result<()> {
    |     -------------------------------------------------------------------- this function returns a `Result`
...
120 |                 self.update_tooltip()?;
    |                                      ^ use `.ok_or(...)?` to provide an error compatible with `std::result::Result<(), anyhow::Error>`

error[E0308]: mismatched types
   --> src/tray/tray.rs:129:43
    |
129 |                     tray_icon.set_tooltip(&tooltip);
    |                               ----------- ^^^^^^^^ expected `Option<_>`, found `&String`
    |                               |
    |                               arguments to this method are incorrect
    |
    = note:   expected enum `std::option::Option<_>`
            found reference `&std::string::String`
note: method defined here
   --> /home/stfu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tray-icon-0.21.3/src/lib.rs:387:12
    |
387 |     pub fn set_tooltip<S: AsRef<str>>(&self, tooltip: Option<S>) -> Result<()> {
    |            ^^^^^^^^^^^
help: try wrapping the expression in `Some`
    |
129 |                     tray_icon.set_tooltip(Some(&tooltip));
    |                                           +++++        +

error[E0308]: mismatched types
   --> src/tray/tray.rs:147:35
    |
147 |             tray_icon.set_tooltip(&tooltip);
    |                       ----------- ^^^^^^^^ expected `Option<_>`, found `&String`
    |                       |
    |                       arguments to this method are incorrect
    |
    = note:   expected enum `std::option::Option<_>`
            found reference `&std::string::String`
note: method defined here
   --> /home/stfu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tray-icon-0.21.3/src/lib.rs:387:12
    |
387 |     pub fn set_tooltip<S: AsRef<str>>(&self, tooltip: Option<S>) -> Result<()> {
    |            ^^^^^^^^^^^
help: try wrapping the expression in `Some`
    |
147 |             tray_icon.set_tooltip(Some(&tooltip));
    |                                   +++++        +

error[E0599]: no method named `show_notification` found for reference `&tray_icon::TrayIcon` in the current scope
   --> src/tray/tray.rs:158:23
    |
158 |             tray_icon.show_notification(title, body);
    |                       ^^^^^^^^^^^^^^^^^ method not found in `&tray_icon::TrayIcon`

error[E0624]: method `validate` is private
   --> src/main.rs:247:43
    |
247 | ...                   if dialog.validate() {
    |                                 ^^^^^^^^ private method
    |
   ::: src/gui/add_game_dialog.rs:334:5
    |
334 |     fn validate(&mut self) -> bool {
    |     ------------------------------ private method defined here

warning: unused variable: `parts`
  --> src/config/paths.rs:45:13
   |
45 |         let parts: Vec<&str> = relative_path.split('/').collect();
   |             ^^^^^ help: if this is intentional, prefix it with an underscore: `_parts`
   |
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

error[E0391]: cycle detected when computing drop-check constraints for `gui::confirmation_dialog::ConfirmationDialog`
  --> src/gui/confirmation_dialog.rs:12:1
   |
12 | pub struct ConfirmationDialog {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: ...which requires computing drop-check constraints for `Message`...
  --> src/main.rs:39:1
   |
39 | pub enum Message {
   | ^^^^^^^^^^^^^^^^
   = note: ...which again requires computing drop-check constraints for `gui::confirmation_dialog::ConfirmationDialog`, completing the cycle
   = note: cycle used when computing dropck types for `gui::confirmation_dialog::ConfirmationDialog`
   = note: see https://rustc-dev-guide.rust-lang.org/overview.html#queries and https://rustc-dev-guide.rust-lang.org/query.html for more information

error: lifetime may not live long enough
   --> src/gui/proton_manager_dialog.rs:439:9
    |
408 |           &self,
    |           - let's call the lifetime of this reference `'2`
409 |           release: &ProtonVersionEntry,
    |                    - let's call the lifetime of this reference `'1`
...
439 | /         row![
440 | |             version_text,
441 | |             Space::with_width(Length::Fill),
442 | |             size_text,
...   |
448 | |         .align_y(Alignment::Center)
449 | |         .into()
    | |_______________^ method was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
    |
help: consider introducing a named lifetime parameter and update trait if needed
    |
407 ~     fn view_release_row<'a>(
408 |         &self,
409 ~         release: &'a ProtonVersionEntry,
410 |         i18n: &I18n,
411 ~     ) -> Element<'a, ProtonManagerMessage> {
    |

warning: unused variable: `i18n`
   --> src/gui/proton_manager_dialog.rs:410:9
    |
410 |         i18n: &I18n,
    |         ^^^^ help: if this is intentional, prefix it with an underscore: `_i18n`

warning: unused variable: `i18n`
   --> src/gui/proton_manager_dialog.rs:453:37
    |
453 |     fn view_progress_section(&self, i18n: &I18n) -> Element<ProtonManagerMessage> {
    |                                     ^^^^ help: if this is intentional, prefix it with an underscore: `_i18n`

warning: unused variable: `name`
   --> src/gui/settings_dialog.rs:165:33
    |
165 |                 if let Some((_, name)) = self.languages.iter().find(|(c, _)| c == &code) {
    |                                 ^^^^ help: if this is intentional, prefix it with an underscore: `_name`

warning: unused variable: `mangohud`
   --> src/launcher/game_launcher.rs:116:25
    |
116 |             if let Some(mangohud) = Paths::mangohud() {
    |                         ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_mangohud`

warning: unused variable: `log_file`
   --> src/launcher/game_launcher.rs:185:21
    |
185 |                 let log_file = Paths::logs_dir().join(format!("{}.log", game.gameid));
    |                     ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_log_file`

warning: unused variable: `locale_paths`
  --> src/locale/i18n.rs:32:13
   |
32 |         let locale_paths = vec![
   |             ^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_locale_paths`

error[E0382]: use of moved value: `relative`
  --> src/locale/i18n.rs:54:64
   |
49 |     fn get_locale_dir(relative: String) -> PathBuf {
   |                       -------- move occurs because `relative` has type `std::string::String`, which does not implement the `Copy` trait
...
53 |             Some(PathBuf::from("/usr/share/locale").join(relative)),
   |                                                          -------- value moved here
54 |             Some(PathBuf::from("/usr/local/share/locale").join(relative)),
   |                                                                ^^^^^^^^ value used here after move
   |
help: consider borrowing `relative`
   |
53 |             Some(PathBuf::from("/usr/share/locale").join(&relative)),
   |                                                          +

error[E0382]: borrow of moved value: `relative`
  --> src/locale/i18n.rs:66:48
   |
49 |     fn get_locale_dir(relative: String) -> PathBuf {
   |                       -------- move occurs because `relative` has type `std::string::String`, which does not implement the `Copy` trait
...
54 |             Some(PathBuf::from("/usr/local/share/locale").join(relative)),
   |                                                                -------- value moved here
...
66 |         Paths::user_data(&format!("locale/{}", relative))
   |                                                ^^^^^^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::__export::format_args` which comes from the expansion of the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider borrowing `relative`
   |
54 |             Some(PathBuf::from("/usr/local/share/locale").join(&relative)),
   |                                                                +

warning: unused variable: `config`
   --> src/proton/proton_manager.rs:191:61
    |
191 |     async fn extract_archive(&self, archive_path: &PathBuf, config: &ProtonConfig) -> Result<()> {
    |                                                             ^^^^^^ help: if this is intentional, prefix it with an underscore: `_config`

warning: variable does not need to be mutable
   --> src/steam/shortcuts.rs:189:36
    |
189 |         if let Some((existing_key, mut existing_obj)) = self.find_shortcut(&game.title) {
    |                                    ----^^^^^^^^^^^^
    |                                    |
    |                                    help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `existing_obj`
   --> src/steam/shortcuts.rs:189:36
    |
189 |         if let Some((existing_key, mut existing_obj)) = self.find_shortcut(&game.title) {
    |                                    ^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_existing_obj`

warning: unused variable: `key`
   --> src/steam/shortcuts.rs:286:14
    |
286 |         for (key, value) in &self.shortcuts {
    |              ^^^ help: if this is intentional, prefix it with an underscore: `_key`

Some errors have detailed explanations: E0072, E0277, E0282, E0308, E0369, E0382, E0391, E0433, E0599...
For more information about an error, try `rustc --explain E0072`.
warning: `faugus-launcher-rs` (bin "faugus-launcher-rs") generated 47 warnings
error: could not compile `faugus-launcher-rs` (bin "faugus-launcher-rs") due to 70 previous errors; 47 warnings emitted
