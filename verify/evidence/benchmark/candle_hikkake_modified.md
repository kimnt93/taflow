# CandleHikkakeModified benchmark (`CDLHIKKAKEMOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 51.39M | 0.020 | 49.66M | 0.052 | 2.67× | 2.58× |
| 10,000 | 0.071 | 140.93M | 0.067 | 148.36M | 0.096 | 1.36× | 1.43× |
| 100,000 | 0.651 | 153.58M | 0.620 | 161.36M | 0.647 | 0.99× | 1.04× |
| 1,000,000 | 6.739 | 148.39M | 6.384 | 156.65M | 6.090 | 0.90× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.169 | 0.130 | 0.77× |
| 1 | 5 | 0.319 | 0.532 | 1.67× |
| 1 | 10 | 0.583 | 0.968 | 1.66× |
| 10 | 1 | 0.069 | 0.102 | 1.49× |
| 10 | 5 | 0.339 | 0.520 | 1.53× |
| 10 | 10 | 0.607 | 0.971 | 1.60× |
| 100 | 1 | 0.062 | 0.106 | 1.72× |
| 100 | 5 | 0.318 | 0.556 | 1.75× |
| 100 | 10 | 0.635 | 0.975 | 1.53× |
| 1,000 | 1 | 0.073 | 0.094 | 1.28× |
| 1,000 | 5 | 0.269 | 0.547 | 2.03× |
| 1,000 | 10 | 0.693 | 1.054 | 1.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
