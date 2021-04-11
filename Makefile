include config.mk

all: koi

${OBJ}: config.mk

koi: ${OBJ}
	cargo build --release

install: all
	mkdir -p ${DESTDIR}${PREFIX}/bin
	cp -f target/release/koi ${DESTDIR}${PREFIX}/bin
	chmod 755 ${DESTDIR}${PREFIX}/bin/koi

uninstall:
	rm -f ${DESTDIR}${PREFIX}/bin/koi
