test:
	for dir in day_*; do \
		(cd "$$dir" && cargo test) || exit 1; \
	done
