.PHONY: all links common html pdf clean preview

CODE2URL := ./tools/code2url.py

links:
	$(CODE2URL)

common: links
	git log -1 --format="Última actualización: %cs." > date.md

html: common
	quarto render --to html

pdf: common
	quarto render --to pdf

all: common 
	quarto render

clean:
	rm -rf _book .quarto

preview: links
	quarto preview &>/dev/null
