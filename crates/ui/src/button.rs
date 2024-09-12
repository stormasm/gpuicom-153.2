use crate::{
    h_flex,
    indicator::Indicator,
    theme::{ActiveTheme, Colorize as _},
    tooltip::Tooltip,
    Disableable, Icon, Selectable, Sizable, Size,
};
use gpui::{
    div, prelude::FluentBuilder as _, px, relative, AnyElement, ClickEvent, Corners, Div, Edges,
    ElementId, FocusHandle, Hsla, InteractiveElement, IntoElement, MouseButton, ParentElement,
    Pixels, RenderOnce, SharedString, StatefulInteractiveElement as _, Styled, WindowContext,
};

pub enum ButtonRounded {
    None,
    Small,
    Medium,
    Large,
    Size(Pixels),
}

impl From<Pixels> for ButtonRounded {
    fn from(px: Pixels) -> Self {
        ButtonRounded::Size(px)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ButtonCustomStyle {
    color: Hsla,
    foreground: Hsla,
    border: Hsla,
    hover: Hsla,
    active: Hsla,
}

impl ButtonCustomStyle {
    pub fn new(cx: &WindowContext) -> Self {
        Self {
            color: cx.theme().secondary,
            foreground: cx.theme().secondary_foreground,
            border: cx.theme().border,
            hover: cx.theme().secondary_hover,
            active: cx.theme().secondary_active,
        }
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = color;
        self
    }

    pub fn foreground(mut self, color: Hsla) -> Self {
        self.foreground = color;
        self
    }

    pub fn border(mut self, color: Hsla) -> Self {
        self.border = color;
        self
    }

    pub fn hover(mut self, color: Hsla) -> Self {
        self.hover = color;
        self
    }

    pub fn active(mut self, color: Hsla) -> Self {
        self.active = color;
        self
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Danger,
    Outline,
    Ghost,
    Link,
    Text,
    Custom(ButtonCustomStyle),
}

impl ButtonStyle {
    fn is_link(&self) -> bool {
        matches!(self, Self::Link)
    }

    fn is_text(&self) -> bool {
        matches!(self, Self::Text)
    }

    fn no_padding(&self) -> bool {
        self.is_link() || self.is_text()
    }
}

#[derive(IntoElement)]
pub struct Button {
    pub base: Div,
    id: ElementId,
    focus_handle: FocusHandle,
    icon: Option<Icon>,
    label: Option<SharedString>,
    children: Vec<AnyElement>,
    disabled: bool,
    selected: bool,
    style: ButtonStyle,
    rounded: ButtonRounded,
    border_corners: Corners<bool>,
    border_edges: Edges<bool>,
    size: Size,
    compact: bool,
    tooltip: Option<SharedString>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
    loading: bool,
}

impl From<Button> for AnyElement {
    fn from(button: Button) -> Self {
        button.into_any_element()
    }
}

impl Button {
    pub fn new(id: impl Into<ElementId>, cx: &mut WindowContext) -> Self {
        Self {
            base: div(),
            focus_handle: cx.focus_handle(),
            id: id.into(),
            icon: None,
            label: None,
            disabled: false,
            selected: false,
            style: ButtonStyle::Secondary,
            rounded: ButtonRounded::Medium,
            border_corners: Corners::all(true),
            border_edges: Edges::all(true),
            size: Size::Medium,
            tooltip: None,
            on_click: None,
            loading: false,
            compact: false,
            children: Vec::new(),
        }
    }

    /// With the primary style for the Button.
    pub fn primary(mut self) -> Self {
        self.style = ButtonStyle::Primary;
        self
    }

    /// With the secondary style for the Button.
    pub fn danger(mut self) -> Self {
        self.style = ButtonStyle::Danger;
        self
    }

    /// With the ghost style for the Button.
    pub fn ghost(mut self) -> Self {
        self.style = ButtonStyle::Ghost;
        self
    }

    /// With the outline style for the Button.
    pub fn outline(mut self) -> Self {
        self.style = ButtonStyle::Outline;
        self
    }

    /// With the link style for the Button.
    pub fn link(mut self) -> Self {
        self.style = ButtonStyle::Link;
        self
    }

    /// With the text style for the Button, it will no padding look like a normal text.
    pub fn text(mut self) -> Self {
        self.style = ButtonStyle::Text;
        self
    }

    /// With the custom style for the Button.
    pub fn custom(mut self, custom: ButtonCustomStyle) -> Self {
        self.style = ButtonStyle::Custom(custom);
        self
    }

    /// Set the border radius of the Button.
    pub fn rounded(mut self, rounded: impl Into<ButtonRounded>) -> Self {
        self.rounded = rounded.into();
        self
    }

    /// Set the border corners side of the Button.
    pub(crate) fn border_corners(mut self, corners: impl Into<Corners<bool>>) -> Self {
        self.border_corners = corners.into();
        self
    }

    /// Set the border edges of the Button.
    pub(crate) fn border_edges(mut self, edges: impl Into<Edges<bool>>) -> Self {
        self.border_edges = edges.into();
        self
    }

    /// Set label to the Button, if no label is set, the button will be in Icon Button mode.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the icon of the button, if the Button have no label, the button well in Icon Button mode.
    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set the tooltip of the button.
    pub fn tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Set the ButtonStyle
    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    /// Set true to show the loading indicator.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Set the button to compact mode, then padding will be reduced.
    pub fn compact(mut self) -> Self {
        self.compact = true;
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl Disableable for Button {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Selectable for Button {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl Sizable for Button {
    fn with_size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl ParentElement for Button {
    fn extend(&mut self, elements: impl IntoIterator<Item = gpui::AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for Button {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let style: ButtonStyle = self.style;
        let normal_style = style.normal(cx);
        let focused = self.focus_handle.is_focused(cx);
        let icon_size = match self.size {
            Size::Size(v) => Size::Size(v * 0.75),
            _ => self.size,
        };

        self.base
            .id(self.id)
            .track_focus(&self.focus_handle)
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .overflow_hidden()
            .when(!style.no_padding(), |this| {
                if self.label.is_none() && self.children.is_empty() {
                    // Icon Button
                    match self.size {
                        Size::Size(px) => this.size(px),
                        Size::XSmall => this.size_5(),
                        Size::Small => this.size_6(),
                        Size::Large | Size::Medium => this.size_8(),
                    }
                } else {
                    // Normal Button
                    match self.size {
                        Size::Size(size) => this.px(size * 0.2),
                        Size::XSmall => this.h_5().px_1(),
                        Size::Small => this.h_6().px_3().when(self.compact, |this| this.px_2()),
                        _ => this.h_8().px_4().when(self.compact, |this| this.px_2()),
                    }
                }
            })
            .when(
                self.border_corners.top_left && self.border_corners.bottom_left,
                |this| match self.rounded {
                    ButtonRounded::Small => this.rounded_l(px(cx.theme().radius * 0.5)),
                    ButtonRounded::Medium => this.rounded_l(px(cx.theme().radius)),
                    ButtonRounded::Large => this.rounded_l(px(cx.theme().radius * 2.0)),
                    ButtonRounded::Size(px) => this.rounded_l(px),
                    ButtonRounded::None => this.rounded_none(),
                },
            )
            .when(
                self.border_corners.top_right && self.border_corners.bottom_right,
                |this| match self.rounded {
                    ButtonRounded::Small => this.rounded_r(px(cx.theme().radius * 0.5)),
                    ButtonRounded::Medium => this.rounded_r(px(cx.theme().radius)),
                    ButtonRounded::Large => this.rounded_r(px(cx.theme().radius * 2.0)),
                    ButtonRounded::Size(px) => this.rounded_r(px),
                    ButtonRounded::None => this.rounded_none(),
                },
            )
            .when(self.border_edges.left, |this| this.border_l_1())
            .when(self.border_edges.right, |this| this.border_r_1())
            .when(self.border_edges.top, |this| this.border_t_1())
            .when(self.border_edges.bottom, |this| this.border_b_1())
            .text_color(normal_style.fg)
            .when(self.selected, |this| {
                let selected_style = style.selected(cx);
                this.bg(selected_style.bg)
                    .border_color(selected_style.border)
                    .text_color(selected_style.fg)
            })
            .when(!self.disabled && !self.selected, |this| {
                this.border_color(normal_style.border)
                    .bg(normal_style.bg)
                    .when(normal_style.underline, |this| this.text_decoration_1())
                    .hover(|this| {
                        let hover_style = style.hovered(cx);
                        this.bg(hover_style.bg)
                            .border_color(hover_style.border)
                            .text_color(crate::red_400())
                    })
                    .active(|this| {
                        let active_style = style.active(cx);
                        this.bg(active_style.bg)
                            .border_color(active_style.border)
                            .text_color(active_style.fg)
                    })
            })
            .when(focused, |this| this.border_color(cx.theme().ring))
            .when_some(
                self.on_click.filter(|_| !self.disabled && !self.loading),
                |this, on_click| {
                    this.on_mouse_down(MouseButton::Left, |_, cx| {
                        cx.prevent_default();
                        cx.stop_propagation()
                    })
                    .on_click(move |event, cx| {
                        (on_click)(event, cx);
                    })
                },
            )
            .when(self.disabled, |this| {
                let disabled_style = style.disabled(cx);
                this.cursor_not_allowed()
                    .bg(disabled_style.bg)
                    .text_color(disabled_style.fg)
                    .border_color(disabled_style.border)
            })
            .child({
                h_flex()
                    .id("label")
                    .items_center()
                    .justify_center()
                    .h_full()
                    .gap_2()
                    .map(|this| match self.size {
                        Size::XSmall => this.text_xs(),
                        Size::Small => this.text_sm(),
                        _ => this.text_base(),
                    })
                    .when(!self.loading, |this| {
                        this.when_some(self.icon, |this, icon| {
                            this.child(icon.with_size(icon_size))
                        })
                    })
                    .when(self.loading, |this| {
                        this.child(Indicator::new().with_size(self.size))
                    })
                    .when_some(self.label, |this, label| {
                        this.child(
                            div()
                                // FIXME: Move down 3px to vertically center the text
                                .mt(px(2.))
                                .flex_none()
                                .line_height(relative(1.))
                                .child(label),
                        )
                    })
                    .children(self.children)
            })
            .when(self.loading, |this| this.bg(normal_style.bg.opacity(0.8)))
            .when_some(self.tooltip.clone(), |this, tooltip| {
                this.tooltip(move |cx| Tooltip::new(tooltip.clone(), cx))
            })
    }
}

struct ButtonStyles {
    bg: Hsla,
    border: Hsla,
    fg: Hsla,
    underline: bool,
}

impl ButtonStyle {
    fn bg_color(&self, cx: &WindowContext) -> Hsla {
        match self {
            ButtonStyle::Primary => cx.theme().primary,
            ButtonStyle::Secondary => cx.theme().secondary,
            ButtonStyle::Danger => cx.theme().destructive,
            ButtonStyle::Outline | ButtonStyle::Ghost | ButtonStyle::Link | ButtonStyle::Text => {
                cx.theme().transparent
            }
            ButtonStyle::Custom(colors) => colors.color,
        }
    }

    fn text_color(&self, cx: &WindowContext) -> Hsla {
        match self {
            ButtonStyle::Primary => cx.theme().primary_foreground,
            ButtonStyle::Secondary | ButtonStyle::Outline | ButtonStyle::Ghost => {
                cx.theme().secondary_foreground
            }
            ButtonStyle::Danger => cx.theme().destructive_foreground,
            ButtonStyle::Link => cx.theme().link,
            ButtonStyle::Text => cx.theme().foreground,
            ButtonStyle::Custom(colors) => colors.foreground,
        }
    }

    fn border_color(&self, cx: &WindowContext) -> Hsla {
        match self {
            ButtonStyle::Primary => cx.theme().primary,
            ButtonStyle::Secondary => cx.theme().border,
            ButtonStyle::Danger => cx.theme().destructive,
            ButtonStyle::Outline => cx.theme().border,
            ButtonStyle::Ghost | ButtonStyle::Link | ButtonStyle::Text => cx.theme().transparent,
            ButtonStyle::Custom(colors) => colors.border,
        }
    }

    fn underline(&self, _: &WindowContext) -> bool {
        match self {
            ButtonStyle::Link => true,
            _ => false,
        }
    }

    fn normal(&self, cx: &WindowContext) -> ButtonStyles {
        let bg = self.bg_color(cx);
        let border = self.border_color(cx);
        let fg = self.text_color(cx);
        let underline = self.underline(cx);

        ButtonStyles {
            bg,
            border,
            fg,
            underline,
        }
    }

    fn hovered(&self, cx: &WindowContext) -> ButtonStyles {
        let bg = match self {
            ButtonStyle::Primary => cx.theme().primary_hover,
            ButtonStyle::Secondary | ButtonStyle::Outline => cx.theme().secondary_hover,
            ButtonStyle::Danger => cx.theme().destructive_hover,
            ButtonStyle::Ghost => cx.theme().secondary_hover,
            ButtonStyle::Link => cx.theme().transparent,
            ButtonStyle::Text => cx.theme().transparent,
            ButtonStyle::Custom(colors) => colors.hover,
        };
        let border = self.border_color(cx);
        let fg = match self {
            ButtonStyle::Link => cx.theme().link_hover,
            _ => self.text_color(cx),
        };
        let underline = self.underline(cx);

        ButtonStyles {
            bg,
            border,
            fg,
            underline,
        }
    }

    fn active(&self, cx: &WindowContext) -> ButtonStyles {
        let bg = match self {
            ButtonStyle::Primary => cx.theme().primary_active,
            ButtonStyle::Secondary | ButtonStyle::Outline | ButtonStyle::Ghost => {
                cx.theme().secondary_active
            }
            ButtonStyle::Danger => cx.theme().destructive_active,
            ButtonStyle::Link => cx.theme().transparent,
            ButtonStyle::Text => cx.theme().transparent,
            ButtonStyle::Custom(colors) => colors.active,
        };
        let border = self.border_color(cx);
        let fg = match self {
            ButtonStyle::Link => cx.theme().link_active,
            ButtonStyle::Text => cx.theme().foreground.opacity(0.7),
            _ => self.text_color(cx),
        };
        let underline = self.underline(cx);

        ButtonStyles {
            bg,
            border,
            fg,
            underline,
        }
    }

    fn selected(&self, cx: &WindowContext) -> ButtonStyles {
        let bg = match self {
            ButtonStyle::Primary => cx.theme().primary_active,
            ButtonStyle::Secondary | ButtonStyle::Outline | ButtonStyle::Ghost => {
                cx.theme().secondary_active
            }
            ButtonStyle::Danger => cx.theme().destructive_active,
            ButtonStyle::Link => cx.theme().transparent,
            ButtonStyle::Text => cx.theme().transparent,
            ButtonStyle::Custom(colors) => colors.active,
        };
        let border = self.border_color(cx);
        let fg = match self {
            ButtonStyle::Link => cx.theme().link_active,
            ButtonStyle::Text => cx.theme().foreground.opacity(0.7),
            _ => self.text_color(cx),
        };
        let underline = self.underline(cx);

        ButtonStyles {
            bg,
            border,
            fg,
            underline,
        }
    }

    fn disabled(&self, cx: &WindowContext) -> ButtonStyles {
        let bg = match self {
            ButtonStyle::Link | ButtonStyle::Ghost | ButtonStyle::Text => cx.theme().transparent,
            _ => cx.theme().secondary.darken(0.2).grayscale(),
        };
        let fg = match self {
            ButtonStyle::Link | ButtonStyle::Text | ButtonStyle::Ghost => {
                cx.theme().link.grayscale()
            }
            _ => cx.theme().secondary_foreground.darken(0.2).grayscale(),
        };

        let border = bg;
        let underline = self.underline(cx);

        ButtonStyles {
            bg,
            border,
            fg,
            underline,
        }
    }
}
