//! Terminal User Interface

use std::{fmt, io::Write, marker::PhantomData, mem::ManuallyDrop};

use crate::ui::{self, Point};
use termios::Termios;

/// Unlike same-name constaint from `unistd.h(0p)` it may changed. Maybe...
// NOTE: how about make it editable?
// FIXME: `1` is a stdout fd, not stdin. Rename this shit...
const STDIN_FILENO: i32 = 1;

/// Init termios state. Returns original [`Termios`] that can be passed
/// to [`termios_restore`] later.
pub fn termios_init() -> Result<Termios, std::io::Error> {
    let mut termios = Termios::from_fd(STDIN_FILENO)?;
    let result = termios;

    use termios::*;

    termios.c_lflag &= !(ECHO | ECHOE | ECHOK | ECHONL);
    termios.c_lflag &= !ICANON;
    // FIXME: oppsie
    // termios.c_lflag &= ~(VSTOP | VSUSP | VQUIT);
    termios.c_cc[VMIN] = 1;
    termios.c_cc[VTIME] = 0;

    termios::tcsetattr(STDIN_FILENO, termios::TCSANOW, &termios)?;
    std::io::stdout().write_all(b"\x1b[H\x1b[2J\x1b[3J\x1b[?25l")?; // clear, hide cursor
    std::io::stdout().flush()?;

    Ok(result)
}

/// Restores termios state. See also [`termios_init`]
pub fn termios_restore(v: Termios) -> Result<(), std::io::Error> {
    std::io::stdout().write_all(b"\x1b[H\x1b[2J\x1b[3J\x1b[?25h")?; // clear, show cursor
    std::io::stdout().flush()?;
    termios::tcsetattr(STDIN_FILENO, termios::TCSANOW, &v)
}

/// Guard for terminal settings. On drop restores it.
///
/// # Example
/// ```no_run
/// let value = 42;
/// {
///     let _guard = TermIOsGuard::init(value).expect("termios");
///     println!("this text written with tui termios settings");
///     // *_guard drops*
/// }
/// println!("this is not");
/// ```
///
/// # Notes
/// On created this guard sets the main settings like "no echo" and
/// cursor style, but it doesn't clears the screen. Also, on drop screen will not
/// be restored.
pub struct TermiosGuard<T: ?Sized> {
    _guard: TermiosInnerGuard,
    pub v: T,
}
struct TermiosInnerGuard {
    saved_state: Termios,
}
impl<T> TermiosGuard<T> {
    /// Init termios state (by executing [`termios_init`]) and wraps `v` into guard.
    pub fn init(v: T) -> Result<Self, std::io::Error> {
        Ok(Self {
            _guard: TermiosInnerGuard {
                saved_state: termios_init()?,
            },
            v,
        })
    }

    /// "Leaks" guard without restoring state.
    pub fn leak(self) -> T {
        _ = ManuallyDrop::new(self._guard);
        self.v
    }
    /// Takes inner value and restore state.
    pub fn into_inner(self) -> T {
        self.v
    }

    /// Converts `TermiosGuard<T>` into `TermiosGuard<U>`
    pub fn map<F, U>(self, f: F) -> TermiosGuard<U>
    where
        F: FnOnce(T) -> U,
    {
        TermiosGuard {
            _guard: self._guard,
            v: f(self.v),
        }
    }
}
impl<T: ?Sized> std::ops::Deref for TermiosGuard<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.v
    }
}
impl<T: ?Sized> std::ops::DerefMut for TermiosGuard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}
impl std::ops::Drop for TermiosInnerGuard {
    fn drop(&mut self) {
        // calculated
        _ = termios_restore(self.saved_state);
    }
}

/// Main struct of user interface. Implements [`crate::ui::Context`] trait
#[derive(Default)]
pub struct Context(());

const STATUS_OFFSET: Point = Point(0, 0);
const MAIN_OFFSET: Point = Point(0, 3);
const LORE_OFFSET: Point = Point(crate::game::MAX_POINT.0 + 3, 3);

impl ui::Context for Context {
    type Error = std::io::Error;
    type Status<'a> = Fragment<'a, TextTy, { STATUS_OFFSET.0 }, { STATUS_OFFSET.1 }, 0, 2, false>;
    type Main<'a> = Fragment<
        'a,
        BlockTy,
        { MAIN_OFFSET.0 },
        { MAIN_OFFSET.1 },
        { crate::game::MAX_POINT.0 },
        { crate::game::MAX_POINT.1 },
        true,
    >;
    type Lore<'a> = Fragment<
        'a,
        TextTy,
        { LORE_OFFSET.0 },
        { LORE_OFFSET.1 },
        0,
        { crate::game::MAX_POINT.1 },
        false,
    >;

    fn status(&mut self) -> Self::Status<'_> {
        Fragment::init(TextTy { line: 0 })
    }
    fn main(&mut self) -> Self::Main<'_> {
        Fragment::init(BlockTy)
    }
    fn lore(&mut self) -> Self::Lore<'_> {
        Fragment::init(TextTy { line: 0 })
    }

    fn apply(&mut self) -> Result<(), Self::Error> {
        std::io::stdout().flush()
    }
}

impl Context {
    /// Creates new context. If you need to init interface see [`Context::init`]
    pub fn new() -> Self {
        Self(())
    }
    /// Init TUI. Clears screen and sets termios state
    pub fn init() -> Result<TermiosGuard<Self>, std::io::Error> {
        TermiosGuard::init(Self::new())
    }

