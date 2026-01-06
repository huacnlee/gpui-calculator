use gpui::*;
use gpui_component::{button::*, theme::*, *};

// Define quit action
actions!(calculator, [Quit]);

// Calculator operators
#[derive(Debug, Clone, Copy, PartialEq)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    fn to_symbol(&self) -> &str {
        match self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "×",
            Operator::Divide => "÷",
        }
    }
}

// Calculator state
struct Calculator {
    display: String,
    formula: String, // Display complete calculation formula
    current_value: f64,
    previous_value: Option<f64>,
    operator: Option<Operator>,
    should_reset: bool,
}

impl Calculator {
    fn new() -> Self {
        Self {
            display: "0".to_string(),
            formula: String::new(),
            current_value: 0.0,
            previous_value: None,
            operator: None,
            should_reset: false,
        }
    }

    fn input_digit(&mut self, digit: &str) {
        if self.should_reset {
            self.display = digit.to_string();
            self.should_reset = false;
            // Keep formula intact when starting new input after operator
        } else if self.display == "0" {
            self.display = digit.to_string();
        } else {
            self.display.push_str(digit);
        }
        self.current_value = self.display.parse().unwrap_or(0.0);
        self.update_formula();
    }

    fn input_decimal(&mut self) {
        if self.should_reset {
            self.display = "0.".to_string();
            self.should_reset = false;
        } else if !self.display.contains('.') {
            self.display.push('.');
        }
        self.update_formula();
    }

    fn set_operator(&mut self, op: Operator) {
        if let Some(prev) = self.previous_value {
            if let Some(current_op) = self.operator {
                self.calculate(current_op, prev);
            }
        }

        self.previous_value = Some(self.current_value);
        self.operator = Some(op);
        self.should_reset = true;
        self.update_formula();
    }

    fn calculate(&mut self, op: Operator, prev: f64) {
        let result = match op {
            Operator::Add => prev + self.current_value,
            Operator::Subtract => prev - self.current_value,
            Operator::Multiply => prev * self.current_value,
            Operator::Divide => {
                if self.current_value != 0.0 {
                    prev / self.current_value
                } else {
                    0.0 // Avoid division by zero
                }
            }
        };

        self.current_value = result;
        self.display = self.format_number(result);
    }

    fn format_number(&self, num: f64) -> String {
        if num.fract() == 0.0 && num.abs() < 1e10 {
            format!("{}", num as i64)
        } else {
            format!("{:.8}", num)
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string()
        }
    }

    fn update_formula(&mut self) {
        // Build complete formula showing all operations
        if let Some(prev) = self.previous_value {
            if let Some(op) = self.operator {
                // Show: previous_value operator current_input
                let current_input = if self.should_reset {
                    String::new()
                } else {
                    self.display.clone()
                };

                self.formula = format!(
                    "{} {} {}",
                    self.format_number(prev),
                    op.to_symbol(),
                    current_input
                )
                .trim()
                .to_string();
            }
        } else if !self.formula.contains('=') {
            // No operation in progress, just show current number
            if !self.display.is_empty() && self.display != "0" {
                self.formula = self.display.clone();
            }
        }
        // When formula contains '=', it stays unchanged (preserves calculation history)
    }

    fn equals(&mut self) {
        if let (Some(op), Some(prev)) = (self.operator, self.previous_value) {
            // Display complete formula
            let final_formula = format!(
                "{} {} {} =",
                self.format_number(prev),
                op.to_symbol(),
                &self.display
            );

            self.calculate(op, prev);
            self.formula = final_formula;
            self.operator = None;
            self.previous_value = None;
            self.should_reset = true;
        }
    }

    fn clear(&mut self) {
        self.display = "0".to_string();
        self.formula = String::new();
        self.current_value = 0.0;
        self.previous_value = None;
        self.operator = None;
        self.should_reset = false;
    }

    fn backspace(&mut self) {
        if !self.should_reset && self.display.len() > 1 {
            self.display.pop();
            self.current_value = self.display.parse().unwrap_or(0.0);
            self.update_formula();
        } else {
            self.clear();
        }
    }
}

struct CalculatorView {
    calculator: Calculator,
}

impl CalculatorView {
    fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            calculator: Calculator::new(),
        }
    }

    fn create_number_button(&mut self, number: &str, cx: &mut Context<Self>) -> Button {
        let num = number.to_string();
        Button::new(SharedString::from(format!("btn-{}", number)))
            .label(num.clone())
            .flex_1()
            .large()
            .on_click(cx.listener(move |view, _, _, cx| {
                view.calculator.input_digit(&num);
                cx.notify();
            }))
    }

    fn create_operator_button(
        &mut self,
        label: &str,
        op: Operator,
        cx: &mut Context<Self>,
    ) -> Button {
        Button::new(SharedString::from(format!("btn-op-{}", label)))
            .label(label.to_string())
            .primary()
            .large()
            .on_click(cx.listener(move |view, _, _, cx| {
                view.calculator.set_operator(op);
                cx.notify();
            }))
    }
}

