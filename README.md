The code is the repository accompanies the [Target Feature vs Target CPU](http://www.nickwilcox.com/blog/target_cpu_vs_target_feature/) post on my blog.

This code will only run on CPU's with AVX2 support due to the manually written AVX2 function which does no target checking.

To benchmark the version that is not auto-vectorized use
```
RUSTFLAGS="-C target-feature=+avx2" cargo bench
```

To benchmark the version that is auto-vectorized with `VPGATHERQD`
```
RUSTFLAGS="-C target-cpu=skylake" cargo bench
```
