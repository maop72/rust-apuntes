# README.quarto.md

July, 2026

# Custom Full-Page PDF Cover for Quarto Books

This project uses a custom full-page cover for the PDF version of
a Quarto book while preserving all the advantages of Quarto's PDF
generation.

The solution has the following properties:

- Full-page cover (edge-to-edge).
- Original Quarto title page preserved (title, author, date,
  etc.).
- Table of contents, hyperlinks, bookmarks and page references
  remain intact.
- No PDF post-processing (`pdfunite`, `qpdf`, etc.).
- Works directly with `quarto render`, including GitHub Actions.

## Background

Quarto's `cover-image` option only affects HTML output.

For PDF output, Quarto normally generates a title page by calling
`\maketitle`. Replacing this with a full-page image is not
straightforward because ordinary LaTeX commands such as
`\includegraphics` are constrained by the page margins.

The solution described here inserts an additional page *before*
Quarto's title page using a custom template partial and the
`eso-pic` package.

## Directory layout

``` text
.
├── _quarto.yml
├── header.tex
├── templates/
│   └── before-body.tex
└── portada.pdf
```

`portada.pdf` is the custom full-page cover.

## Step 1: Use a custom template partial

Copy Quarto's standard `before-body.tex` template into your
project:

``` text
templates/before-body.tex
```

and enable it in `_quarto.yml`:

``` yaml
format:
  pdf:
    template-partials:
      - templates/before-body.tex
```

This allows inserting arbitrary LaTeX immediately before Quarto
generates its title page.

## Step 2: Load the required packages

In `header.tex`:

``` latex
\usepackage{graphicx}
\usepackage{eso-pic}
```

## Step 3: Insert the custom cover

Replace the contents of `templates/before-body.tex` with
something similar to:

``` latex
$if(has-frontmatter)$
\frontmatter
$endif$

%--------------------------------------------------
% Custom full-page cover
%--------------------------------------------------

\thispagestyle{empty}

\AddToShipoutPictureFG*{%
  \AtPageLowerLeft{%
    \includegraphics[
      width=\paperwidth,
      height=\paperheight
    ]{portada.pdf}%
  }%
}

\null
\clearpage

%--------------------------------------------------
% Standard Quarto title page
%--------------------------------------------------

$if(title)$
\maketitle

$if(abstract)$
\begin{abstract}
$abstract$
\end{abstract}
$endif$

$endif$
```

The first page becomes the custom cover, while the second page is
Quarto's standard title page containing the title, author and
date.

## Why `eso-pic`?

A normal `\includegraphics` is placed inside LaTeX's text area
and therefore respects the document margins.

`eso-pic` draws directly onto the physical page during shipout,
allowing the cover to occupy the entire paper size (`\paperwidth`
× `\paperheight`).

## Advantages

Compared with merging PDFs afterwards, this approach:

- keeps hyperlinks and bookmarks intact,
- preserves page numbering,
- works with GitHub Actions,
- requires no external PDF manipulation tools,
- remains entirely within Quarto's rendering pipeline.

## Notes

This solution relies only on standard Quarto customization
mechanisms (`template-partials`) and standard LaTeX packages
(`graphicx` and `eso-pic`).

It has been tested with recent versions of Quarto and produces a
professional two-stage front matter:

1.  Full-page custom cover.
2.  Standard Quarto title page.
3.  Table of contents.
4.  Main document.
