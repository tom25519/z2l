//! The Z2L Terminal User Interface (TUI).

use bus::{Bus, BusReader};
use cursive::align::HAlign;
use cursive::direction::Direction;
use cursive::event::{AnyCb, Event, EventResult, Key};
use cursive::theme::{BaseColor, Color, Palette, PaletteColor, Theme};
use cursive::view::{CannotFocus, Margins, Nameable, Selector, ViewNotFound};
use cursive::views::{LinearLayout, NamedView, PaddedView, Panel, ScrollView, TextView};
use cursive::{Cursive, Printer, Rect, Vec2, View};
use z2l_core::{ControlMessage, InstructionLog};

/// 1 character margins on all sides.
const MARGINS_ALL: Margins = Margins {
    left: 1,
    right: 1,
    top: 1,
    bottom: 1,
};

/// 1 character margins on the left & right.
const MARGINS_HORIZONTAL: Margins = Margins {
    left: 1,
    right: 1,
    top: 0,
    bottom: 0,
};

/// The main Z2L [`View`].
pub struct Z2LView {
    control_bus: Bus<ControlMessage>,
    log_rx: BusReader<InstructionLog>,
    inner: PaddedView<LinearLayout>,
}

impl Z2LView {
    /// Create a new Z2LView.
    pub fn new(control_bus: Bus<ControlMessage>, log_rx: BusReader<InstructionLog>) -> Self {
        let inner = PaddedView::new(
            MARGINS_ALL,
            LinearLayout::vertical()
                .child(registers())
                .child(instructions())
                .child(help()),
        );

        Self {
            control_bus,
            log_rx,
            inner,
        }
    }

    /// Update the instruction list to show current instructions.
    fn update_instructions(&mut self, instr: &str) {
        self.call_on_any(&Selector::Name("instr-list"), &mut |list: &mut dyn View| {
            if let Some(list) = list.as_any_mut().downcast_mut::<NamedView<TextView>>() {
                let mut list = list.get_mut();
                let content = list.get_shared_content();
                let mut content_str = content.get_content().source().to_owned();
                content_str.insert_str(0, instr);
                content.set_content(content_str);
            }
        })
    }

    fn update_dyn_textview(view: &mut dyn View, value: &str) {
        if let Some(view) = view.as_any_mut().downcast_mut::<NamedView<TextView>>() {
            view.get_mut().set_content(value);
        }
    }

    /// Update the registers to show the current values.
    fn update_registers(&mut self, registers: &[i32]) {
        for (i, value) in registers.iter().enumerate() {
            self.call_on_any(
                &Selector::Name(&format!("reg{}", i)),
                &mut |reg: &mut dyn View| {
                    Self::update_dyn_textview(reg, &format!("{:08x}", value));
                },
            )
        }
    }

    /// Update the program counter to show the current value.
    fn update_pc(&mut self, value: u32) {
        self.call_on_any(&Selector::Name("pc"), &mut |pc: &mut dyn View| {
            Self::update_dyn_textview(pc, &format!("{:08x}", value));
        });
    }

    /// Update the TUI if any instructions have been executed.
    pub fn update(&mut self) {
        while let Ok(instruction) = self.log_rx.try_recv() {
            match instruction {
                InstructionLog::Ok {
                    instr,
                    registers,
                    pc,
                } => {
                    if let Some(instr) = instr {
                        self.update_instructions(&format!("{}\n", instr));
                    }

                    self.update_registers(&registers);
                    self.update_pc(pc);
                }
                InstructionLog::Exception {
                    exception,
                    registers,
                    pc,
                } => {
                    self.update_instructions(&format!("Encountered exception: {:?}\n", exception));
                    self.update_registers(&registers);
                    self.update_pc(pc);

                    self.control_bus.broadcast(ControlMessage::Halt);
                }
            }
        }
    }
}

impl View for Z2LView {
    fn draw(&self, printer: &Printer) {
        self.inner.draw(printer)
    }

