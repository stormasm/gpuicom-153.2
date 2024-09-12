use gpui::{
    anchored, canvas, deferred, div, prelude::FluentBuilder as _, px, relative, AnchorCorner,
    AppContext, Bounds, ElementId, EventEmitter, FocusHandle, FocusableView, Hsla,
    InteractiveElement as _, IntoElement, KeyBinding, MouseButton, ParentElement, Pixels, Point,
    Render, SharedString, StatefulInteractiveElement as _, Styled, View, ViewContext,
    VisualContext,
};

use crate::{
    divider::Divider,
    h_flex,
    input::{InputEvent, TextInput},
    popover::Escape,
    theme::{ActiveTheme as _, Colorize},
    tooltip::Tooltip,
    v_flex, ColorExt as _, Sizable, Size, StyleSized,
};

const KEY_CONTEXT: &'static str = "ColorPicker";

pub fn init(cx: &mut AppContext) {
    cx.bind_keys([KeyBinding::new("escape", Escape, Some(KEY_CONTEXT))])
}

#[derive(Clone)]
pub enum ColorPickerEvent {
    Change(Option<Hsla>),
}

fn color_palettes() -> Vec<Vec<Hsla>> {
    use crate::colors::DEFAULT_COLOR;
    use itertools::Itertools as _;

    macro_rules! c {
        ($color:tt) => {
            DEFAULT_COLOR
                .$color
                .keys()
                .sorted()
                .map(|k| DEFAULT_COLOR.$color.get(k).map(|c| c.hsla).unwrap())
                .collect::<Vec<_>>()
        };
    }

    vec![
        c!(stone),
        c!(red),
        c!(orange),
        c!(yellow),
        c!(green),
        c!(cyan),
        c!(blue),
        c!(purple),
        c!(pink),
    ]
}

pub struct ColorPicker {
    id: ElementId,
    focus_handle: FocusHandle,
    value: Option<Hsla>,
    featured_colors: Vec<Hsla>,
    hovered_color: Option<Hsla>,
    label: Option<SharedString>,
    size: Size,
    anchor: AnchorCorner,
    color_input: View<TextInput>,

    open: bool,
    bounds: Bounds<Pixels>,
}

impl ColorPicker {
    pub fn new(id: impl Into<ElementId>, cx: &mut ViewContext<Self>) -> Self {
        let color_input = cx.new_view(|cx| TextInput::new(cx).xsmall());

        cx.subscribe(&color_input, |this, _, ev: &InputEvent, cx| match ev {
            InputEvent::Change(value) => {
                if let Ok(color) = Hsla::parse_hex_string(value) {
                    this.value = Some(color);
                    this.hovered_color = Some(color);
                }
            }
            InputEvent::PressEnter => {
                let val = this.color_input.read(cx).text();
                if let Ok(color) = Hsla::parse_hex_string(&val) {
                    this.open = false;
                    this.update_value(Some(color), true, cx);
                }
            }
            _ => {}
        })
        .detach();

        Self {
            id: id.into(),
            focus_handle: cx.focus_handle(),
            featured_colors: vec![
                crate::black(),
                crate::gray_600(),
                crate::gray_400(),
                crate::white(),
                crate::red_600(),
                crate::orange_600(),
                crate::yellow_600(),
                crate::green_600(),
                crate::blue_600(),
                crate::indigo_600(),
                crate::purple_600(),
            ],
            value: None,
            hovered_color: None,
            size: Size::Medium,
            label: None,
            anchor: AnchorCorner::TopLeft,
            color_input,
            open: false,
            bounds: Bounds::default(),
        }
    }

    /// Set the featured colors to be displayed in the color picker.
    ///
    /// This is used to display a set of colors that the user can quickly select from,
    /// for example provided user's last used colors.
    pub fn featured_colors(mut self, colors: Vec<Hsla>) -> Self {
        self.featured_colors = colors;
        self
    }

    /// Set current color value.
    pub fn set_value(&mut self, value: Hsla, cx: &mut ViewContext<Self>) {
        self.update_value(Some(value), false, cx)
    }

    /// Set the size of the color picker, default is `Size::Medium`.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Set the label to be displayed above the color picker.
    ///
    /// Default is `None`.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the anchor corner of the color picker.
    ///
    /// Default is `AnchorCorner::TopLeft`.
    pub fn anchor(mut self, anchor: AnchorCorner) -> Self {
        self.anchor = anchor;
        self
    }

    fn on_escape(&mut self, _: &Escape, cx: &mut ViewContext<Self>) {
        self.open = false;
        cx.notify();
    }

    fn toggle_picker(&mut self, _: &gpui::ClickEvent, cx: &mut ViewContext<Self>) {
        self.open = !self.open;
        cx.notify();
    }

    fn update_value(&mut self, value: Option<Hsla>, emit: bool, cx: &mut ViewContext<Self>) {
        self.value = value;
        self.hovered_color = value;
        self.color_input.update(cx, |view, cx| {
            if let Some(value) = value {
                view.set_text(value.to_hex_string(), cx);
            } else {
                view.set_text("", cx);
            }
        });
        if emit {
            cx.emit(ColorPickerEvent::Change(value));
        }
        cx.notify();
    }

