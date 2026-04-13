//! True/False player — one statement at a time.

use dioxus::prelude::*;

use crate::game_engine::{
    derive_field, GamePlayerState, GameProgress,
    GameResults,
};
use crate::ui::{Button, ButtonVariant};

use super::types::{TrueFalseSet, TrueFalseStatement};

const PLAYER_CLS: &str =
    "flex flex-col gap-6 max-w-[700px] \
     mx-auto w-full items-center";

const STATEMENT_CLS: &str =
    "text-lg font-semibold \
     text-[var(--color-text-primary)] \
     p-6 bg-[var(--card)] \
     border-2 border-[var(--foreground)] \
     rounded-[12px] text-center w-full";

const CHOICES_CLS: &str =
    "flex gap-4 justify-center w-full";

/// True/False player — drives a full statement run.
#[component]
pub fn TrueFalsePlayer(
    /// Set of statements to play.
    set: TrueFalseSet,
    /// Called when the user wants to leave the player.
    on_back: EventHandler<()>,
) -> Element {
    let total = set.statements.len();
    let stmts = use_hook(|| set.statements.clone());
    let state = GamePlayerState::new(total);
    let answered = use_signal(|| false);
    let text =
        derive_field(stmts.clone(), state.current, |s| {
            s.statement.clone()
        });

    if (state.finished)() {
        return render_results(state, total, on_back);
    }
    render_play(PlayCtx { state, stmts, answered, text })
}

/// Bundle of reactive context for the play view.
struct PlayCtx {
    state: GamePlayerState,
    stmts: Vec<TrueFalseStatement>,
    answered: Signal<bool>,
    text: Memo<String>,
}

/// Render the end-of-run results screen.
fn render_results(
    state: GamePlayerState,
    total: usize,
    on_back: EventHandler<()>,
) -> Element {
    rsx! {
        GameResults {
            score: (state.score)(),
            total: total,
            items: (state.results)(),
            on_replay: move |_| state.replay(),
            on_back: move |_| on_back.call(()),
        }
    }
}

/// Render the active play view (statement + choices).
fn render_play(ctx: PlayCtx) -> Element {
    let total = ctx.stmts.len();
    let state = ctx.state;
    let mut answered = ctx.answered;
    let text = ctx.text;
    let choices = render_choices(&ctx);
    rsx! {
        div { class: PLAYER_CLS,
            GameProgress {
                current: state.current,
                total: total,
            }
            div { class: STATEMENT_CLS, "{text}" }
            div { class: CHOICES_CLS, {choices} }
            if answered() {
                Button {
                    text: "Next",
                    on_click: move |_| {
                        answered.set(false);
                        state.advance();
                    },
                }
            }
        }
    }
}

/// Render the True and False choice buttons.
fn render_choices(ctx: &PlayCtx) -> Element {
    let on_true = make_choice_handler(ctx, true);
    let on_false = make_choice_handler(ctx, false);
    let disabled = ctx.answered;
    rsx! {
        Button {
            text: "True",
            variant: ButtonVariant::Primary,
            disabled: disabled,
            on_click: on_true,
        }
        Button {
            text: "False",
            variant: ButtonVariant::Destructive,
            disabled: disabled,
            on_click: on_false,
        }
    }
}

/// Build the click handler for a True or False choice.
fn make_choice_handler(
    ctx: &PlayCtx,
    answer: bool,
) -> impl FnMut(MouseEvent) + 'static {
    let mut answered = ctx.answered;
    let stmts = ctx.stmts.clone();
    let state = ctx.state;
    move |_| {
        if answered() {
            return;
        }
        answered.set(true);
        let stmt = &stmts[(state.current)()];
        state.record(
            stmt.answer == answer,
            stmt.statement.clone(),
            stmt.explanation.clone(),
        );
    }
}

