# set the day to run
DAY=day03

# Environment varables
ENV_VARS=CARGO_INCREMENTAL=1

run:
	$(ENV_VARS) cargo run --bin $(DAY)

watch:
	$(ENV_VARS) cargo watch -s "cargo test --bin $(DAY) -- --nocapture"