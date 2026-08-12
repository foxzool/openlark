'use strict';

const path = require('path');
const readline = require('readline');
const { execFileSync } = require('child_process');

let browser;
let englishContext;

function loadChromium() {
  try {
    return require('playwright').chromium;
  } catch (localError) {
    try {
      const globalRoot = execFileSync('npm', ['root', '-g'], {
        encoding: 'utf8',
        stdio: ['ignore', 'pipe', 'ignore'],
      }).trim();
      return require(path.join(globalRoot, 'playwright')).chromium;
    } catch (globalError) {
      return null;
    }
  }
}

async function ensureBrowser() {
  if (browser) {
    return browser;
  }
  const chromium = loadChromium();
  if (!chromium) {
    return null;
  }
  browser = await chromium.launch({ headless: true });
  return browser;
}

async function ensureEnglishContext() {
  const activeBrowser = await ensureBrowser();
  if (!activeBrowser) {
    return null;
  }
  if (!englishContext) {
    // 固定英文 locale，避免飞书按地区渲染中文段标题导致解析归零
    englishContext = await activeBrowser.newContext({
      locale: 'en-US',
      extraHTTPHeaders: {
        'Accept-Language': 'en-US,en',
      },
    });
  }
  return englishContext;
}

async function render(request) {
  const context = await ensureEnglishContext();
  if (!context) {
    return {
      id: request.id,
      status: 'unavailable',
      code: 'adapter_unavailable',
    };
  }

  const page = await context.newPage();
  try {
    const timeout = Math.max(1, request.timeout_ms);
    await page.goto(request.url, {
      waitUntil: 'networkidle',
      timeout: Math.max(1, Math.floor(timeout * 0.7)),
    });
    const remaining = Math.max(0, timeout - Math.floor(timeout * 0.7));
    await page.waitForTimeout(Math.min(2000, Math.floor(remaining / 2)));
    for (let index = 0; index < 4; index += 1) {
      await page.evaluate(() => window.scrollTo(0, document.body.scrollHeight));
      await page.waitForTimeout(Math.min(300, Math.floor(remaining / 10)));
    }
    await page.evaluate(() => window.scrollTo(0, 0));
    const content = await page.evaluate(() => document.body.innerText);
    return {
      id: request.id,
      status: 'ok',
      source_uri: page.url(),
      content,
    };
  } catch (error) {
    return {
      id: request.id,
      status: 'unavailable',
      code: error && error.name === 'TimeoutError'
        ? 'acquisition_timeout'
        : 'acquisition_failed',
    };
  } finally {
    await page.close();
  }
}

async function shutdown() {
  if (englishContext) {
    await englishContext.close();
    englishContext = undefined;
  }
  if (browser) {
    await browser.close();
    browser = undefined;
  }
}

const input = readline.createInterface({ input: process.stdin });
let queue = Promise.resolve();

input.on('line', (line) => {
  queue = queue.then(async () => {
    let request;
    try {
      request = JSON.parse(line);
    } catch (error) {
      process.stdout.write('{"status":"contract_error"}\n');
      return;
    }
    if (request.type === 'shutdown') {
      await shutdown();
      input.close();
      return;
    }
    if (
      !Number.isInteger(request.id)
      || typeof request.url !== 'string'
      || !/^https?:\/\//i.test(request.url)
      || !Number.isInteger(request.timeout_ms)
      || request.timeout_ms <= 0
    ) {
      process.stdout.write(
        `${JSON.stringify({ id: request.id, status: 'contract_error' })}\n`
      );
      return;
    }
    const response = await render(request);
    process.stdout.write(`${JSON.stringify(response)}\n`);
  }).catch(async () => {
    await shutdown();
    process.exitCode = 1;
    input.close();
  });
});

input.on('close', () => {
  queue = queue.then(shutdown);
});
