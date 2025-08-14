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

getpid02
getrlimit03   !!!
getrusage01
getsid02
madvise02
madvise10
ppoll01
request_key02


DONE:
clock_settime01
clock_settime02
clock_nanosleep04

TODO:
clock_settime03
clock_nanosleep01
clone03
clone08
clone301
clone302
execve02
execve03
futex_cmp_requeue01
futex_cmp_requeue02
getitimer01
setitimer02
sbrk01
sched_getscheduler01

shmctl07
shmem_2nstest
shmnstest

sigpending02
sigsuspend01
sigwait

tgkill01
tgkill02
tgkill03
tkill02

waitid05
waitpid11
waitpid12
waitpid13

mmap12
mmap13
mmap15
";
// net系：需要实现很多新东西，alg系、AF_UNIX地址系

pub const LTP_SH: &str = r#####"
echo "start to test ltp in musl"
cd /
cd /musl/ltp/testcases/bin



file_list="
# List of LTP test cases to run
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
