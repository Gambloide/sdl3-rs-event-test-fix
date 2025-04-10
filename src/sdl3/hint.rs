use crate::sys;
use libc::c_char;
use std::ffi::{CStr, CString};

pub mod names;

pub enum Hint {
    Default,
    Normal,
    Override,
}

/// A hint that specifies whether a fullscreen [Window](../video/Window.t.html) will be
/// minimized if key focus is lost.
///
/// [Official SDL documentation](https://wiki.libsdl.org/SDL_HINT_VIDEO_MINIMIZE_ON_FOCUS_LOSS)
///
/// # Default
/// This is enabled by default.
///
/// # Example
/// ```rust,no_run
/// sdl3::hint::set_video_minimize_on_focus_loss(false);
/// ```
///
/// * `value`: `true` to enable minimizing of the Window if it loses key focus when in fullscreen mode,
///            `false` to disable this feature.
pub fn set_video_minimize_on_focus_loss(value: bool) -> bool {
    set(
        names::VIDEO_MINIMIZE_ON_FOCUS_LOSS,
        if value { "1" } else { "0" },
    )
}

/// A hint that specifies whether a fullscreen [Window](../video/Window.t.html) will be
/// minimized if key focus is lost.
///
/// [Official SDL documentation](https://wiki.libsdl.org/SDL_HINT_VIDEO_MINIMIZE_ON_FOCUS_LOSS)
///
/// # Example
/// ```rust,no_run
/// sdl3::hint::set_video_minimize_on_focus_loss_with_priority(false, &sdl3::hint::Hint::Override);
/// ```
///
/// * `value`: `true` to enable minimizing of the Window if it loses key focus when in fullscreen mode,
///            `false` to disable this feature.
/// * `priority`: The priority controls the behavior when setting a hint that already has a value.
///               Hints will replace existing hints of their priority and lower.
///               Environment variables are considered to have override priority.
pub fn set_video_minimize_on_focus_loss_with_priority(value: bool, priority: &Hint) -> bool {
    set_with_priority(
        names::VIDEO_MINIMIZE_ON_FOCUS_LOSS,
        if value { "1" } else { "0" },
        priority,
    )
}

/// A hint that specifies whether a fullscreen [Window](../video/Window.t.html) will be
/// minimized if key focus is lost.
///
/// [Official SDL documentation](https://wiki.libsdl.org/SDL_HINT_VIDEO_MINIMIZE_ON_FOCUS_LOSS)
///
/// # Default
/// By default this will return `true`.
///
/// # Example
/// ```rust,no_run
/// assert_eq!(sdl3::hint::get_video_minimize_on_focus_loss(), true);
///
/// sdl3::hint::set_video_minimize_on_focus_loss(false);
/// assert_eq!(sdl3::hint::get_video_minimize_on_focus_loss(), false);
/// ```
pub fn get_video_minimize_on_focus_loss() -> bool {
    let Some(value) = get(names::VIDEO_MINIMIZE_ON_FOCUS_LOSS) else {
        return true;
    };
    &*value == "1"
}

#[doc(alias = "SDL_SetHint")]
pub fn set(name: &str, value: &str) -> bool {
    let name = CString::new(name).unwrap();
    let value = CString::new(value).unwrap();
    unsafe {
        sys::hints::SDL_SetHint(
            name.as_ptr() as *const c_char,
            value.as_ptr() as *const c_char,
        )
    }
}

#[doc(alias = "SDL_GetHint")]
pub fn get(name: &str) -> Option<String> {
    use std::str;

    let name = CString::new(name).unwrap();

    unsafe {
        let res = sys::hints::SDL_GetHint(name.as_ptr() as *const c_char);

        if res.is_null() {
            None
        } else {
            Some(
                str::from_utf8(CStr::from_ptr(res as *const _).to_bytes())
                    .unwrap()
                    .to_owned(),
            )
        }
    }
}

#[doc(alias = "SDL_SetHintWithPriority")]
pub fn set_with_priority(name: &str, value: &str, priority: &Hint) -> bool {
    let name = CString::new(name).unwrap();
    let value = CString::new(value).unwrap();

    let priority_val = match *priority {
        Hint::Normal => sys::hints::SDL_HINT_NORMAL,
        Hint::Override => sys::hints::SDL_HINT_OVERRIDE,
        Hint::Default => sys::hints::SDL_HINT_DEFAULT,
    };

    unsafe {
        sys::hints::SDL_SetHintWithPriority(
            name.as_ptr() as *const c_char,
            value.as_ptr() as *const c_char,
            priority_val,
        )
    }
}
