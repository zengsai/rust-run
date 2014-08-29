rr: main.rs
	rustc -o rr main.rs

clean:
	rm -rf rr

install: rr
	cp rr /usr/local/bin/

uninstall:
	rm -rf /usr/local/bin/rr
