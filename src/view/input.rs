use maud::{html, Markup, PreEscaped, Render};
use std::fmt::Display;

pub struct Input<'a> {
    pub label: &'a str,
    pub field_name: &'a str,
    pub kind: InputKind,
    pub value: Option<&'a str>,
    pub errors: Option<&'a Vec<String>>,
    pub on_change_validation: Option<OnChangeValidation>,
}

pub enum InputKind {
    Text,
    Email,
    Password,
}

impl Display for InputKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_str(match self {
            InputKind::Text => "text",
            InputKind::Email => "email",
            InputKind::Password => "password",
        });
    }
}

pub enum OnChangeValidation {
    Username,
    Email,
}

impl Display for OnChangeValidation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_str(match self {
            OnChangeValidation::Username => "/check-username",
            OnChangeValidation::Email => "/check-email",
        });
    }
}

impl<'a> Input<'a> {
    pub fn new(label: &'a str, field_name: &'a str) -> Self {
        return Self {
            label,
            field_name,
            kind: InputKind::Text,
            value: None,
            errors: None,
            on_change_validation: None,
        };
    }

    pub fn kind(mut self, kind: InputKind) -> Self {
        self.kind = kind;
        return self;
    }

    pub fn errors(mut self, errors: Option<&'a Vec<String>>) -> Self {
        self.errors = errors;
        return self;
    }

    pub fn value(mut self, value: &'a str) -> Self {
        self.value = Some(value);
        return self;
    }

    pub fn validate_on_change(mut self, custom_validation: OnChangeValidation) -> Self {
        self.on_change_validation = Some(custom_validation);
        return self;
    }
}

impl<'a> Render for Input<'a> {
    fn render(&self) -> Markup {
        return html! {
            div class="form-control" id={ "control_"(self.field_name) } {
                label class="label" for=(self.field_name) {
                    span class="capitalize" { (self.label)": " span class="text-red-500" { "*" } }
                }
                input id=(self.field_name)
                        type=(self.kind.to_string())
                        class={ "input input-bordered bg-white "(if self.errors.is_some() { "input-error" } else { "" }) }
                        name=(self.field_name)
                        value=(self.value.unwrap_or(""))
                        required
                        hx-trigger=[self.on_change_validation.as_ref().map(|_| "keyup changed delay:500ms")]
                        hx-swap=[self.on_change_validation.as_ref().map(|_| "morphdom")]
                        hx-post=[self.on_change_validation.as_ref().map(|v| v.to_string())]
                        hx-target=[self.on_change_validation.as_ref().map(|_| format!("#control_{}",self.field_name))];
                (self.errors.map_or(PreEscaped("".to_string()), |errors| error(self.field_name, &errors)))
            }
        };
    }
}

fn error(field_name: &str, errors: &Vec<String>) -> Markup {
    return html! {
        label class="label text-red-500" for=(field_name) {
            @for error in errors {
                span { (error) }
            }
        }
    };
}
