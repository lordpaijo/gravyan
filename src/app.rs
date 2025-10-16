pub enum InputField {
    Height,
    Velocity,
}

pub enum AppState {
    Input,
    Result,
}

pub struct App {
    pub height_input: String,
    pub velocity_input: String,
    pub selected_field: InputField,
    pub state: AppState,
    pub result: Option<CalculationResult>,
}

pub struct CalculationResult {
    pub velocity: f64,
    pub height: f64,
    pub gravity: f64,
}

impl App {
    pub fn new() -> App {
        App {
            height_input: String::new(),
            velocity_input: String::new(),
            selected_field: InputField::Height,
            state: AppState::Input,
            result: None,
        }
    }

    pub fn switch_field(&mut self) {
        self.selected_field = match self.selected_field {
            InputField::Height => InputField::Velocity,
            InputField::Velocity => InputField::Height,
        };
    }

    pub fn handle_input(&mut self, c: char) {
        let input = match self.selected_field {
            InputField::Height => &mut self.height_input,
            InputField::Velocity => &mut self.velocity_input,
        };

        if c.is_numeric() || c == '.' || c == '-' {
            input.push(c);
        }
    }

    pub fn handle_backspace(&mut self) {
        let input = match self.selected_field {
            InputField::Height => &mut self.height_input,
            InputField::Velocity => &mut self.velocity_input,
        };
        input.pop();
    }

    pub fn calculate(&mut self) {
        if let (Ok(height), Ok(velocity)) = (
            self.height_input.parse::<f64>(),
            self.velocity_input.parse::<f64>(),
        ) {
            if height > 0.0 {
                let gravity = velocity.powf(2.0) / (height * 2.0);
                self.result = Some(CalculationResult {
                    velocity,
                    height,
                    gravity,
                });
                self.state = AppState::Result;
            }
        }
    }

    pub fn reset(&mut self) {
        self.height_input.clear();
        self.velocity_input.clear();
        self.selected_field = InputField::Height;
        self.state = AppState::Input;
        self.result = None;
    }
}
