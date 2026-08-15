# MathCosh benchmark (`COSH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 137.33M | 0.006 | 162.42M | 0.035 | 4.82× | 5.70× |
| 10,000 | 0.060 | 167.80M | 0.053 | 187.48M | 0.087 | 1.46× | 1.63× |
| 100,000 | 0.551 | 181.37M | 0.556 | 179.76M | 0.559 | 1.01× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.107 | 1.19× |
| 1 | 5 | 0.242 | 0.479 | 1.97× |
| 1 | 10 | 0.370 | 0.924 | 2.50× |
| 10 | 1 | 0.042 | 0.090 | 2.16× |
| 10 | 5 | 0.190 | 0.467 | 2.45× |
| 10 | 10 | 0.396 | 0.862 | 2.18× |
| 100 | 1 | 0.041 | 0.084 | 2.03× |
| 100 | 5 | 0.175 | 0.412 | 2.36× |
| 100 | 10 | 0.385 | 0.967 | 2.51× |
| 1,000 | 1 | 0.051 | 0.099 | 1.94× |
| 1,000 | 5 | 0.196 | 0.444 | 2.27× |
| 1,000 | 10 | 0.415 | 0.927 | 2.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
