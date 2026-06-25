#!/usr/bin/env node
/**
 * 飞书开放平台文档抓取脚本（playwright 渲染版）
 *
 * 解决问题：飞书文档是 SPA，fetch_docpath.py 和直接 HTTP 都抓不到字段表。
 * 本脚本用 playwright 真实渲染页面，等待 JS 执行后导出完整 innerText。
 *
 * 依赖：playwright + chromium。首次用前跑：npx playwright install chromium
 *
 * 用法：
 *   单页：node fetch_doc.js <完整URL> <输出文件>
 *   批量：node fetch_doc.js --batch <path1> <path2> ... --out-dir <目录> [--base <基URL>]
 *
 *   path 是 fullPath 去掉前导 /document 的部分，如 approval-v4/task/pass
 *   脚本会自动拼成 https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/<path>
 *
 * 示例：
 *   node fetch_doc.js \
 *     "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/pass" \
 *     /tmp/doc_pass.txt
 *
 *   node fetch_doc.js --batch approval-v4/instance/add_cc approval-v4/task/pass --out-dir /tmp/docs
 */

const fs = require('fs');
const path = require('path');

// 尝试加载 playwright（不依赖个人本机绝对路径）
// 1) 优先用当前 NODE_PATH / 本地已装的 playwright
// 2) 失败则查 npm 全局 node_modules 根（等价于 npx 全局缓存解析到的位置）
// 这样在任何机器上都能跑，不写死作者本机的 npx 缓存路径。
let chromium;
try {
  ({ chromium } = require('playwright'));
} catch (e) {
  let resolved = null;
  try {
    const { execSync } = require('child_process');
    const globalRoot = execSync('npm root -g', { encoding: 'utf8' }).trim();
    resolved = require('path').join(globalRoot, 'playwright');
    ({ chromium } = require(resolved));
  } catch (e2) {
    resolved = null;
  }
  if (!chromium) {
    console.error('❌ 找不到 playwright 模块。请先安装：npm i -g playwright && npx playwright install chromium');
    process.exit(1);
  }
}

const DOC_BASE = 'https://open.feishu.cn';
// reference 类文档的公共前缀
const REFERENCE_PREFIX = '/document/uAjLw4CM/ukTMukTMukTM/reference/';

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
    fs.writeFileSync(outFile, text, 'utf-8');

    // 简单健康检查：内容太少说明可能 URL 错或没渲染
    const flag = text.length < 500 ? '⚠️ 内容过少，检查 URL 是否正确' : '✅';
    console.log(`${flag} ${path.basename(outFile)}: ${text.length} chars`);
    return text.length;
  } finally {
    await page.close();
  }
}

async function main() {
  const args = process.argv.slice(2);

  if (args[0] === '--batch') {
    // 批量模式
    const outDirIdx = args.indexOf('--out-dir');
    const baseIdx = args.indexOf('--base');
    const outDir = outDirIdx >= 0 ? args[outDirIdx + 1] : '/tmp/feishu-docs';
    const basePath = baseIdx >= 0 ? args[baseIdx + 1] : DOC_BASE + REFERENCE_PREFIX;

    const paths = args.slice(1).filter(
      (a, i) => a !== '--out-dir' && a !== '--base' && args[i + 1] !== '--out-dir' && args[i + 1] !== '--base' && !a.startsWith('--')
    );

    if (paths.length === 0) {
      console.error('用法: node fetch_doc.js --batch <path1> <path2> ... --out-dir <目录>');
      process.exit(1);
    }

    fs.mkdirSync(outDir, { recursive: true });
    const browser = await chromium.launch({ headless: true });
    console.log(`📥 批量抓取 ${paths.length} 个文档到 ${outDir}`);
    for (const p of paths) {
      const url = basePath + p;
      const name = p.replace(/\//g, '_');
      const outFile = path.join(outDir, `doc_${name}.txt`);
      try {
        await fetchOne(browser, url, outFile);
      } catch (e) {
        console.log(`❌ ${p}: ${e.message}`);
      }
    }
    await browser.close();
    console.log('✅ 批量完成');
  } else {
    // 单页模式
    const [url, outFile] = args;
    if (!url || !outFile) {
      console.error('用法: node fetch_doc.js <完整URL> <输出文件>');
      process.exit(1);
    }
    const browser = await chromium.launch({ headless: true });
    try {
      await fetchOne(browser, url, outFile);
    } finally {
      await browser.close();
    }
  }
}

main().catch((e) => {
  console.error('❌', e.message);
  process.exit(1);
});
