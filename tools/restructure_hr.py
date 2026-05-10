#!/usr/bin/env python3
"""
OpenLark HR 模块自动化重构脚本

根据 api_list_export.csv 生成标准化的目录结构和 API 文件骨架。
目录结构规范: src/{bizTag}/{project}/{version}/{resource}/{name}.rs
"""

import csv
import os
import sys
from pathlib import Path
from typing import List, Dict, Tuple

# HR 相关的 bizTag
HR_BIZ_TAGS = {
    'hire', 'feishu_people', 'attendance', 'compensation_management',
    'performance', 'payroll', 'okr', 'ehr'
}

# 项目根目录
PROJECT_ROOT = Path(__file__).parent.parent
HR_SRC_DIR = PROJECT_ROOT / "crates" / "openlark-hr" / "src"

# 保留的目录（不生成这些目录下的文件）
PROTECTED_DIRS = {'common', 'security', 'endpoints'}


def to_pascal_case(snake_str: str) -> str:
    """将 snake_case 转换为 PascalCase"""
    return ''.join(word.capitalize() for word in snake_str.split('_'))


def generate_api_file_content(biz_tag: str, project: str, version: str,
                               resource: str, name: str, api_name: str) -> str:
    """生成 API 文件的内容"""
    # 将 name 转换为 PascalCase 作为请求类型名
    request_name = to_pascal_case(name) + "Request"
    response_name = to_pascal_case(name) + "Response"

    # 构建 docPath 注释
    doc_path = f"https://open.feishu.cn/document/server-docs/{project}-{version}/{resource}/{name}"

    content = f'''//! {api_name}
//!
//! docPath: {doc_path}

use openlark_core::{{
    api::{{ApiRequest, ApiResponseTrait, ResponseFormat}},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
}};
use serde::{{Deserialize, Serialize}};
use serde_json::Value;

/// {api_name}请求
#[derive(Debug, Clone)]
pub struct {request_name} {{
    /// 配置信息
    config: Config,
    // TEMPLATE: 添加请求字段
}}

impl {request_name} {{
    /// 创建请求
    pub fn new(config: Config) -> Self {{
        Self {{
            config,
            // TEMPLATE: 初始化字段
        }}
    }}

    // TEMPLATE: 添加字段 setter 方法

    /// 执行请求
    pub async fn execute(self) -> SDKResult<{response_name}> {{
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }}

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<{response_name}> {{
        // TEMPLATE: 实现 API 调用逻辑
        todo!("实现 {api_name} API 调用")
    }}
}}

/// {api_name}响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct {response_name} {{
    /// 响应数据
    ///
    /// TEMPLATE: 根据官方文档添加具体字段
    pub data: Value,
}}

impl ApiResponseTrait for {response_name} {{
    fn data_format() -> ResponseFormat {{
        ResponseFormat::Data
    }}
}}
'''
    return content


def generate_mod_rs_content(dirs: List[str], files: List[str]) -> str:
    """生成 mod.rs 的内容"""
    lines = []

    # 导出子模块
    for dir_name in sorted(dirs):
        lines.append(f"pub mod {dir_name};")

    # 导出文件模块
    for file_name in sorted(files):
        lines.append(f"pub mod {file_name};")

    if not lines:
        lines.append("// TEMPLATE: 添加子模块和 API 导出")

    return '\n'.join(lines) + '\n'


def parse_csv(csv_path: str) -> List[Dict]:
    """解析 CSV 文件，返回 HR 相关的 API 列表"""
    hr_apis = []

    with open(csv_path, 'r', encoding='utf-8') as f:
        reader = csv.reader(f)
        header = next(reader)  # 跳过表头

        for row in reader:
            if len(row) < 7:
                continue

            # CSV 列: id, name, bizTag, meta.Project, meta.Version, meta.Resource, meta.Name
            api_id = row[0]
            api_name = row[1]
            biz_tag = row[2]
            project = row[3]
            version = row[4]
            resource = row[5]
            name = row[6]

            # 只处理 HR 相关的 API
            if biz_tag in HR_BIZ_TAGS:
                hr_apis.append({
                    'id': api_id,
                    'name': api_name,
                    'biz_tag': biz_tag,
                    'project': project,
                    'version': version,
                    'resource': resource,
                    'api_name': name,
                })

    return hr_apis


def create_directory_structure(api: Dict) -> Tuple[Path, str]:
    """
    根据 API 信息创建目录结构
    返回: (文件路径, 模块路径)
    """
    # 处理资源路径: app.table -> app/table
    resource_parts = api['resource'].split('.')

    # 处理文件名: batch:create -> batch_create
    file_name = api['api_name'].replace(':', '_')

    # 构建目录路径
    path_parts = [
        api['biz_tag'],
        api['project'],
        api['version'],
    ] + resource_parts

    # 完整目录路径
    dir_path = HR_SRC_DIR
    for part in path_parts:
        dir_path = dir_path / part

    # 文件路径
    file_path = dir_path / f"{file_name}.rs"

    # 模块路径 (用于 mod.rs)
    module_path = '::'.join(path_parts + [file_name])

    return file_path, module_path


