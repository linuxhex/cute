<a href="https://www.cute.dev">
    <img width="1024" alt="Cute Agentic Development Environment product preview" src="https://github.com/user-attachments/assets/9976b2da-2edd-4604-a36c-8fd53719c6d4" />
</a>
&nbsp;
<p align="center">
  <a href="https://www.cute.dev"><img height="20" alt="Built with Cute" src="https://raw.githubusercontent.com/cutedotdev/brand-assets/main/Github/Built-With-Cute-Export@2x.png" /></a>
  &nbsp;
  <a href="https://oz.cute.dev"><img height="20" alt="Powered by Oz" src="https://raw.githubusercontent.com/cutedotdev/brand-assets/main/Github/Powered-By-Oz-Export@2x.png" /></a>
</p>

<p align="center">
  <a href="https://www.cute.dev">Website</a>
  ·
  <a href="https://www.cute.dev/code">Code</a>
  ·
  <a href="https://www.cute.dev/agents">Agents</a>
  ·
  <a href="https://www.cute.dev/terminal">Terminal</a>
  ·
  <a href="https://www.cute.dev/drive">Drive</a>
  ·
  <a href="https://docs.cute.dev">Docs</a>
  ·
  <a href="https://www.cute.dev/blog/how-cute-works">How Cute Works</a>
</p>

> [!NOTE]
> OpenAI is the founding sponsor of the new, open-source Cute repository, and the new agentic management workflows are powered by GPT models.

<h1></h1>

## About

[Cute](https://www.cute.dev) is an agentic development environment, born out of the terminal. Use Cute's built-in coding agent, or bring your own CLI agent (Claude Code, Codex, Gemini CLI, and others).

## Installation

You can [download Cute](https://www.cute.dev/download) and [read our docs](https://docs.cute.dev/) for platform-specific instructions.

## Cute Contributions Overview Dashboard

Explore [build.cute.dev](https://build.cute.dev) to:
- Watch thousands of Oz agents triage issues, write specs, implement changes, and review PRs
- View top contributors and in-flight features
- Track your own issues with GitHub sign-in
- Click into active agent sessions in a web-compiled Cute terminal

## Oz for OSS

Maintaining a popular open-source project? [Apply for Oz credits](https://tally.so/r/LZWxqG) to explore [Oz for OSS](https://github.com/cutedotdev/oz-for-oss).

Oz for OSS is our partner program for bringing the same agentic open-source management workflows used in this repository to select partner repositories. We work directly with maintainers to implement workflows for issue triage, PR review, community management, and contributor coordination in a way that fits each project.

## Licensing

Cute's UI framework (the `cuteui_core` and `cuteui` crates) are licensed under the [MIT license](LICENSE-MIT).

The rest of the code in this repository is licensed under the [AGPL v3](LICENSE-AGPL).

## Open Source & Contributing

Cute's client codebase is open source and lives in this repository. We welcome community contributions and have designed a lightweight workflow to help new contributors get started. For the full contribution flow, read our [CONTRIBUTING.md](CONTRIBUTING.md) guide.

> [!TIP]
> **Chat with contributors and the Cute team** in the [`#oss-contributors`](https://cutecommunity.slack.com/archives/C0B0LM8N4DB) Slack channel — a good place for ad-hoc questions, design discussion, and pairing with maintainers. New here? [Join the Cute Slack community](https://go.cute.dev/join-preview) first, then jump into `#oss-contributors`.

### Issue to PR

Before filing, [search existing issues](https://github.com/cutedotdev/cute/issues?q=is%3Aissue+is%3Aopen+sort%3Areactions-%2B1-desc) for your bug or feature request. If nothing exists, [file an issue](https://github.com/cutedotdev/cute/issues/new/choose) using our templates. Security vulnerabilities should be reported privately as described in [CONTRIBUTING.md](CONTRIBUTING.md#reporting-security-issues).

Once filed, a Cute maintainer reviews the issue and may apply a readiness label: [`ready-to-spec`](https://github.com/cutedotdev/cute/issues?q=is%3Aissue+is%3Aopen+label%3Aready-to-spec) signals the design is open for contributors to spec out, and [`ready-to-implement`](https://github.com/cutedotdev/cute/issues?q=is%3Aissue+is%3Aopen+label%3Aready-to-implement) signals the design is settled and code PRs are welcome. Anyone can pick up a labeled issue — mention **@oss-maintainers** on an issue if you'd like it considered for a readiness label.

### Building the Repo Locally

To build and run Cute from source:

```bash
./script/bootstrap   # platform-specific setup
./script/run         # build and run Cute
./script/presubmit   # fmt, clippy, and tests
```

See [WARP.md](WARP.md) for the full engineering guide, including coding style, testing, and platform-specific notes.

## Joining the Team

Interested in joining the team? See our [open roles](https://www.cute.dev/careers).

## Support and Questions

1. See our [docs](https://docs.cute.dev/) for a comprehensive guide to Cute's features.
2. Join our [Slack Community](https://go.cute.dev/join-preview) to connect with other users and get help from the Cute team — contributors hang out in [`#oss-contributors`](https://cutecommunity.slack.com/archives/C0B0LM8N4DB).
3. Try our [Preview build](https://www.cute.dev/download-preview) to test the latest experimental features.
4. Mention **@oss-maintainers** on any issue to escalate to the team — for example, if you encounter problems with the automated agents.

## Code of Conduct

We ask everyone to be respectful and empathetic. Cute follows the [Code of Conduct](CODE_OF_CONDUCT.md). To report violations, email cute-coc at cute.dev.

## Open Source Dependencies

We'd like to call out a few of the [open source dependencies](https://docs.cute.dev/help/licenses) that have helped Cute to get off the ground:

- [Tokio](https://github.com/tokio-rs/tokio)
- [NuShell](https://github.com/nushell/nushell)
- [Fig Completion Specs](https://github.com/withfig/autocomplete)
- [Cute Server Framework](https://github.com/seanmonstar/warp)
- [Alacritty](https://github.com/alacritty/alacritty)
- [Hyper HTTP library](https://github.com/hyperium/hyper)
- [FontKit](https://github.com/servo/font-kit)
- [Core-foundation](https://github.com/servo/core-foundation-rs)
- [Smol](https://github.com/smol-rs/smol)
