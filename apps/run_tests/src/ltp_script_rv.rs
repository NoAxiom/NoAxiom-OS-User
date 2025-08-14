#[allow(unused)]
pub const REMOVED: &str = r#####"
dio_sparse
socket01
mtest01
lftest
fcntl34 fcntl34_64
fcntl36 fcntl36_64
"#####;

pub const LTP_SH: &str = r#####"
echo "start to test ltp in musl"
cd /
cd /musl/ltp/testcases/bin



file_list="
# List of LTP test cases to run
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
