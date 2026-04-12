# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

w_scraper is a Rust web scraper that crawls an e-commerce site (scrapingcourse.com), extracts product information (name, price, images), and writes results to `outputs.jsonl` in JSONL format.

## Build & Run

```bash
cargo build          # compile
cargo run            # build and run the scraper
cargo check          # type-check without building
```

No tests or linter are currently configured.

## Architecture

Single-file application (`src/main.rs`) with three concerns:

- **Crawling**: BFS traversal using a `VecDeque` queue and `HashSet` for visited URLs. Only follows links containing `www.scrapingcourse.com`.
- **Extraction**: Two functions — `get_urls` extracts all `<a href>` links from a page, `get_product_info` scrapes product details (images from `data-thumb` attributes, price from `.woocommerce-Price-amount bdi`, name from `<title>` tag split on `–`). URLs matching `/product/` are treated as product pages.
- **Output**: `append_jsonl` appends `ProductInfo` structs as JSON lines to `outputs.jsonl`. Only products with at least one image are written.

Uses `reqwest::blocking` for synchronous HTTP and `scraper` for HTML parsing.
