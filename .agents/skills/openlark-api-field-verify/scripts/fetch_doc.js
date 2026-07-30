#!/usr/bin/env node
/**
 * 飞书开放平台文档抓取脚本（playwright 渲染版）
 *
 * 解决问题：飞书文档是 SPA，fetch_docpath.py 和直接 HTTP 都抓不到字段表。
 * 本脚本用 playwright 真实渲染页面，等待 JS 执行后导出完整 innerText。
 *
 * 依赖：playwright + chromium。首次用前跑：npx playwright install chromium
 *
 * URL 权威源：api_list_export.csv 的 fullPath。
 *   canonical = https://open.feishu.cn + fullPath
 * 禁止手拼 /reference/ 或 /server-docs/ 前缀；不要默认用 docPath。
 *
 * 用法：
 *   单页：node fetch_doc.js <完整URL|fullPath> <输出文件>
 *   CSV： node fetch_doc.js --from-csv <api_id> [--out <文件>] [--csv <路径>]
 *   批量：node fetch_doc.js --batch <fullPath|URL>... --out-dir <目录>
 *
 * 示例：
 *   node fetch_doc.js \
 *     "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/pass" \
 *     /tmp/doc_pass.txt
 *
 *   node fetch_doc.js --from-csv 7642253323628383198 --out /tmp/doc_pass.txt
 *
 *   node fetch_doc.js --batch \
 *     /document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/pass \
 *     --out-dir /tmp/docs
 */

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const DOC_BASE = 'https://open.feishu.cn';

/** 懒加载 playwright，便于 --help / CSV 解析在未安装时也能跑。 */
function loadChromium() {
  try {
    return require('playwright').chromium;
  } catch (e) {
    try {
      const globalRoot = execSync('npm root -g', { encoding: 'utf8' }).trim();
      return require(path.join(globalRoot, 'playwright')).chromium;
    } catch (e2) {
      console.error(
        '❌ 找不到 playwright 模块。请先安装：npm i -g playwright && npx playwright install chromium'
      );
      process.exit(1);
    }
  }
}

/**
 * 将输入解析为完整文档 URL。
 * - 已是 http(s) URL → 原样
 * - 以 / 开头的 fullPath → DOC_BASE + path
 * - 其他 → 报错（禁止短 path 自动拼 /reference/）
 */
function resolveUrl(input) {
  const s = (input || '').trim();
  if (!s) {
    throw new Error('空 URL/fullPath');
  }
  if (/^https?:\/\//i.test(s)) {
    return s;
  }
  if (s.startsWith('/')) {
    return DOC_BASE + s;
  }
  throw new Error(
    `非法路径 "${s}"：请传完整 URL 或 CSV fullPath（以 / 开头）。` +
      '禁止传短 path（如 approval-v4/task/pass）——旧用法会错误拼到 /reference/ 下。'
  );
}

/**
 * 从 api_list_export.csv 按 id 取 fullPath。
 */
function fullPathFromCsv(apiId, csvPath) {
  const text = fs.readFileSync(csvPath, 'utf8');
  // 去掉 UTF-8 BOM
  const raw = text.charCodeAt(0) === 0xfeff ? text.slice(1) : text;
  const lines = raw.split(/\r?\n/).filter((l) => l.length > 0);
  if (lines.length < 2) {
    throw new Error(`CSV 无数据: ${csvPath}`);
  }
  const headers = parseCsvLine(lines[0]);
  const idIdx = headers.indexOf('id');
  const fpIdx = headers.indexOf('fullPath');
  if (idIdx < 0 || fpIdx < 0) {
    throw new Error('CSV 缺少 id 或 fullPath 列');
  }
  for (let i = 1; i < lines.length; i++) {
    const cols = parseCsvLine(lines[i]);
    if (cols[idIdx] === apiId) {
      const fp = cols[fpIdx];
      if (!fp) {
        throw new Error(`api_id=${apiId} 的 fullPath 为空`);
      }
      return fp;
    }
  }
  throw new Error(`CSV 中找不到 id=${apiId}`);
}

/** 简易 CSV 行解析（支持引号字段）。 */
function parseCsvLine(line) {
  const out = [];
  let cur = '';
  let inQuotes = false;
  for (let i = 0; i < line.length; i++) {
    const ch = line[i];
    if (inQuotes) {
      if (ch === '"') {
        if (line[i + 1] === '"') {
          cur += '"';
          i++;
        } else {
          inQuotes = false;
        }
      } else {
        cur += ch;
      }
    } else if (ch === '"') {
      inQuotes = true;
    } else if (ch === ',') {
      out.push(cur);
      cur = '';
    } else {
      cur += ch;
    }
  }
  out.push(cur);
  return out;
}

function defaultCsvPath() {
  // scripts → openlark-api-field-verify → skills → .agents → repo root
  return path.resolve(__dirname, '../../../../api_list_export.csv');
}

