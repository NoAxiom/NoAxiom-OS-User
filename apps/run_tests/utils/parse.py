#!/usr/bin/env python3
# -*- coding: utf-8 -*-

def process_file(input_file, output_file):
    # 用于存储处理后的行
    lines_seen = set()

    with open(input_file, 'r', encoding='utf-8') as f:
        for line in f:
            # 去除行首尾的空白字符
            stripped_line = line.strip()
            # 跳过空行和包含冒号的行
            if not stripped_line or ':' in stripped_line or stripped_line.startswith('#'):
                continue
            # 添加到集合中自动去重
            lines_seen.add(stripped_line)

    # 将集合转换为列表并按字典序排序
    sorted_lines = sorted(lines_seen)

    # 写入输出文件
    with open(output_file, 'w', encoding='utf-8') as f:
        for line in sorted_lines:
            f.write(line + '\n')

    # 同时打印到控制台
    for line in sorted_lines:
        print(line)

if __name__ == '__main__':
    input_filename = "input.txt"
    output_filename = "output.txt"
    process_file(input_filename, output_filename)
    print(f"处理完成，结果已保存到 {output_filename}")