impl Render for CalculatorView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .gap_4()
            .bg(cx.theme().background)
            .child(
                v_flex()
                    .size_full()
                    .gap_3()
                    .p_4()
                    .bg(cx.theme().secondary)
                    // Display
                    .child(
                        v_flex()
                            .w_full()
                            .min_h(px(100.))
                            .p_4()
                            .gap_2()
                            .bg(cx.theme().background)
                            .rounded_md()
                            .border_1()
                            .border_color(cx.theme().border)
                            // Formula display (small text)
                            .child(
                                div()
                                    .w_full()
                                    .h(px(24.))
                                    .flex()
                                    .items_center()
                                    .justify_end()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(cx.theme().muted_foreground)
                                            .child(if self.calculator.formula.is_empty() {
                                                " ".to_string()
                                            } else {
                                                self.calculator.formula.clone()
                                            }),
                                    ),
                            )
                            // Current value display (large text)
                            .child(
                                div().w_full().flex().items_center().justify_end().child(
                                    div()
                                        .text_2xl()
                                        .font_semibold()
                                        .text_color(cx.theme().foreground)
                                        .child(self.calculator.display.clone()),
                                ),
                            ),
                    )
                    // Button area
                    .child(
                        v_flex()
                            .gap_2()
                            .flex_1()
                            // First row: C, ⌫, ÷
                            .child(
                                h_flex()
                                    .gap_2()
                                    .flex_1()
                                    .child(
                                        Button::new("btn-clear")
                                            .label("C")
                                            .danger()
                                            .large()
                                            .flex_1()
                                            .on_click(cx.listener(|view, _, _, cx| {
                                                view.calculator.clear();
                                                cx.notify();
                                            })),
                                    )
                                    .child(
                                        Button::new("btn-backspace")
                                            .label("⌫")
                                            .warning()
                                            .large()
                                            .flex_1()
                                            .on_click(cx.listener(|view, _, _, cx| {
                                                view.calculator.backspace();
                                                cx.notify();
                                            })),
                                    )
                                    .child(
                                        self.create_operator_button("÷", Operator::Divide, cx)
                                            .flex_1(),
                                    ),
                            )
                            // Second row: 7, 8, 9, ×
                            .child(
                                h_flex()
                                    .gap_2()
                                    .flex_1()
                                    .child(self.create_number_button("7", cx))
                                    .child(self.create_number_button("8", cx))
                                    .child(self.create_number_button("9", cx))
                                    .child(
                                        self.create_operator_button("×", Operator::Multiply, cx)
                                            .flex_1(),
                                    ),
                            )
                            // Third row: 4, 5, 6, -
                            .child(
                                h_flex()
                                    .gap_2()
                                    .flex_1()
                                    .child(self.create_number_button("4", cx))
                                    .child(self.create_number_button("5", cx))
                                    .child(self.create_number_button("6", cx))
                                    .child(
                                        self.create_operator_button("-", Operator::Subtract, cx)
                                            .flex_1(),
                                    ),
                            )
                            // Fourth row: 1, 2, 3, +
                            .child(
                                h_flex()
                                    .gap_2()
                                    .flex_1()
                                    .child(self.create_number_button("1", cx))
                                    .child(self.create_number_button("2", cx))
                                    .child(self.create_number_button("3", cx))
                                    .child(
                                        self.create_operator_button("+", Operator::Add, cx)
                                            .flex_1(),
                                    ),
                            )
                            // Fifth row: 0, ., =
                            .child(
                                h_flex()
                                    .gap_2()
                                    .flex_1()
                                    .child(self.create_number_button("0", cx))
                                    .child(
                                        Button::new("btn-decimal")
                                            .label(".")
                                            .flex_1()
                                            .large()
                                            .on_click(cx.listener(|view, _, _, cx| {
                                                view.calculator.input_decimal();
                                                cx.notify();
                                            })),
                                    )
                                    .child(
                                        Button::new("btn-equals")
                                            .label("=")
                                            .success()
                                            .large()
                                            .flex_1()
                                            .on_click(cx.listener(|view, _, _, cx| {
                                                view.calculator.equals();
                                                cx.notify();
                                            })),
                                    ),
                            ),
                    ),
            )
    }
}

fn main() {
    let app = Application::new().with_assets(gpui_component_assets::Assets);

    app.run(move |cx| {
        // Initialize GPUI Component
        gpui_component::init(cx);

        Theme::global_mut(cx).shadow = false;

        cx.spawn(async move |cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::new(px(100.), px(100.)),
                        size: gpui::Size {
                            width: px(420.),
                            height: px(500.),
                        },
                    })),
                    titlebar: Some(TitlebarOptions {
                        title: Some("Calculator".into()),
                        appears_transparent: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|cx| CalculatorView::new(cx));
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
