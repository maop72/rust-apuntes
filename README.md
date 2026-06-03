Material didáctico sobre Rust, escrito por Miguel Ortuño en
Quarto. <https://quarto.org/>.

Quarto es una herramienta que, a partir de texto Markdown, puede
generar PDF, HTML y otros formatos. El HTML puede incluir
ejemplos de código en diversos lenguajes de programación,
ejecutables directamente desde la propia página web que se
generará.

# Instalación

``` bash
apt install quarto

# Quarto usará xelatex para generar PDF.
apt install texlive texlive-xetex texlive-latex-extra
```

# Uso

## Previsualización interactiva

Para ver el documento completo en HTML, basta con ejecutar,
dentro del directorio que lo contiene, el comando

``` bash
make preview
```

Esto lanza un servidor web local, abre el navegador y recompila
automáticamente el documento si detecta algún cambio. 

## Generar HTML y PDF

Esta orden genera el documento completo en el subdirectorio
*\_book*.  

``` bash
make all
```

# Limpieza

``` bash
make clean
```

# Lectura online

<https://maop72.github.io/rust-apuntes/>

# Versión en PDF

<https://maop72.github.io/rust-apuntes/Apuntes-de-programaci%C3%B3n-en-Rust.pdf>

# Código fuente

<https://github.com/maop72/rust-apuntes>

# Licencias

- Documentación: CC BY-SA 4.0 (véase LICENSE-CC-BY-SA).
- Ejemplos de código Rust: MIT (véase LICENSE-MIT).
