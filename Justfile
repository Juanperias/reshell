default: build

build: always
	cargo rustc --release --target dos.json  -- -g -C relocation-model=static -C link-args=-no-pie --emit=obj
	cp target/dos/release/deps/reshell-*.o dist/reshell.o	
	objconv -fCOFF[32] dist/reshell.o -o dist/reshell.obj
	i586-pc-msdosdjgpp-ld -o dist/reshell.exe dist/reshell.obj
	mv dist/reshell.exe reshell.exe

clean:
	cargo clean
	rm -rf dist

always:
	mkdir -p dist