    fn layout(&mut self, size: Vec2) {
        self.update();
        self.inner.layout(size)
    }

    fn needs_relayout(&self) -> bool {
        self.inner.needs_relayout()
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        self.inner.required_size(constraint)
    }

    fn on_event(&mut self, e: Event) -> EventResult {
        match e {
            Event::Char('q') => {
                self.control_bus.broadcast(ControlMessage::Halt);
                EventResult::with_cb(|siv| siv.quit())
            }
            Event::Char('r') => {
                self.control_bus.broadcast(ControlMessage::Reset);
                EventResult::consumed()
            }
            Event::Key(Key::Enter) => {
                self.control_bus.broadcast(ControlMessage::ManualTick);
                EventResult::consumed()
            }
            e => self.inner.on_event(e),
        }
    }

    fn call_on_any(&mut self, selector: &Selector, callback: AnyCb) {
        self.inner.call_on_any(selector, callback)
    }

    fn focus_view(&mut self, selector: &Selector) -> Result<EventResult, ViewNotFound> {
        self.inner.focus_view(selector)
    }

    fn take_focus(&mut self, source: Direction) -> Result<EventResult, CannotFocus> {
        self.inner.take_focus(source)
    }

    fn important_area(&self, view_size: Vec2) -> Rect {
        self.inner.important_area(view_size)
    }

    fn type_name(&self) -> &'static str {
        self.inner.type_name()
    }
}

/// Create a [`Cursive`] instance which implements the TUI.
pub fn create(control_bus: Bus<ControlMessage>, log_rx: BusReader<InstructionLog>) -> Cursive {
    let mut siv = Cursive::new();
    siv.add_layer(Z2LView::new(control_bus, log_rx));
    siv.set_theme(theme());
    siv
}

/// Simple Cursive theme for Z2L.
fn theme() -> Theme {
    let mut palette = Palette::default();
    palette[PaletteColor::Background] = Color::TerminalDefault;
    palette[PaletteColor::View] = Color::Light(BaseColor::White);
    palette[PaletteColor::Primary] = Color::Dark(BaseColor::Black);
    palette[PaletteColor::Secondary] = Color::Light(BaseColor::Black);
    palette[PaletteColor::TitlePrimary] = Color::Dark(BaseColor::Black);
    Theme {
        shadow: false,
        palette,
        ..Theme::default()
    }
}

/// The "Help" panel.
///
/// This shows some help text on using the TUI.
fn help() -> Panel<TextView> {
    Panel::new(TextView::new(
        "Press enter to advance the clock. Use the arrow keys to navigate. Press <q> to quit. Press <r> to reset.",
    ))
    .title("Help")
    .title_position(HAlign::Left)
}

/// The "Registers" panel.
///
/// This shows the current values of each register.
fn registers() -> Panel<NamedView<ScrollView<LinearLayout>>> {
    let mut layout = LinearLayout::horizontal();

    layout =
        layout.child(Panel::new(TextView::new(format!("{:08x}", 0)).with_name("pc")).title("PC"));

    for i in 0..32 {
        layout = layout.child(
            Panel::new(TextView::new(format!("{:08x}", 0)).with_name(format!("reg{}", i)))
                .title(format!("x{}", i)),
        );
    }

    let mut scroll = ScrollView::new(layout);
    scroll.set_scroll_x(true);
    scroll.set_scroll_y(false);

    Panel::new(scroll.with_name("reg-scroll"))
        .title("Registers")
        .title_position(HAlign::Left)
}

/// The "Instructions" panel.
///
/// This shows the history of executed instructions.
fn instructions() -> Panel<NamedView<ScrollView<PaddedView<NamedView<TextView>>>>> {
    Panel::new(
        ScrollView::new(PaddedView::new(
            MARGINS_HORIZONTAL,
            TextView::new("").with_name("instr-list"),
        ))
        .with_name("instr-scroll"),
    )
    .title("Instruction History")
    .title_position(HAlign::Left)
}
