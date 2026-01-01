//! Yew Demo Page - A stunning showcase of Rust + WebAssembly
//! 
//! Features:
//! - Real-time clock with interval updates
//! - Interactive counter with animations
//! - Fibonacci performance benchmark
//! - Modern glassmorphism UI

use yew::prelude::*;
use gloo_timers::callback::Interval;
use js_sys::Date;

// ============================================================
// MAIN APP COMPONENT
// ============================================================

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="app">
            // Animated background orbs
            <div class="orb orb-1"></div>
            <div class="orb orb-2"></div>
            <div class="orb orb-3"></div>
            
            // Hero Section
            <HeroSection />
            
            // Cards Grid
            <div class="cards-grid">
                <ClockCard />
                <CounterCard />
                <PerformanceCard />
            </div>
            
            // Footer
            <Footer />
        </div>
    }
}

// ============================================================
// HERO SECTION
// ============================================================

#[function_component(HeroSection)]
fn hero_section() -> Html {
    html! {
        <header class="hero">
            <div class="hero-icon">{ "🦀" }</div>
            <h1 class="hero-title">
                { "Powered by " }
                <span class="gradient-text">{ "Rust + WASM" }</span>
            </h1>
            <p class="hero-subtitle">
                { "This entire page is rendered by " }
                <strong>{ "Yew" }</strong>
                { " — a modern Rust framework that compiles to WebAssembly" }
            </p>
            <div class="hero-stats">
                <div class="stat">
                    <span class="stat-value">{ "~280KB" }</span>
                    <span class="stat-label">{ "Bundle Size" }</span>
                </div>
                <div class="stat">
                    <span class="stat-value">{ "0.21" }</span>
                    <span class="stat-label">{ "Yew Version" }</span>
                </div>
                <div class="stat">
                    <span class="stat-value">{ "Native" }</span>
                    <span class="stat-label">{ "Performance" }</span>
                </div>
            </div>
        </header>
    }
}

// ============================================================
// REAL-TIME CLOCK CARD
// ============================================================

#[function_component(ClockCard)]
fn clock_card() -> Html {
    let time = use_state(|| get_current_time());
    let date = use_state(|| get_current_date());

    // Update clock every second
    {
        let time = time.clone();
        let date = date.clone();
        use_effect_with((), move |_| {
            let interval = Interval::new(1000, move || {
                time.set(get_current_time());
                date.set(get_current_date());
            });
            move || drop(interval)
        });
    }

    html! {
        <div class="card card-clock">
            <div class="card-icon">{ "🕐" }</div>
            <h2 class="card-title">{ "Real-Time Clock" }</h2>
            <div class="clock-display">
                <span class="clock-time">{ &*time }</span>
                <span class="clock-date">{ &*date }</span>
            </div>
            <p class="card-caption">
                { "Updates every second via " }
                <code>{ "use_effect" }</code>
            </p>
        </div>
    }
}

fn get_current_time() -> String {
    let date = Date::new_0();
    format!(
        "{:02}:{:02}:{:02}",
        date.get_hours(),
        date.get_minutes(),
        date.get_seconds()
    )
}

fn get_current_date() -> String {
    let date = Date::new_0();
    let days = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    format!(
        "{}, {} {}",
        days[date.get_day() as usize],
        months[date.get_month() as usize],
        date.get_date()
    )
}

// ============================================================
// INTERACTIVE COUNTER CARD
// ============================================================

#[function_component(CounterCard)]
fn counter_card() -> Html {
    let counter = use_state(|| 0_i32);

    let increment = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    let decrement = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter - 1))
    };

    let add_ten = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 10))
    };

    let reset = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(0))
    };

    html! {
        <div class="card card-counter">
            <div class="card-icon">{ "🔢" }</div>
            <h2 class="card-title">{ "Interactive Counter" }</h2>
            <div class="counter-display">
                <span class="counter-value">{ *counter }</span>
            </div>
            <div class="counter-buttons">
                <button class="btn btn-secondary" onclick={decrement}>{ "−1" }</button>
                <button class="btn btn-primary" onclick={increment}>{ "+1" }</button>
                <button class="btn btn-accent" onclick={add_ten}>{ "+10" }</button>
                <button class="btn btn-ghost" onclick={reset}>{ "Reset" }</button>
            </div>
            <p class="card-caption">
                { "State management with " }
                <code>{ "use_state" }</code>
            </p>
        </div>
    }
}

// ============================================================
// PERFORMANCE BENCHMARK CARD
// ============================================================

#[function_component(PerformanceCard)]
fn performance_card() -> Html {
    let result = use_state(|| 0_u64);
    let time_ms = use_state(|| 0.0_f64);
    let n = use_state(|| 40_u32);

    let calculate = {
        let result = result.clone();
        let time_ms = time_ms.clone();
        let n = n.clone();
        Callback::from(move |_| {
            let start = js_sys::Date::now();
            let fib_result = fibonacci(*n);
            let end = js_sys::Date::now();
            
            result.set(fib_result);
            time_ms.set(end - start);
        })
    };

    let increase_n = {
        let n = n.clone();
        Callback::from(move |_| n.set((*n + 2).min(50)))
    };

    let decrease_n = {
        let n = n.clone();
        Callback::from(move |_| n.set((*n).saturating_sub(2).max(10)))
    };

    html! {
        <div class="card card-performance">
            <div class="card-icon">{ "⚡" }</div>
            <h2 class="card-title">{ "WASM Performance" }</h2>
            <div class="perf-control">
                <button class="btn btn-sm" onclick={decrease_n}>{ "−" }</button>
                <span class="perf-n">{ format!("fib({})", *n) }</span>
                <button class="btn btn-sm" onclick={increase_n}>{ "+" }</button>
            </div>
            <button class="btn btn-primary btn-large" onclick={calculate}>
                { "Calculate Fibonacci" }
            </button>
            <div class="perf-results">
                <div class="perf-result">
                    <span class="perf-value">{ format_number(*result) }</span>
                    <span class="perf-label">{ "Result" }</span>
                </div>
                <div class="perf-result">
                    <span class="perf-value perf-time">{ format!("{:.2}ms", *time_ms) }</span>
                    <span class="perf-label">{ "Compute Time" }</span>
                </div>
            </div>
            <p class="card-caption">
                { "Pure Rust running at " }
                <strong>{ "native speed" }</strong>
            </p>
        </div>
    }
}

fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0_u64;
            let mut b = 1_u64;
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        }
    }
}

fn format_number(n: u64) -> String {
    if n == 0 {
        return "—".to_string();
    }
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.insert(0, ',');
        }
        result.insert(0, c);
    }
    result
}

// ============================================================
// FOOTER
// ============================================================

#[function_component(Footer)]
fn footer() -> Html {
    html! {
        <footer class="footer">
            <div class="tech-badges">
                <span class="badge badge-rust">{ "🦀 Rust" }</span>
                <span class="badge badge-yew">{ "⚙️ Yew 0.21" }</span>
                <span class="badge badge-wasm">{ "🌐 WASM" }</span>
                <span class="badge badge-trunk">{ "📦 Trunk" }</span>
            </div>
            <p class="footer-text">
                { "Built with ❤️ using the Yew Framework" }
            </p>
        </footer>
    }
}

// ============================================================
// ENTRY POINT
// ============================================================

fn main() {
    yew::Renderer::<App>::new().render();
}
