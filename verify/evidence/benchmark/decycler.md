# Decycler benchmark (`Decycler` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 160.91M | 0.005 | 187.71M | 0.156 | 25.18× | 29.37× |
| 10,000 | 0.048 | 206.40M | 0.047 | 210.69M | 0.488 | 10.06× | 10.27× |
| 100,000 | 0.458 | 218.23M | 0.442 | 226.49M | 4.079 | 8.90× | 9.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.138 | 0.231 | 1.67× |
| 1 | 5 | 0.257 | 0.936 | 3.64× |
| 1 | 10 | 0.398 | 2.525 | 6.34× |
| 10 | 1 | 0.049 | 0.188 | 3.83× |
| 10 | 5 | 0.189 | 0.928 | 4.90× |
| 10 | 10 | 0.395 | 2.091 | 5.29× |
| 100 | 1 | 0.045 | 0.192 | 4.26× |
| 100 | 5 | 0.196 | 0.936 | 4.79× |
| 100 | 10 | 0.397 | 2.216 | 5.59× |
| 1,000 | 1 | 0.048 | 0.220 | 4.58× |
| 1,000 | 5 | 0.206 | 1.119 | 5.44× |
| 1,000 | 10 | 0.458 | 2.518 | 5.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
