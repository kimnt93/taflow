# CandleStalledPattern benchmark (`CDLSTALLEDPATTERN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 61.52M | 0.013 | 75.62M | 0.046 | 2.80× | 3.45× |
| 10,000 | 0.156 | 64.28M | 0.158 | 63.23M | 0.166 | 1.07× | 1.05× |
| 100,000 | 1.802 | 55.50M | 1.578 | 63.35M | 1.312 | 0.73× | 0.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.108 | 1.26× |
| 1 | 5 | 0.246 | 0.500 | 2.04× |
| 1 | 10 | 0.415 | 0.917 | 2.21× |
| 10 | 1 | 0.045 | 0.091 | 2.04× |
| 10 | 5 | 0.188 | 0.442 | 2.35× |
| 10 | 10 | 0.421 | 0.926 | 2.20× |
| 100 | 1 | 0.047 | 0.092 | 1.96× |
| 100 | 5 | 0.196 | 0.444 | 2.27× |
| 100 | 10 | 0.389 | 1.008 | 2.59× |
| 1,000 | 1 | 0.064 | 0.107 | 1.68× |
| 1,000 | 5 | 0.201 | 0.501 | 2.49× |
| 1,000 | 10 | 0.431 | 1.042 | 2.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
