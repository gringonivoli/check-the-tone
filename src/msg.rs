pub struct Msg {
    prompt_tpl: String,
    value: String,
}

impl Msg {
    pub fn default(value: String) -> Self {
        Msg {
            value,
            prompt_tpl: "Check the tone of the following message.
                1. Say whether the tone is polite, clear, friendly and collaborative.
                2. Then provide a revised version of the message.

                Use this exact format:

                Tone:
                <your evaluation>

                Revised message:
                <rewritten message>

                Message: <_PASTE MESSAGE HERE_>"
                .to_string(),
        }
    }

    pub fn prompt(&self) -> String {
        self.prompt_tpl
            .replace("<_PASTE MESSAGE HERE_>", &self.value)
    }
}

impl From<&str> for Msg {
    fn from(value: &str) -> Self {
        Self::default(value.to_string())
    }
}

#[cfg(test)]
#[test]
fn test_msg_prompt() {
    let test_value = "a test value";
    let msg: Msg = test_value.into();

    assert!(msg.prompt().contains(test_value));
    assert!(msg.prompt().contains("Check the tone"));
}