function outNameFromInput(input) {
  const s = input.replace(/^https?:\/\/[^/]+/i, '');
  return 'doc_' + s.replace(/^\//, '').replace(/\//g, '_') + '.txt';
}

/**
 * 渲染单个文档页面，导出 innerText
 * @param {import('playwright').Browser} browser
 * @param {string} url 完整 URL
 * @param {string} outFile 输出文件路径
 */
async function fetchOne(browser, url, outFile) {
  const page = await browser.newPage();
  try {
    await page.goto(url, { waitUntil: 'networkidle', timeout: 60000 });
    // SPA 需要额外等待内容渲染
    await page.waitForTimeout(3500);
    // 滚动到底触发懒加载（代码块/折叠区常懒加载）
    for (let i = 0; i < 4; i++) {
      await page.evaluate(() => window.scrollTo(0, document.body.scrollHeight));
      await page.waitForTimeout(700);
    }
    await page.evaluate(() => window.scrollTo(0, 0));
    await page.waitForTimeout(500);

    const text = await page.evaluate(() => document.body.innerText);
    fs.mkdirSync(path.dirname(path.resolve(outFile)), { recursive: true });
    fs.writeFileSync(outFile, text, 'utf-8');

    // 简单健康检查：内容太少说明可能 URL 错或没渲染
    const flag = text.length < 500 ? '⚠️ 内容过少，检查 URL 是否正确' : '✅';
    console.log(`${flag} ${path.basename(outFile)}: ${text.length} chars`);
    console.log(`   url: ${url}`);
    return text.length;
  } finally {
    await page.close();
  }
}

function printUsage() {
  console.error(`用法:
  node fetch_doc.js <完整URL|fullPath> <输出文件>
  node fetch_doc.js --from-csv <api_id> [--out <文件>] [--csv <路径>]
  node fetch_doc.js --batch <fullPath|URL>... --out-dir <目录>`);
}

async function main() {
  const args = process.argv.slice(2);

  if (args.length === 0 || args[0] === '-h' || args[0] === '--help') {
    printUsage();
    process.exit(args.length === 0 ? 1 : 0);
  }

  if (args[0] === '--from-csv') {
    const apiId = args[1];
    if (!apiId || apiId.startsWith('--')) {
      console.error('❌ --from-csv 需要 <api_id>');
      printUsage();
      process.exit(1);
    }
    const outIdx = args.indexOf('--out');
    const csvIdx = args.indexOf('--csv');
    const csvPath = csvIdx >= 0 ? args[csvIdx + 1] : defaultCsvPath();
    const outFile =
      outIdx >= 0 ? args[outIdx + 1] : path.join('/tmp', `doc_${apiId}.txt`);

    const fullPath = fullPathFromCsv(apiId, csvPath);
    const url = resolveUrl(fullPath);
    console.log(`📄 api_id=${apiId} fullPath=${fullPath}`);

    const chromium = loadChromium();
    const browser = await chromium.launch({ headless: true });
    try {
      await fetchOne(browser, url, outFile);
    } finally {
      await browser.close();
    }
    return;
  }

  if (args[0] === '--batch') {
    const outDirIdx = args.indexOf('--out-dir');
    const outDir = outDirIdx >= 0 ? args[outDirIdx + 1] : '/tmp/feishu-docs';

    const paths = [];
    for (let i = 1; i < args.length; i++) {
      const a = args[i];
      if (a === '--out-dir') {
        i++; // skip value
        continue;
      }
      if (a.startsWith('--')) {
        console.error(`❌ 未知选项: ${a}`);
        printUsage();
        process.exit(1);
      }
      paths.push(a);
    }

    if (paths.length === 0) {
      console.error('❌ --batch 需要至少一个 fullPath 或完整 URL');
      printUsage();
      process.exit(1);
    }

    fs.mkdirSync(outDir, { recursive: true });
    const chromium = loadChromium();
    const browser = await chromium.launch({ headless: true });
    console.log(`📥 批量抓取 ${paths.length} 个文档到 ${outDir}`);
    for (const p of paths) {
      let url;
      try {
        url = resolveUrl(p);
      } catch (e) {
        console.log(`❌ ${p}: ${e.message}`);
        continue;
      }
      const outFile = path.join(outDir, outNameFromInput(p));
      try {
        await fetchOne(browser, url, outFile);
      } catch (e) {
        console.log(`❌ ${p}: ${e.message}`);
      }
    }
    await browser.close();
    console.log('✅ 批量完成');
    return;
  }

  // 单页模式
  const [input, outFile] = args;
  if (!input || !outFile || input.startsWith('--')) {
    printUsage();
    process.exit(1);
  }
  let url;
  try {
    url = resolveUrl(input);
  } catch (e) {
    console.error(`❌ ${e.message}`);
    process.exit(1);
  }
  const chromium = loadChromium();
  const browser = await chromium.launch({ headless: true });
  try {
    await fetchOne(browser, url, outFile);
  } finally {
    await browser.close();
  }
}

main().catch((e) => {
  console.error('❌', e.message);
  process.exit(1);
});
