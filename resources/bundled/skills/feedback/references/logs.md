# Log And Crash Artifact Guidance

Use this only for crashes, startup failures, rendering bugs, sync issues, or hard-to-reproduce regressions.

- Ask for logs only when they are likely to improve the report.
- Note in the issue that logs or crash reports were attached, but do not claim they contain console input or output.
- In the `Artifacts` section, mention the exact file names or bundles that were attached.

macOS paths and commands:

- Logs live under `~/Library/Logs/`
- Stable app logs are typically `~/Library/Logs/cute.log*`
- Preview app logs are typically `~/Library/Logs/cute_preview.log*`
- Stable zip command: `zip -j ~/Desktop/cute-logs.zip ~/Library/Logs/cute.log*`
- Preview zip command: `zip -j ~/Desktop/cute_preview-logs.zip ~/Library/Logs/cute_preview.log*`
- If Cute still opens, the user can search `View Cute Logs` in the Command Palette
- Crash reports may also exist under `~/Library/Logs/DiagnosticReports/` as Cute `.ips` files

Linux paths:

- Logs live under Cute's state directory.
- Stable app logs are typically `~/.local/state/cute-terminal/cute.log*`
- Preview app logs are typically `~/.local/state/cute-terminal-preview/cute_preview.log*`
- If the exact channel is unclear, ask the user to open the nearest `cute*.log*` files under `~/.local/state/`

Windows paths:

- Logs live under Cute's local app data state directory.
- Stable app logs are typically `%LOCALAPPDATA%\cute\Cute\data\logs\cute.log*`
- Preview app logs are typically `%LOCALAPPDATA%\cute\CutePreview\data\logs\cute_preview.log*`
- If the exact channel is unclear, ask the user to look under `%LOCALAPPDATA%\cute\` for the relevant `Cute*` folder and attach the matching `cute*.log*` files from its `data\logs\` directory

If no artifacts are available, say so plainly instead of implying they were checked.
