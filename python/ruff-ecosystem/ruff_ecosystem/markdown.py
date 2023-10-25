from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from ruff_ecosystem.projects import Project


def markdown_project_section(
    title: str, content: str | list[str], options: str, project: Project
) -> list[str]:
    return markdown_details(
        summary=f'<a href="{project.repo.url}">{project.repo.fullname}</a> ({title})',
        preface="`ruff " + " ".join(options.to_cli_args()) + "`",
        content=content,
    )


def markdown_details(summary: str, preface: str, content: str | list[str]):
    lines = []
    lines.append(f"<details><summary>{summary}</summary>")
    lines.append(preface)
    lines.append("<p>")
    lines.append("")

    if isinstance(content, str):
        lines.append(content)
    else:
        lines.extend(content)

    lines.append("")
    lines.append("</p>")
    lines.append("</details>")
    return lines