pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// Draw the widget on standard output.
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{}", &buffer);
    }
}

pub struct Label {
    label: String,
}

impl Label {
    fn new(label: &str) -> Label {
        Label {
            label: label.to_owned(),
        }
    }
}

pub struct Button {
    label: Label,
    callback: Box<dyn FnMut()>,
}

impl Button {
    fn new(label: &str, callback: Box<dyn FnMut()>) -> Button {
        Button {
            label: Label::new(label),
            callback,
        }
    }
}

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }
}


impl Widget for Label {
    fn width(&self) -> usize {
        10 // hardcode width
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let x = buffer.write_str(&self.label);
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        self.label.width() + 10
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let w = self.width();
        let x = buffer.write_fmt(format_args!("+{:-<w$}+\n", ""));
        let x = buffer.write_fmt(format_args!("|{:^w$}|", &self.label.label));
        let x = buffer.write_fmt(format_args!("\n+{:-<w$}+", ""));
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        50 // hardcode width
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let w = self.width();
        let x = buffer.write_fmt(format_args!("+{:=<w$}+\n", ""));
        let x = buffer.write_fmt(format_args!("|{: ^w$}|", &self.title));
        let x = buffer.write_fmt(format_args!("\n+{:=<w$}+\n", ""));
        for widget in &self.widgets {
            let x = buffer.write_str("\n");
            widget.draw_into(buffer);
            let x = buffer.write_str("\n");
        }
        
    }
}

fn main() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new(
        "Click me!",
        Box::new(|| println!("You clicked the button!")),
    )));
    window.draw();
}