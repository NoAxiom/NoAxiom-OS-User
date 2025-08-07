#[allow(unused)]
pub const BUG: &str = "
futex_wait01
futex_wait02
futex_wait03
futex_wait04
futex_wait05
futex_wait_bitset01
futex_wake01
futex_wake03
select04
sendfile07
sendfile07_64
writev01
thp01
times03
";
// wait系的都有bug，包括未列于上的waitpid系列大部分有bug

pub const GIVE_UP: &str = "
accept02
af_alg01
af_alg02
af_alg03
af_alg05
bind01
bind02
bind03
";
// net系：需要实现很多新东西，alg系、AF_UNIX地址系

pub const LTP_SH: &str = r#####"
echo "start to test ltp in musl"
cd /
cd /musl/ltp/testcases/bin



file_list="
access01
access02
access03
accept01
accept03
accept4_01
chdir01
chdir04
chmod01
chown02
chmod03
chmod05
chmod07
chown01
chown03
chown05
chroot01
chroot02
chroot03
chroot04
"
set -- $file_list

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
