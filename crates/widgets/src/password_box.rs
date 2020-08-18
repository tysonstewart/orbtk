use super::behaviors::TextBehavior;
use crate::prelude::*;
use crate::shell::prelude::KeyEvent;
use crate::{api::prelude::*, proc_macros::*, theme::prelude::*};

enum PasswordAction {
    Key(KeyEvent),
}

#[derive(Default, AsAny)]
struct PasswordBoxState {
    action: Option<PasswordAction>,
    echo: char,
}

impl PasswordBoxState {
    fn action(&mut self, action: PasswordAction) {
        self.action = Some(action);
    }

    fn mask(&mut self, ctx: &mut Context) {
        let mut new_prompt = String16::new();

        for _ in ctx.widget().get::<String16>("text").as_string().chars() {
            new_prompt.push(self.echo);
        }

        ctx.widget().set::<String16>("mask", new_prompt);
    }
}

impl State for PasswordBoxState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.echo = ctx.widget().clone("echo");
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(action) = &self.action {
            match action {
                PasswordAction::Key(_key_event) => {
                    self.mask(ctx);
                }
            }

            self.action = None;
        }
    }
}

widget!(
    /// The PasswordBox is a specialised [`TextBox`] masking its input.
    ///
    /// It is for use cases when users needs to enter sensitive data
    /// (like passwords, credit card PIN-codes, etc) that should not be readable directly on the display.
    /// PasswordBox masks its input with the `echo` char (the default value is an asterisk).
    ///
    /// The value typed in the `PasswordBox` can be obtained through the `text` property.
    /// You can process this value in your application and set the authentication logic as appropriate.
    /// It is a good practice to clear the content of the `text` property after the value is used.
    ///
    /// Notes:
    /// * If the input is empty, it will render the content of the `water_mark` property.
    /// * Changing the `echo` property after the `PasswordBox` is created has no effect.
    /// * The password is stored in plain text currently
    ///
    /// For an example how to use the PasswordBox, check the [`example`].
    ///
    /// [`TextBox`]: ./struct.TextBox.html
    /// [`example`]: https://github.com/redox-os/orbtk/tree/develop/examples/login.rs
    PasswordBox<PasswordBoxState>: KeyDownHandler {
        /// Sets or shares the echo character which used to mask the input
        echo: char,

        /// Sets or shares the mask property.It contains the masked input.
        mask: String16,

        /// Sets or shares the text property.It holds the password.
        text: String16,

        /// Sets or shares the water_mark text property.
        water_mark: String16,

        /// Sets or shares the text selection property.
        text_selection: TextSelection,

        /// Sets or shares the foreground property.
        foreground: Brush,

        /// Sets or shares the font size property.
        font_size: f64,

        /// Sets or shares the font property.
        font: String,

        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the border radius property.
        border_radius: f64,

        /// Sets or shares the border thickness property.
        border_width: Thickness,

        /// Sets or shares the border brush property.
        border_brush: Brush,

        /// Sets or shares the padding property.
        padding: Thickness,

        /// Sets or shares the focused property.
        focused: bool,

        /// Sets or shares ta value that describes if the PasswordBox should lost focus on activation (when Enter pressed).
        lost_focus_on_activation: bool,

        /// Used to request focus from outside. Set to `true` tor request focus.
        request_focus: bool
    }
);

impl Template for PasswordBox {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let text_block = TextBlock::new()
            .v_align("center")
            .h_align("start")
            .foreground(id)
            .text(("mask", id))
            .water_mark(id)
            .font(id)
            .font_size(id)
            .build(ctx);

        let cursor = Cursor::new()
            .h_align("start")
            .text_block(text_block.0)
            .focused(id)
            .text_selection(id)
            .build(ctx);

        let text_behavior = TextBehavior::new()
            .cursor(cursor.0)
            .focused(id)
            .font(id)
            .font_size(id)
            .lost_focus_on_activation(id)
            .target(id.0)
            .request_focus(id)
            .text(id)
            .text_selection(id)
            .build(ctx);

        self.name("PasswordBox")
            .style(STYLE_TEXT_BOX)
            .echo('*')
            .text("")
            .mask("")
            .water_mark("Password")
            .on_changed_filter(vec!["text", "mask"])
            .foreground(colors::LINK_WATER_COLOR)
            .font_size(fonts::FONT_SIZE_12)
            .font("Roboto-Regular")
            .text_selection(TextSelection::default())
            .padding(4.0)
            .background(colors::LYNCH_COLOR)
            .border_brush("transparent")
            .border_width(0.0)
            .border_radius(2.0)
            .min_width(128.0)
            .height(32.0)
            .focused(false)
            .lost_focus_on_activation(true)
            .child(text_behavior)
            .child(
                Container::new()
                    .background(id)
                    .border_radius(id)
                    .border_width(id)
                    .border_brush(id)
                    .padding(id)
                    .child(
                        Grid::new()
                            .clip(true)
                            // It is important that cursor is the first child
                            // should be refactored in the future.
                            .child(cursor)
                            .child(text_block)
                            .build(ctx),
                    )
                    .build(ctx),
            )
            .on_key_down(move |states, event| -> bool {
                states
                    .get_mut::<PasswordBoxState>(id)
                    .action(PasswordAction::Key(event));
                false
            })
    }
}
