<p align="center"><strong>Forestx CLI</strong> is a coding agent from OpenAI that runs locally on your computer.
<p align="center">
  <img src="https://github.com/openai/forestx/blob/main/.github/forestx-cli-splash.png" alt="Forestx CLI splash" width="80%" />
</p>
</br>
If you want Forestx in your code editor (VS Code, Cursor, Windsurf), <a href="https://developers.openai.com/forestx/ide">install in your IDE.</a>
</br>If you want the desktop app experience, run <code>forestx app</code> or visit <a href="https://chatgpt.com/forestx?app-landing-page=true">the Forestx App page</a>.
</br>If you are looking for the <em>cloud-based agent</em> from OpenAI, <strong>Forestx Web</strong>, go to <a href="https://chatgpt.com/forestx">chatgpt.com/forestx</a>.</p>

---

## Quickstart

### Installing and running Forestx CLI

Run the following on Mac or Linux to install Forestx CLI:

```shell
curl -fsSL https://chatgpt.com/forestx/install.sh | sh
```

Run the following on Windows to install Forestx CLI:

```shell
powershell -ExecutionPolicy ByPass -c "irm https://chatgpt.com/forestx/install.ps1 | iex"
```

The standalone installers download from `https://releases.openai.com/forestx` by default and fall back to GitHub Releases if a metadata or asset download is unavailable. To force GitHub Releases, set `FORESTX_INSTALLER_USE_RELEASES_OPENAI_COM` to `false` (`0` and `no` are also accepted):

```shell
curl -fsSL https://chatgpt.com/forestx/install.sh | FORESTX_INSTALLER_USE_RELEASES_OPENAI_COM=false sh
```

```powershell
$env:FORESTX_INSTALLER_USE_RELEASES_OPENAI_COM='false'; irm https://chatgpt.com/forestx/install.ps1 | iex
```

Forestx CLI can also be installed via the following package managers:

```shell
# Install using npm
npm install -g @openai/forestx
```

```shell
# Install using Homebrew
brew install --cask forestx
```

Then simply run `forestx` to get started.

<details>
<summary>You can also go to the <a href="https://github.com/openai/forestx/releases/latest">latest GitHub Release</a> and download the appropriate binary for your platform.</summary>

Each GitHub Release contains many executables, but in practice, you likely want one of these:

- macOS
  - Apple Silicon/arm64: `forestx-aarch64-apple-darwin.tar.gz`
  - x86_64 (older Mac hardware): `forestx-x86_64-apple-darwin.tar.gz`
- Linux
  - x86_64: `forestx-x86_64-unknown-linux-musl.tar.gz`
  - arm64: `forestx-aarch64-unknown-linux-musl.tar.gz`

Each archive contains a single entry with the platform baked into the name (e.g., `forestx-x86_64-unknown-linux-musl`), so you likely want to rename it to `forestx` after extracting it.

</details>

### Using Forestx with your ChatGPT plan

Run `forestx` and select **Sign in with ChatGPT**. We recommend signing into your ChatGPT account to use Forestx as part of your Plus, Pro, Business, Edu, or Enterprise plan. [Learn more about what's included in your ChatGPT plan](https://help.openai.com/en/articles/11369540-forestx-in-chatgpt).

You can also use Forestx with an API key, but this requires [additional setup](https://developers.openai.com/forestx/auth#sign-in-with-an-api-key).

## Docs

- [**Forestx Documentation**](https://developers.openai.com/forestx)
- [**Contributing**](./docs/contributing.md)
- [**Installing & building**](./docs/install.md)
- [**Open source fund**](./docs/open-source-fund.md)

This repository is licensed under the [Apache-2.0 License](LICENSE).
