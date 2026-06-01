.PHONY: all render preview clean

all: render

render:
	./tools/code2url.py
	quarto render

preview:
	./code2url.py -v
	quarto preview

clean:
	rm -rf _book .quarto