    fn render_item(
        &self,
        color: Hsla,
        clickable: bool,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        div()
            .id(SharedString::from(format!(
                "color-{}",
                color.to_hex_string()
            )))
            .h_5()
            .w_5()
            .bg(color)
            .border_1()
            .border_color(color.darken(0.1))
            .when(clickable, |this| {
                this.cursor_pointer()
                    .hover(|this| {
                        this.border_color(color.darken(0.3))
                            .bg(color.lighten(0.1))
                            .shadow_sm()
                    })
                    .active(|this| this.border_color(color.darken(0.5)).bg(color.darken(0.2)))
                    .on_mouse_move(cx.listener(move |view, _, cx| {
                        view.hovered_color = Some(color);
                        cx.notify();
                    }))
                    .on_click(cx.listener(move |view, _, cx| {
                        view.update_value(Some(color), true, cx);
                        view.open = false;
                        cx.notify();
                    }))
            })
    }

    fn render_colors(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .gap_3()
            .child(
                h_flex().gap_1().children(
                    self.featured_colors
                        .iter()
                        .map(|color| self.render_item(*color, true, cx)),
                ),
            )
            .child(Divider::horizontal())
            .child(
                v_flex()
                    .gap_1()
                    .children(color_palettes().iter().map(|sub_colors| {
                        h_flex().gap_1().children(
                            sub_colors
                                .iter()
                                .rev()
                                .map(|color| self.render_item(*color, true, cx)),
                        )
                    })),
            )
            .when_some(self.hovered_color, |this, hovered_color| {
                this.child(Divider::horizontal()).child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(
                            div()
                                .bg(hovered_color)
                                .flex_shrink_0()
                                .border_1()
                                .border_color(hovered_color.darken(0.2))
                                .size_5()
                                .rounded(px(cx.theme().radius)),
                        )
                        .child(self.color_input.clone()),
                )
            })
    }

    fn resolved_corner(&self, bounds: Bounds<Pixels>) -> Point<Pixels> {
        match self.anchor {
            AnchorCorner::TopLeft => AnchorCorner::BottomLeft,
            AnchorCorner::TopRight => AnchorCorner::BottomRight,
            AnchorCorner::BottomLeft => AnchorCorner::TopLeft,
            AnchorCorner::BottomRight => AnchorCorner::TopRight,
        }
        .corner(bounds)
    }
}

impl Sizable for ColorPicker {
    fn with_size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }
}
impl EventEmitter<ColorPickerEvent> for ColorPicker {}
impl FocusableView for ColorPicker {
    fn focus_handle(&self, _: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ColorPicker {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let display_title: SharedString = if let Some(value) = self.value {
            value.to_hex_string()
        } else {
            "".to_string()
        }
        .into();

        let view = cx.view().clone();

        div()
            .id(self.id.clone())
            .key_context(KEY_CONTEXT)
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::on_escape))
            .child(
                h_flex()
                    .id("color-picker-input")
                    .cursor_pointer()
                    .gap_2()
                    .items_center()
                    .input_text_size(self.size)
                    .line_height(relative(1.))
                    .child(
                        div()
                            .id("color-picker-square")
                            .bg(cx.theme().background)
                            .border_1()
                            .border_color(cx.theme().input)
                            .rounded(px(cx.theme().radius))
                            .bg(cx.theme().background)
                            .shadow_sm()
                            .overflow_hidden()
                            .size_with(self.size)
                            .when_some(self.value, |this, value| {
                                this.bg(value).border_color(value.darken(0.3))
                            })
                            .tooltip(move |cx| Tooltip::new(display_title.clone(), cx)),
                    )
                    .when_some(self.label.clone(), |this, label| this.child(label))
                    .on_click(cx.listener(Self::toggle_picker))
                    .child(
                        canvas(
                            move |bounds, cx| view.update(cx, |r, _| r.bounds = bounds),
                            |_, _, _| {},
                        )
                        .absolute()
                        .size_full(),
                    ),
            )
            .when(self.open, |this| {
                this.child(
                    deferred(
                        anchored()
                            .anchor(self.anchor)
                            .snap_to_window_with_margin(px(8.))
                            .position(self.resolved_corner(self.bounds))
                            .child(
                                div()
                                    .track_focus(&self.focus_handle)
                                    .occlude()
                                    .map(|this| match self.anchor {
                                        AnchorCorner::TopLeft | AnchorCorner::TopRight => {
                                            this.mt_1p5()
                                        }
                                        AnchorCorner::BottomLeft | AnchorCorner::BottomRight => {
                                            this.mb_1p5()
                                        }
                                    })
                                    .w_72()
                                    .overflow_hidden()
                                    .rounded_lg()
                                    .p_3()
                                    .border_1()
                                    .border_color(cx.theme().border)
                                    .shadow_lg()
                                    .rounded_lg()
                                    .bg(cx.theme().background)
                                    .on_mouse_up_out(
                                        MouseButton::Left,
                                        cx.listener(|view, _, cx| view.on_escape(&Escape, cx)),
                                    )
                                    .child(self.render_colors(cx)),
                            ),
                    )
                    .with_priority(1),
                )
            })
    }
}
