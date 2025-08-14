#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os

def get_input_file():
    input_files = ["input.txt", "full.txt"]  # 尝试的文件列表，按优先级排序
    for file in input_files:
        if os.path.exists(file):
            return file
    raise FileNotFoundError(f"未找到输入文件，尝试了: {', '.join(input_files)}")

def process_file(output_file):
    # 用于存储处理后的行
    lines_seen = set()

    input_file = get_input_file()
    print(f"使用输入文件: {input_file}")

    try:
        with open(input_file, 'r', encoding='utf-8') as f:
            for line in f:
                # 去除行首尾的空白字符
                stripped_line = line.strip()
                # 跳过空行和包含冒号的行
                if not stripped_line or ':' in stripped_line or stripped_line.startswith('#'):
                    continue
                # 添加到集合中自动去重
                lines_seen.add(stripped_line)
    except IOError as e:
        raise IOError(f"无法读取文件 {input_file}: {e}")

    # 将集合转换为列表并按字典序排序
    sorted_lines = sorted(lines_seen)

    # 写入输出文件
    with open(output_file, 'w', encoding='utf-8') as f:
        for line in sorted_lines:
            f.write(line + '\n')

    # 生成../src/ltp_full.rs文件
    generate_rs_file(sorted_lines)

def generate_rs_file(test_cases):
    # 使用原始字符串(r-string)避免转义问题
    rs_template = r'''pub const LTP_SH: &str = r#####"
echo "start to test ltp in musl"
cd /
cd /musl/ltp/testcases/bin


file_list="
{test_cases_list}
"
set -- $file_list

echo "start to test ltp in musl"
cd /
cd /musl/ltp/testcases/bin

echo "#### OS COMP TEST GROUP START ltp-musl ####"

for file in $@; do
  # 跳过目录，仅处理文件
  if [ -f "$file" ]; then
    # 输出文件名
    echo "RUN LTP CASE $(basename "$file")"

    "./$file"
    ret=$?

    # 输出文件名和返回值
    echo "FAIL LTP CASE $(basename "$file") : $ret"
  fi
done


echo "#### OS COMP TEST GROUP END ltp-musl ####"

echo "start to test ltp in glibc"
cd /
cd /glibc/ltp/testcases/bin


echo "#### OS COMP TEST GROUP START ltp-glibc ####"

for file in $@; do
  if [ -f "$file" ]; then
    echo "RUN LTP CASE $(basename "$file")"

    "./$file"
    ret=$?

    echo "FAIL LTP CASE $(basename "$file") : $ret"
  fi
done


echo "#### OS COMP TEST GROUP END ltp-glibc ####"
"#####;
'''

    # 使用format方法而不是f-string来避免转义问题
    rs_content = rs_template.format(test_cases_list='\n'.join(test_cases))

    rs_file_path = "../src/ltp_full.rs"
    os.makedirs(os.path.dirname(rs_file_path), exist_ok=True)  # 确保目录存在
    with open(rs_file_path, 'w', encoding='utf-8') as f:
        f.write(rs_content)
    print(f"Rust源文件已生成到 {rs_file_path}")

if __name__ == '__main__':
    try:
        output_filename = "output.txt"
        process_file(output_filename)
        print(f"处理完成，结果已保存到 {output_filename}")
    except Exception as e:
        print(f"错误: {e}")
        exit(1)
