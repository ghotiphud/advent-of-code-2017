# set the day to run
DAY=day10

# Environment varables
ENV_VARS=CARGO_INCREMENTAL=1

run:
	$(ENV_VARS) cargo run --bin $(DAY) --release

watch:
	$(ENV_VARS) cargo watch -s "cargo test --bin $(DAY) -- --nocapture"