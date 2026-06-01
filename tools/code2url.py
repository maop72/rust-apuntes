#!/usr/bin/env python3
"""
code2url.py

Scan Quarto .qmd files, associate each managed playground link with the
most recent include directive found before it, and create or update the
URL so that the code parameter always contains the current source file
encoded using standard percent encoding.
"""

import argparse
import os
import re
import sys
import urllib.parse


LINK_TEXT = "[Ejecutar▶]"

LINK_DESTINATION = (
    "https://play.rust-lang.org/?version=stable&edition=2024&code="
    "ENCODED_CODE"
)


INCLUDE_RE = re.compile(
    r"""
    ^\s*
    \{\{<
    \s+include\s+
    (?P<path>[^>]+?)
    \s*
    >\}\}
    \s*$
    """,
    re.VERBOSE,
)

MANAGED_LINK_RE = re.compile(
    r"""
    ^
    \[
    (?P<text>[^\]]*)
    \]
    \(
    (?P<url>.*)
    \)
    $
    """,
    re.VERBOSE,
)

MARKER_RE = re.compile(
    r"""
    ^
    \[
    (?P<text>[^\]]*)
    \]
    $
    """,
    re.VERBOSE,
)


def normalize_link_text(text):
    """Normalize spaces immediately inside square brackets."""

    text = re.sub(r"^\[\s*", "[", text)
    text = re.sub(r"\s*\]$", "]", text)
    return text


def is_link_text_equivalent(raw_text):
    """Check whether a link text matches LINK_TEXT."""

    candidate = "[{}]".format(raw_text)
    return normalize_link_text(candidate) == LINK_TEXT


def read_text_preserve_newlines(path):
    """Read a UTF-8 text file preserving line endings."""

    handle = open(path, "r", encoding="utf-8", newline="")
    try:
        return handle.read()
    finally:
        handle.close()


def write_text_preserve_newlines(path, content):
    """Write a UTF-8 text file preserving provided line endings."""

    handle = open(path, "w", encoding="utf-8", newline="")
    try:
        handle.write(content)
    finally:
        handle.close()


def encode_code(code_text):
    """Percent-encode UTF-8 source code."""

    return urllib.parse.quote_from_bytes(code_text.encode("utf-8"))


def extract_code_parameter(url):
    """Extract and decode the code parameter from a URL."""

    parsed = urllib.parse.urlparse(url)
    params = urllib.parse.parse_qs(parsed.query, keep_blank_values=True)

    if "code" not in params:
        return None

    encoded_value = params["code"][0]
    decoded_bytes = urllib.parse.unquote_to_bytes(encoded_value)
    return decoded_bytes.decode("utf-8")


def replace_code_parameter(url, code_text):
    """Replace or add the code parameter in a URL."""

    parsed = urllib.parse.urlparse(url)
    query_items = urllib.parse.parse_qsl(
        parsed.query,
        keep_blank_values=True,
    )

    encoded_code = encode_code(code_text)

    found = False
    new_items = []

    for key, value in query_items:
        if key == "code":
            new_items.append(("code", encoded_code))
            found = True
        else:
            new_items.append((key, value))

    if not found:
        new_items.append(("code", encoded_code))

    query_parts = []

    for key, value in new_items:
        if key == "code":
            query_parts.append("code={}".format(value))
        else:
            query_parts.append(
                "{}={}".format(
                    urllib.parse.quote_plus(key),
                    urllib.parse.quote_plus(value),
                )
            )

    new_query = "&".join(query_parts)

    return urllib.parse.urlunparse(
        (
            parsed.scheme,
            parsed.netloc,
            parsed.path,
            parsed.params,
            new_query,
            parsed.fragment,
        )
    )


def build_new_link(code_text):
    """Create a new playground link from LINK_DESTINATION."""

    encoded_code = encode_code(code_text)
    url = LINK_DESTINATION.replace("ENCODED_CODE", encoded_code)
    return "{}({})".format(LINK_TEXT, url)