def ensure_mod_rs_exists(dir_path: Path, created_files: set):
    """确保目录及其所有父目录都有 mod.rs"""
    # 从 HR_SRC_DIR 开始，逐级检查并创建 mod.rs
    current = HR_SRC_DIR

    # 获取相对于 HR_SRC_DIR 的路径部分
    try:
        rel_parts = dir_path.relative_to(HR_SRC_DIR).parts
    except ValueError:
        return

    for part in rel_parts:
        current = current / part

        if current.is_dir():
            mod_rs = current / "mod.rs"
            if not mod_rs.exists() and str(mod_rs) not in created_files:
                # 创建空的 mod.rs，稍后填充内容
                mod_rs.touch()
                created_files.add(str(mod_rs))


def update_mod_rs(dir_path: Path):
    """更新目录的 mod.rs 文件，导出所有子模块和文件"""
    mod_rs = dir_path / "mod.rs"

    if not dir_path.exists():
        return

    # 获取所有子目录和 .rs 文件（排除 mod.rs）
    dirs = []
    files = []

    for item in dir_path.iterdir():
        if item.is_dir() and item.name not in PROTECTED_DIRS:
            dirs.append(item.name)
        elif item.is_file() and item.suffix == '.rs' and item.name != 'mod.rs':
            files.append(item.stem)

    # 生成 mod.rs 内容
    content = generate_mod_rs_content(dirs, files)

    # 写入文件
    with open(mod_rs, 'w', encoding='utf-8') as f:
        f.write(content)


def update_all_mod_rs():
    """递归更新所有 mod.rs 文件"""
    # 从 HR_SRC_DIR 开始，递归更新所有 mod.rs
    for root, dirs, files in os.walk(HR_SRC_DIR):
        root_path = Path(root)

        # 跳过保护目录
        dirs[:] = [d for d in dirs if d not in PROTECTED_DIRS]

        # 更新当前目录的 mod.rs
        update_mod_rs(root_path)


def main():
    """主函数"""
    csv_path = PROJECT_ROOT / "api_list_export.csv"

    if not csv_path.exists():
        print(f"错误: CSV 文件不存在: {csv_path}")
        sys.exit(1)

    print(f"正在读取 CSV 文件: {csv_path}")
    hr_apis = parse_csv(str(csv_path))
    print(f"找到 {len(hr_apis)} 个 HR 相关 API")

    if len(hr_apis) == 0:
        print("没有找到 HR 相关 API，请检查 CSV 文件")
        sys.exit(1)

    # 统计信息
    stats = {
        'created_files': 0,
        'skipped_files': 0,
        'created_dirs': 0,
    }

    # 记录已创建的 mod.rs 文件，避免重复处理
    created_mod_rs = set()

    # 创建目录结构和文件
    for api in hr_apis:
        file_path, _ = create_directory_structure(api)
        dir_path = file_path.parent

        # 创建目录
        if not dir_path.exists():
            dir_path.mkdir(parents=True, exist_ok=True)
            stats['created_dirs'] += 1

        # 确保所有父目录都有 mod.rs
        ensure_mod_rs_exists(dir_path, created_mod_rs)

        # 创建 API 文件（如果不存在）
        if not file_path.exists():
            content = generate_api_file_content(
                api['biz_tag'],
                api['project'],
                api['version'],
                api['resource'],
                api['api_name'],
                api['name']
            )
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            stats['created_files'] += 1
            print(f"  创建: {file_path.relative_to(PROJECT_ROOT)}")
        else:
            stats['skipped_files'] += 1

    # 更新所有 mod.rs 文件
    print("\n正在更新 mod.rs 文件...")
    update_all_mod_rs()

    # 输出统计信息
    print("\n" + "=" * 50)
    print("重构完成!")
    print("=" * 50)
    print(f"HR API 总数: {len(hr_apis)}")
    print(f"新创建文件: {stats['created_files']}")
    print(f"跳过已存在: {stats['skipped_files']}")
    print(f"创建目录数: {stats['created_dirs']}")
    print("=" * 50)

    # 验证示例文件
    example_file = HR_SRC_DIR / "feishu_people" / "corehr" / "v1" / "person" / "create.rs"
    if example_file.exists():
        print(f"\n✓ 示例文件验证通过: {example_file.relative_to(PROJECT_ROOT)}")
    else:
        print(f"\n✗ 示例文件未找到: {example_file.relative_to(PROJECT_ROOT)}")


if __name__ == "__main__":
    main()
