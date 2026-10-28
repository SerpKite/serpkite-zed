# SerpKite MCP Server for Zed

This extension adds [SerpKite](https://serpkite.com)'s hosted MCP server to Zed's Agent Panel: Google search (web, news, images, videos, maps, places, shopping, scholar, patents, autocomplete), one place's full details, public webpages, extraction and site crawls as LLM-ready Markdown, plus cited answers from live Google results, async deep research reports, and the answers of ChatGPT, Gemini, Copilot Search, Google AI Mode and Brave with the sources they cited.

It runs [`mcp-remote`](https://www.npmjs.com/package/mcp-remote) to connect to `https://api.serpkite.com/v1/mcp`. Nothing is bundled.

## Setup

1. Install **SerpKite MCP Server** from Zed's extensions page.
2. Choose how to sign in:
   - **OAuth (default):** leave `serpkite_api_key` unset. On first use a browser window opens to sign in to SerpKite and approve Zed.
   - **API key:** create a key at [app.serpkite.com/keys](https://app.serpkite.com/keys) and add it to your settings:

```json
{
  "context_servers": {
    "mcp-server-serpkite": {
      "settings": {
        "serpkite_api_key": "skt_live_..."
      }
    }
  }
}
```

New accounts get 1,000 free credits, plus 1,000 every month. Failed and empty searches are not billed.

## Tools

`search`, `news`, `maps`, `place`, `scholar`, `patents`, `shopping`, `images`, `videos`, `autocomplete`, `chatgpt`, `gemini`, `copilot`, `ai_mode`, `brave_ai`, `webpage`, `answer`, `extract`, `map`, `crawl`, `crawl_result`, `research`, `research_result`. Inputs and costs: [serpkite.com/docs/mcp](https://serpkite.com/docs/mcp).

## License

MIT
