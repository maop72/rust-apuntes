.PHONY: all links grafos common html pdf clean preview

CODE2URL := ./tools/code2url.py

links:
	$(CODE2URL)

# Generación de los grafos a partir de fuentes Graphviz
grafos:
	@for f in grafos/*.dot; do \
		[ -f "$$f" ] || continue; \
		dot -Tsvg "$$f" -o "$${f%.dot}.svg"; \
	done

common: links grafos

html: common
	quarto render --to html

pdf: common
	quarto render --to pdf

all: common
	quarto render

clean:
	rm -rf _book .quarto
	rm -f grafos/*.svg

preview: links grafos
	quarto preview &>/dev/null
