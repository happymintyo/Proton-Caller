build:
	g++ -o proton-call src/*.cpp -std=c++17

install:
	mkdir -p /usr/share/licenses/proton-call/
	install -g 0 -o 0 ./proton-call usr/bin/
	install -g 0 -o 0 ./LICENSE /usr/share/licenses/proton-call/
	install -g 0 -o 0 -m 0644 manual/proton-call.6 /usr/share/man/man6/
	chmod +x /usr/bin/proton-call
	gzip -f usr/share/man/man6/proton-call.6


install-pkgbuild:
	mkdir -p "$pkgdir"/usr/share/licenses/proton-call/
	install -g 0 -o 0 ./proton-call "$pkgdir"/usr/bin/
	install -g 0 -o 0 ./LICENSE "$pkgdir"/usr/share/licenses/proton-call/
	install -g 0 -o 0 -m 0644 manual/proton-call.6 "$pkgdir"/usr/share/man/man6/
	chmod +x "$pkgdir"/usr/bin/proton-call
	gzip -f "$pkgdir"/usr/share/man/man6/proton-call.6

uninstall:
	rm -f ./proton-call /usr/bin/proton-call
	rm -rf /usr/share/licenses/proton-call/
	rm -f /usr/share/man/man6/proton-call.6.gz
