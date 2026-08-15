# VolumeOscillator benchmark (`VolumeOscillator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 74.78M | 0.011 | 87.76M | 0.205 | 15.34× | 18.00× |
| 10,000 | 0.116 | 86.31M | 0.103 | 96.66M | 0.542 | 4.68× | 5.24× |
| 100,000 | 1.201 | 83.26M | 1.043 | 95.85M | 4.112 | 3.42× | 3.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.283 | 3.09× |
| 1 | 5 | 0.255 | 1.384 | 5.42× |
| 1 | 10 | 0.404 | 2.601 | 6.44× |
| 10 | 1 | 0.046 | 0.241 | 5.19× |
| 10 | 5 | 0.214 | 1.390 | 6.50× |
| 10 | 10 | 0.401 | 2.531 | 6.31× |
| 100 | 1 | 0.048 | 0.242 | 5.08× |
| 100 | 5 | 0.206 | 1.413 | 6.86× |
| 100 | 10 | 0.430 | 2.769 | 6.44× |
| 1,000 | 1 | 0.056 | 0.286 | 5.06× |
| 1,000 | 5 | 0.216 | 1.631 | 7.55× |
| 1,000 | 10 | 0.461 | 2.933 | 6.36× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
