// Confirmation dialog
// Reusable dialog for confirming critical actions

use iced::widget::{button, column, container, row, text, Space};
use iced::{Alignment, Element, Length};

use crate::locale::I18n;
use crate::Message;

/// Confirmation dialog state
#[derive(Debug, Clone)]
pub struct ConfirmationDialog {
    /// Dialog title
    title: String,
    /// Dialog message
    message: String,
    /// Callback when confirmed
    on_confirm: Message,
    /// Callback when cancelled
    on_cancel: Message,
}

impl ConfirmationDialog {
    /// Create a new confirmation dialog
    pub fn new(title: String, message: String, on_confirm: Message, on_cancel: Message) -> Self {
        Self {
            title,
            message,
            on_confirm,
            on_cancel,
        }
    }

    /// Create a delete confirmation dialog
    pub fn delete_confirmation(
        game_title: String,
        on_confirm: Message,
        on_cancel: Message,
    ) -> Self {
        let message = format!(
            "{}: {}\n{}",
            crate::locale::i18n::I18n::default().t("Game"),
            game_title,
            crate::locale::i18n::I18n::default().t("This action cannot be undone.")
        );

        Self::new(
            crate::locale::i18n::I18n::default().t("Confirm"),
            message,
            on_confirm,
            on_cancel,
        )
    }

    /// Create a custom confirmation dialog
    pub fn custom(
        title: impl Into<String>,
        message: impl Into<String>,
        on_confirm: Message,
        on_cancel: Message,
    ) -> Self {
        Self::new(title.into(), message.into(), on_confirm, on_cancel)
    }

    /// View the confirmation dialog
    pub fn view(&self, i18n: &I18n) -> Element<Message> {
        // Background overlay
        let background = container(Space::new(Length::Fill, Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(iced::theme::Container::Transparent);

        // Dialog container
        let dialog = container(
            column![
                // Title
                container(text(&self.title).size(20))
                    .padding(20)
                    .width(Length::Fill)
                    .align_x(iced::alignment::Horizontal::Center),
                // Message
                container(text(&self.message).size(14))
                    .padding(20)
                    .width(Length::Fill),
                // Buttons
                row![
                    Space::new(Length::Fill, Length::Shrink),
                    button(text(i18n.t("No")).size(14))
                        .on_press(self.on_cancel.clone())
                        .padding(10)
                        .width(100),
                    button(text(i18n.t("Yes")).size(14))
                        .on_press(self.on_confirm.clone())
                        .padding(10)
                        .width(100),
                ]
                .spacing(10)
                .padding(20)
                .align_y(Alignment::Center),
            ]
            .spacing(10)
            .width(400)
            .max_width(400),
        )
        .padding(20)
        .style(iced::theme::Container::Box);

        // Stack background and dialog
        container(row![background, dialog])
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confirmation_dialog_creation() {
        let dialog = ConfirmationDialog::new(
            "Test Title".to_string(),
            "Test Message".to_string(),
            Message::CloseAddGameDialog,
            Message::CloseSettingsDialog,
        );

        assert_eq!(dialog.title, "Test Title");
        assert_eq!(dialog.message, "Test Message");
    }
}