def find_qmd_files(root_directory):
    """Return all .qmd files below root_directory."""

    result = []

    for current_root, _, files in os.walk(root_directory):
        for filename in files:
            if filename.endswith(".qmd"):
                result.append(os.path.join(current_root, filename))

    result.sort()
    return result


def verbose_print(enabled, message):
    if enabled:
        print(message)


def process_qmd_file(qmd_path, verbose=False):
    """Process one .qmd file and update managed links."""

    content = read_text_preserve_newlines(qmd_path)
    lines = content.splitlines(True)

    changed = False
    file_status = "up to date"

    last_include_path = None
    last_code = None

    new_lines = []

    for line in lines:
        line_ending = ""

        if line.endswith("\r\n"):
            line_ending = "\r\n"
            stripped = line[:-2]
        elif line.endswith("\n"):
            line_ending = "\n"
            stripped = line[:-1]
        else:
            stripped = line

        include_match = INCLUDE_RE.match(stripped)

        if include_match:
            include_path = include_match.group("path").strip()

            code_path = os.path.join(
                os.path.dirname(qmd_path),
                include_path,
            )

            if not os.path.exists(code_path):
                raise FileNotFoundError(
                    "{}: {} not found".format(
                        qmd_path,
                        include_path,
                    )
                )

            last_include_path = include_path
            last_code = read_text_preserve_newlines(code_path)

            verbose_print(
                verbose,
                "include: {}".format(include_path),
            )

            new_lines.append(line)
            continue

        managed_match = MANAGED_LINK_RE.match(stripped)

        if managed_match:
            if not is_link_text_equivalent(
                managed_match.group("text")
            ):
                new_lines.append(line)
                continue

            if last_code is None:
                raise RuntimeError(
                    "{}: LINK_TEXT before first include".format(
                        qmd_path
                    )
                )

            url = managed_match.group("url")
            current_code = extract_code_parameter(url)

            if current_code == last_code:
                verbose_print(
                    verbose,
                    "{} up to date.".format(last_include_path),
                )
                new_lines.append(line)
                continue

            new_url = replace_code_parameter(url, last_code)
            new_line = "{}({}){}".format(
                LINK_TEXT,
                new_url,
                line_ending,
            )

            new_lines.append(new_line)
            changed = True
            file_status = "updated"

            verbose_print(
                verbose,
                "{} updated.".format(last_include_path),
            )
            continue

        marker_match = MARKER_RE.match(stripped)

        if marker_match:
            if not is_link_text_equivalent(
                marker_match.group("text")
            ):
                new_lines.append(line)
                continue

            if last_code is None:
                raise RuntimeError(
                    "{}: LINK_TEXT before first include".format(
                        qmd_path
                    )
                )

            new_line = "{}{}".format(
                build_new_link(last_code),
                line_ending,
            )

            new_lines.append(new_line)

            changed = True

            if file_status == "up to date":
                file_status = "included"

            verbose_print(
                verbose,
                "{} included.".format(last_include_path),
            )
            continue

        new_lines.append(line)

    if changed:
        write_text_preserve_newlines(
            qmd_path,
            "".join(new_lines),
        )

    print("{}: {}".format(qmd_path, file_status))


def parse_arguments():
    """Parse command line arguments."""

    parser = argparse.ArgumentParser(
        description=(
            "Create or update playground links associated with "
            "Quarto include directives."
        )
    )

    parser.add_argument(
        "-v",
        "--verbose",
        action="store_true",
        help="show diagnostic information",
    )

    return parser.parse_args()


def main():
    args = parse_arguments()

    qmd_files = find_qmd_files(".")

    for qmd_file in qmd_files:
        process_qmd_file(
            qmd_path=qmd_file,
            verbose=args.verbose,
        )

    return 0


if __name__ == "__main__":
    sys.exit(main())
