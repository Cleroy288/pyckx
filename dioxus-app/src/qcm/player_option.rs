//! Answer option rendering for the QCM player.

use dioxus::prelude::*;

use crate::game_engine::GamePlayerState;
use crate::qcm::types::QcmQuestion;

/// Base style for any answer option button.
pub const OPTION_BASE: &str =
    "w-full text-left px-5 py-3 \
     text-sm font-medium rounded-[12px] \
     bg-[var(--card)] \
     border-[1.5px] border-[var(--color-border)] \
     text-[var(--color-text-primary)] \
     cursor-pointer \
     transition-[transform,border-color] duration-150 \
     hover:border-[var(--color-primary)] \
     hover:-translate-y-0.5 \
     disabled:cursor-default disabled:opacity-70 \
     disabled:hover:translate-y-0";

/// Style overlay marking the correct option after submit.
const OPTION_RIGHT: &str =
    "!border-[var(--color-success)] \
     !bg-[var(--color-success-bg)] \
     !text-[var(--color-success-text)]";

/// Style overlay marking the user's wrong choice.
const OPTION_WRONG: &str =
    "!border-[var(--color-error)] \
     !bg-[var(--color-error-bg)] \
     !text-[var(--color-error-text)]";

/// Context needed to render one answer option.
#[derive(Clone)]
pub struct OptionCtx {
    pub sel: Signal<Option<String>>,
    pub ans: Signal<bool>,
    pub right: Memo<String>,
    pub state: GamePlayerState,
    pub questions: Vec<QcmQuestion>,
}

/// Render a single answer option as a clickable button.
pub fn render_option(
    answer: String,
    ctx: OptionCtx,
) -> Element {
    let display = answer.clone();
    let for_class = answer.clone();
    let sel = ctx.sel;
    let ans = ctx.ans;
    let right = ctx.right;
    let class_value = use_memo(move || {
        option_class(
            &for_class, &(sel)(), (ans)(), &(right)(),
        )
    });
    let on_click = move |_| {
        record_answer(
            &answer, sel, ans, ctx.state,
            &ctx.questions,
        );
    };
    rsx! {
        button {
            class: "{class_value}",
            disabled: (ans)(),
            onclick: on_click,
            "{display}"
        }
    }
}

/// Lock in the user's answer and record the result.
fn record_answer(
    answer: &str,
    mut sel: Signal<Option<String>>,
    mut ans: Signal<bool>,
    state: GamePlayerState,
    questions: &[QcmQuestion],
) {
    if (ans)() {
        return;
    }
    sel.set(Some(answer.to_string()));
    ans.set(true);
    let q = &questions[(state.current)()];
    state.record(
        answer == q.right_answer,
        q.question.clone(),
        q.explanation.clone(),
    );
}

/// Compute the CSS class for an answer option.
fn option_class(
    answer: &str,
    selected: &Option<String>,
    answered: bool,
    right: &str,
) -> String {
    if !answered {
        return OPTION_BASE.to_string();
    }
    if answer == right {
        return format!("{OPTION_BASE} {OPTION_RIGHT}");
    }
    if selected.as_deref() == Some(answer) {
        return format!("{OPTION_BASE} {OPTION_WRONG}");
    }
    OPTION_BASE.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_class_unanswered_returns_base() {
        let cls = option_class("a", &None, false, "right");
        assert_eq!(cls, OPTION_BASE);
    }

    #[test]
    fn test_option_class_right_answer_marks_right() {
        let cls = option_class(
            "right",
            &Some("right".into()),
            true,
            "right",
        );
        assert!(cls.contains(OPTION_RIGHT));
    }

    #[test]
    fn test_option_class_selected_wrong_marks_wrong() {
        let cls = option_class(
            "wrong",
            &Some("wrong".into()),
            true,
            "right",
        );
        assert!(cls.contains(OPTION_WRONG));
    }

    #[test]
    fn test_option_class_other_unselected_stays_base() {
        let cls = option_class(
            "other",
            &Some("wrong".into()),
            true,
            "right",
        );
        assert_eq!(cls, OPTION_BASE);
    }
}
