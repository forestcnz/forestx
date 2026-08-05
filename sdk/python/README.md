# OpenAI Forestx Python SDK

Build Python applications that start Forestx threads, run turns, stream progress,
and control workspace access.

## Install

Install the SDK:

```bash
pip install openai-forestx
```

## Quickstart

The SDK reuses your existing Forestx authentication when one is already
available:

```python
from openai_forestx import Forestx

with Forestx() as forestx:
    thread = forestx.thread_start()
    result = thread.run("Explain this repository in three bullets.")
    print(result.final_response)
```

`thread.run(...)` returns a `TurnResult` containing the final response,
collected items, and token usage.

## Authentication

Existing Forestx authentication is reused automatically. To start ChatGPT
browser login explicitly:

```python
from openai_forestx import Forestx

with Forestx() as forestx:
    login = forestx.login_chatgpt()
    print(login.auth_url)
    print(login.wait().success)
```

For device-code login:

```python
with Forestx() as forestx:
    login = forestx.login_chatgpt_device_code()
    print(login.verification_url, login.user_code)
    login.wait()
```

For API-key login:

```python
with Forestx() as forestx:
    forestx.login_api_key("sk-...")
```

## Built-In Help

Use Python's standard `help(openai_forestx)`, `help(Forestx)`, or
`python -m pydoc openai_forestx` documentation tools.

## Documentation

- [Getting started](https://github.com/openai/forestx/blob/main/sdk/python/docs/getting-started.md)
- [API reference](https://github.com/openai/forestx/blob/main/sdk/python/docs/api-reference.md)
- [FAQ](https://github.com/openai/forestx/blob/main/sdk/python/docs/faq.md)
- [Examples](https://github.com/openai/forestx/blob/main/sdk/python/examples/README.md)

The package is licensed under the
[repository Apache License 2.0](https://github.com/openai/forestx/blob/main/LICENSE).