    /// Draw borders between fragments
    pub fn draw_borders(&self) -> Result<(), std::io::Error> {
        let mut f = std::io::stdout();
        write!(f, "\x1b[3;1H")?;
        for _ in 0..crate::game::MAX_POINT.0 + 22 {
            f.write_all(b"-")?;
        }
        for y in 4..=crate::game::MAX_POINT.1 + MAIN_OFFSET.1 + 1 {
            write!(f, "\x1b[{};{}H|", y, crate::game::MAX_POINT.0 + 2)?;
        }
        Ok(())
    }
}

/// Fragment of interface. Implements [`crate::ui::Fragment`] etc...
// NOTE: 5 const generics params is... um... very bad idea. May make more types or etc?..
// TODO: remove `const LIMITED: bool` because `X == 0` is same thing
pub struct Fragment<
    'context,
    S: FragmentType,
    const X: u16,
    const Y: u16,
    const X_MAX: u16,
    const LINES: u16,
    const LIMITED: bool,
>(S, PhantomData<&'context mut ()>);

/// Type of [`Fragment`].
pub trait FragmentType {}

/// Type of [`Fragment`].
pub struct BlockTy;
impl FragmentType for BlockTy {}

/// Type of [`Fragment`], contains current (lore) line.
pub struct TextTy {
    pub line: u16,
}
impl FragmentType for TextTy {}

impl<
        'context,
        S: FragmentType,
        const X: u16,
        const Y: u16,
        const X_MAX: u16,
        const LINES: u16,
        const LIMITED: bool,
    > Fragment<'context, S, X, Y, X_MAX, LINES, LIMITED>
{
    /// Init fragment and set position to (X; Y).
    fn init(s: S) -> Self {
        // NOTE: it may panic
        print!("\x1b[{};{}H", Y + 1, X + 1);
        Self(s, PhantomData)
    }
}

impl<
        'context,
        S: FragmentType,
        const X: u16,
        const Y: u16,
        const X_MAX: u16,
        const LINES: u16,
        const LIMITED: bool,
    > ui::Fragment for Fragment<'context, S, X, Y, X_MAX, LINES, LIMITED>
{
    type Error = std::io::Error;

    fn set_pos(&mut self, pos: Point) -> Result<(), std::io::Error> {
        write!(
            std::io::stdout(),
            "\x1b[{};{}H",
            pos.1 + Y + 1,
            pos.0 + X + 1
        )
    }

    fn clear(&mut self) -> Result<(), Self::Error> {
        if LIMITED {
            for y in 0..LINES {
                write!(std::io::stdout(), "\x1b[{};{}H\x1b[1K", Y + y + 1, X_MAX)?;
            }
        } else {
            for y in 0..LINES {
                write!(std::io::stdout(), "\x1b[{};{}H\x1b[K", Y + y + 1, X)?;
            }
        }
        Ok(())
    }
}
impl<
        'context,
        const X: u16,
        const Y: u16,
        const X_MAX: u16,
        const LINES: u16,
        const LIMITED: bool,
    > ui::BlockFragment for Fragment<'context, BlockTy, X, Y, X_MAX, LINES, LIMITED>
{
    fn put_block(&mut self, block: ui::BlockTy) -> Result<(), std::io::Error> {
        use ui::BlockTy::*;
        let mut s = std::io::stdout();
        match block {
            Air => s.write_all(b" "),
            Player => s.write_all(b"\x1b[1;92m@\x1b[0m"),

            Wheat => s.write_all(b"\x1b[1;32m#\x1b[0m"),
            GrowingWheat => s.write_all(b"\x1b[33m+\x1b[0m"),

            Water => s.write_all(b"\x1b[1;34m%\x1b[0m"),
        }
    }
}
impl<
        'context,
        const X: u16,
        const Y: u16,
        const X_MAX: u16,
        const LINES: u16,
        const LIMITED: bool,
    > fmt::Write for Fragment<'context, TextTy, X, Y, X_MAX, LINES, LIMITED>
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut s = s.split('\n').peekable();
        let mut f = std::io::stdout();

        while let Some(fr) = s.next() {
            f.write_all(fr.as_bytes()).map_err(|_| fmt::Error)?;
            if s.peek().is_some() {
                use ui::Fragment;
                self.0.line += 1;
                self.set_line(self.0.line).map_err(|_| fmt::Error)?;
            }
        }

        Ok(())
    }
}
impl<
        'context,
        const X: u16,
        const Y: u16,
        const X_MAX: u16,
        const LINES: u16,
        const LIMITED: bool,
    > ui::TextFragment for Fragment<'context, TextTy, X, Y, X_MAX, LINES, LIMITED>
{
    fn set_color(&mut self, color: ui::Color) -> Result<(), fmt::Error> {
        use ui::Color::*;
        let mut s = std::io::stdout();
        let r = match color {
            Normal => s.write_all(b"\x1b[0m"),
            Red => s.write_all(b"\x1b[0;31m"),
            BoldRed => s.write_all(b"\x1b[1;31m"),
            Green => s.write_all(b"\x1b[0;32m"),
            Blue => s.write_all(b"\x1b[0;34m"),
            Magenta => s.write_all(b"\x1b[0;35m"),
            Cyan => s.write_all(b"\x1b[0;36m"),
            Gold => s.write_all(b"\x1b[0;93m"),
        };
        r.map_err(|_| fmt::Error)
    }
}
