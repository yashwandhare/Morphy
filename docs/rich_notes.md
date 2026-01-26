# Rich usage notes (Morphy)

Personal notes on how Rich is used in Morphy and what it replaces.
Scope is limited strictly to what exists in the codebase.

---

## Console

```python
from rich.console import Console
console = Console()
````

**Before**

```python
print("File found:", path)
```

**After**

```python
console.print(f"[green]✓ File found:[/green] {path}")
```

**Improvement**

* Centralized output
* Color + formatting without manual ANSI codes
* Required for advanced features (status, tables, rules)

Used everywhere UI output exists.

---

## console.print()

```python
console.print("[bold red]ERROR[/bold red]")
```

**Before**

```python
print("ERROR: File not found")
```

**After**

```python
console.print(f"[{Theme.ERROR}]✗ File not found[/{Theme.ERROR}]")
```

**Improvement**

* Consistent semantics (error, success, info)
* Theme-controlled colors
* More readable terminal output

---

## Prompt.ask()

```python
from rich.prompt import Prompt
```

**Before**

```python
choice = input("Enter choice: ")
if choice not in ["1", "2"]:
    print("Invalid choice")
```

**After**

```python
choice = Prompt.ask(
    "Enter choice",
    choices=["1", "2"],
    default="1"
)
```

**Improvement**

* Built-in validation
* No manual loops or checks
* Predictable user input

Used in menus, file picker, conversion options.

---

## Table

```python
from rich.table import Table
```

**Before**

```python
print("Filename:", img.filename)
print("Format:", img.format)
print("Size:", img.size)
```

**After**

```python
table = Table(show_header=False)
table.add_row("Filename", img.filename)
table.add_row("Format", img.format)
table.add_row("Size", f"{w}x{h}")
console.print(table)
```

**Improvement**

* Structured, scannable output
* No alignment hacks
* Better for metadata display

Used for image metadata and file stats.

---

## console.status()

```python
with console.status("Processing..."):
    run_long_task()
```

**Before**

```python
print("Processing...")
run_long_task()
print("Done")
```

**After**

```python
with console.status("[info]Processing...[/info]"):
    run_long_task()
```

**Improvement**

* Visual feedback during blocking operations
* No fake progress messages
* Cleaner UX for FFmpeg and PDF work

---

## console.rule()

```python
console.rule("VIDEO TO GIF")
```

**Before**

```python
print("==== VIDEO TO GIF ====")
```

**After**

```python
console.rule(f"[bold {Theme.HEADER}]VIDEO TO GIF[/bold {Theme.HEADER}]")
```

**Improvement**

* Clean section separation
* Consistent styling
* Easier to visually parse CLI flow

---

## track() (progress bar)

```python
from rich.progress import track
```

**Before**

```python
for page in pdf:
    process(page)
```

**After**

```python
for page in track(pdf, total=len(pdf)):
    process(page)
```

**Improvement**

* Visible progress for multi-page PDFs
* Zero logic change
* User knows it’s working

---

## Theme integration

```python
Theme.SUCCESS
Theme.ERROR
Theme.INFO
```

**Before**

```python
console.print("[green]Done[/green]")
console.print("[red]Error[/red]")
```

**After**

```python
console.print(f"[{Theme.SUCCESS}]✓ Done[/{Theme.SUCCESS}]")
console.print(f"[{Theme.ERROR}]✗ Error[/{Theme.ERROR}]")
```

**Improvement**

* Single source of truth for colors
* Easy UI tweaks without touching logic
* Consistent meaning across the app

---

## Notes

* Rich is only used in UI-facing code
* No business logic depends on Rich
* Removing Rich would not break functionality, only UX
* All Rich usage is optional but improves clarity and safety

End of notes.
