cross build --profile release-with-debug --features tikv-jemallocator/profiling --target x86_64-unknown-linux-musl

ln -s 'prof:true,lg_prof_interval:10,lg_prof_sample:5' /etc/_rjem_malloc.conf

jeprof <bin> <jeprof.heap>

这种形式，<bin>必须处于运行时的路径下？不然jeprof不能识别符号？

查看heap文件的内容，里边有bin的绝对路径，把绝对路径改了，好像就可以指定不同位置的bin文件了
