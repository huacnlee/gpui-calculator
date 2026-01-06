use gpui::*;
use gpui_component::{button::*, theme::*, *};
use std::path::PathBuf;

// 计算器的运算符
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

// 计算器状态
struct Calculator {
    display: String,
    formula: String, // 新增：显示完整计算公式
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
                    0.0 // 避免除以零
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
        let mut formula = String::new();

        // 始终显示完整的计算过程
        if let Some(prev) = self.previous_value {
            formula.push_str(&self.format_number(prev));

            if let Some(op) = self.operator {
                formula.push_str(" ");
                formula.push_str(op.to_symbol());
                formula.push_str(" ");

                // 如果不是刚按完运算符,显示当前输入的数字
                if !self.should_reset {
                    formula.push_str(&self.display);
                }
            }
        } else if !self.formula.is_empty() && self.formula.ends_with('=') {
            // 如果是计算结果后的新输入,保留之前的公式
            // 不做任何改变,保持原有公式
        } else {
            // 初始状态或只有当前数字
            if !self.display.is_empty() && self.display != "0" {
                formula = self.display.clone();
            }
        }

        // 只有在有实际变化时才更新公式
        if !formula.is_empty() || !self.formula.ends_with('=') {
            self.formula = formula;
        }
    }

    fn equals(&mut self) {
        if let (Some(op), Some(prev)) = (self.operator, self.previous_value) {
            // 显示完整公式
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
                    .w_full()
                    .max_w(px(420.))
                    .gap_3()
                    .p_4()
                    .bg(cx.theme().secondary)
                    .rounded_lg()
                    .shadow_lg()
                    // 显示屏
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
                            // 公式显示（小字）
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
                            // 当前数值显示（大字）
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
                    // 按钮区域
                    .child(
                        v_flex()
                            .gap_2()
                            .flex_1()
                            // 第一行: C, ⌫, ÷
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
                            // 第二行: 7, 8, 9, ×
                            .child(
                                h_flex()
                                    .gap_2()
                                    .flex_1()
                                    .child(self.create_number_button("7", cx).flex_1())
                                    .child(self.create_number_button("8", cx).flex_1())
                                    .child(self.create_number_button("9", cx).flex_1())
                                    .child(
                                        self.create_operator_button("×", Operator::Multiply, cx)
                                            .flex_1(),
                                    ),
                            )
                            // 第三行: 4, 5, 6, -
                            .child(
                                h_flex()
                                    .gap_2()
                                    .flex_1()
                                    .child(self.create_number_button("4", cx).flex_1())
                                    .child(self.create_number_button("5", cx).flex_1())
                                    .child(self.create_number_button("6", cx).flex_1())
                                    .child(
                                        self.create_operator_button("-", Operator::Subtract, cx)
                                            .flex_1(),
                                    ),
                            )
                            // 第四行: 1, 2, 3, +
                            .child(
                                h_flex()
                                    .gap_2()
                                    .flex_1()
                                    .child(self.create_number_button("1", cx).flex_1())
                                    .child(self.create_number_button("2", cx).flex_1())
                                    .child(self.create_number_button("3", cx).flex_1())
                                    .child(
                                        self.create_operator_button("+", Operator::Add, cx)
                                            .flex_1(),
                                    ),
                            )
                            // 第五行: 0, ., =
                            .child(
                                h_flex()
                                    .gap_2()
                                    .flex_1()
                                    .child(self.create_number_button("0", cx).flex_1())
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
                                        div().flex_1().child(
                                            Button::new("btn-equals")
                                                .label("=")
                                                .success()
                                                .large()
                                                .w_full()
                                                .on_click(cx.listener(|view, _, _, cx| {
                                                    view.calculator.equals();
                                                    cx.notify();
                                                })),
                                        ),
                                    ),
                            ),
                    ),
            )
    }
}

// 初始化主题
fn init_theme(cx: &mut App) {
    // 根据系统主题选择 Light 或 Dark
    let theme_name = SharedString::from("Hybrid Dark"); // 默认使用 Dark 主题

    // 加载并监听主题目录
    if let Err(err) = ThemeRegistry::watch_dir(PathBuf::from("./themes"), cx, move |cx| {
        if let Some(theme) = ThemeRegistry::global(cx).themes().get(&theme_name).cloned() {
            Theme::global_mut(cx).apply_config(&theme);
        }
    }) {
        eprintln!("加载主题失败: {}", err);
    }
}

fn main() {
    let app = Application::new().with_assets(gpui_component_assets::Assets);

    app.run(move |cx| {
        // 初始化 GPUI Component
        gpui_component::init(cx);

        // 初始化主题
        init_theme(cx);

        cx.spawn(async move |cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::new(px(100.), px(100.)),
                        size: gpui::Size {
                            width: px(450.),
                            height: px(650.),
                        },
                    })),
                    titlebar: Some(TitlebarOptions {
                        title: Some("计算器".into()),
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
