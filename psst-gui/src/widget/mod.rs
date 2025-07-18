mod checkbox;
mod dispatcher;
mod empty;
pub mod fill_between;
pub mod icons;
mod link;
mod maybe;
mod overlay;
mod promise;
pub mod remote_image;
mod theme;
mod utils;

use std::{sync::Arc, time::Duration};

use druid::{
    widget::{ControllerHost, Padding},
    Data, Env, EventCtx, Insets, Menu, MouseButton, MouseEvent, Selector, UpdateCtx, Widget,
};

pub use checkbox::Checkbox;
pub use dispatcher::ViewDispatcher;
use druid_shell::Cursor;
pub use empty::Empty;
pub use link::Link;
pub use maybe::Maybe;
pub use overlay::Overlay;
pub use promise::Async;
pub use remote_image::RemoteImage;
pub use theme::ThemeScope;
pub use utils::{Border, Clip, FadeOut, Logger};

use crate::{
    controller::{ExClick, ExCursor, ExScroll, OnCommand, OnCommandAsync, OnDebounce, OnUpdate},
    data::{AppState, SliderScrollScale},
};

pub trait MyWidgetExt<T: Data>: Widget<T> + Sized + 'static {
    #[allow(dead_code)]
    fn log(self, label: &'static str) -> Logger<Self> {
        Logger::new(self).with_label(label)
    }

    fn link(self) -> Link<T> {
        Link::new(self)
    }

    fn clip<S>(self, shape: S) -> Clip<S, Self> {
        Clip::new(shape, self)
    }

    fn padding_left(self, p: f64) -> Padding<T, Self> {
        Padding::new(Insets::new(p, 0.0, 0.0, 0.0), self)
    }

    fn padding_right(self, p: f64) -> Padding<T, Self> {
        Padding::new(Insets::new(0.0, 0.0, p, 0.0), self)
    }

    fn padding_horizontal(self, p: f64) -> Padding<T, Self> {
        Padding::new(Insets::new(p, 0.0, p, 0.0), self)
    }

    fn on_debounce(
        self,
        duration: Duration,
        handler: impl Fn(&mut EventCtx, &mut T, &Env) + 'static,
    ) -> ControllerHost<Self, OnDebounce<T>> {
        ControllerHost::new(self, OnDebounce::trailing(duration, handler))
    }

    fn on_update<F>(self, handler: F) -> ControllerHost<Self, OnUpdate<F>>
    where
        F: Fn(&mut UpdateCtx, &T, &T, &Env) + 'static,
    {
        ControllerHost::new(self, OnUpdate::new(handler))
    }

    fn on_left_click(
        self,
        func: impl Fn(&mut EventCtx, &MouseEvent, &mut T, &Env) + 'static,
    ) -> ControllerHost<ControllerHost<Self, ExCursor<T>>, ExClick<T>> {
        self.with_cursor(Cursor::Pointer)
            .on_mouse_click(MouseButton::Left, func)
    }

    fn on_right_click(
        self,
        func: impl Fn(&mut EventCtx, &MouseEvent, &mut T, &Env) + 'static,
    ) -> ControllerHost<Self, ExClick<T>> {
        self.on_mouse_click(MouseButton::Right, func)
    }

    fn on_mouse_click(
        self,
        button: MouseButton,
        func: impl Fn(&mut EventCtx, &MouseEvent, &mut T, &Env) + 'static,
    ) -> ControllerHost<Self, ExClick<T>> {
        ControllerHost::new(self, ExClick::new(Some(button), func))
    }

    fn on_scroll(
        self,
        scale_picker: impl Fn(&mut T) -> &SliderScrollScale + 'static,
        action: impl Fn(&mut EventCtx, &mut T, &Env, f64) + 'static,
    ) -> ControllerHost<Self, ExScroll<T>> {
        ControllerHost::new(self, ExScroll::new(scale_picker, action))
    }

    fn with_cursor(self, cursor: Cursor) -> ControllerHost<Self, ExCursor<T>> {
        ControllerHost::new(self, ExCursor::new(cursor))
    }

    fn on_command<U, F>(
        self,
        selector: Selector<U>,
        func: F,
    ) -> ControllerHost<Self, OnCommand<U, F>>
    where
        U: 'static,
        F: Fn(&mut EventCtx, &U, &mut T),
    {
        ControllerHost::new(self, OnCommand::new(selector, func))
    }

    fn on_command_async<U: Data + Send, V: Data + Send>(
        self,
        selector: Selector<U>,
        request: impl Fn(U) -> V + Sync + Send + 'static,
        preflight: impl Fn(&mut EventCtx, &mut T, U) + 'static,
        response: impl Fn(&mut EventCtx, &mut T, (U, V)) + 'static,
    ) -> OnCommandAsync<Self, T, U, V> {
        OnCommandAsync::new(
            self,
            selector,
            Box::new(preflight),
            Arc::new(request),
            Box::new(response),
        )
    }

    fn context_menu(
        self,
        func: impl Fn(&T) -> Menu<AppState> + 'static,
    ) -> ControllerHost<Self, ExClick<T>> {
        self.on_right_click(move |ctx, event, data, _env| {
            ctx.show_context_menu(func(data), event.window_pos);
        })
    }
}

impl<T: Data, W: Widget<T> + 'static> MyWidgetExt<T> for W {}
