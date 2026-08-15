# CandleSpinningTop benchmark (`CDLSPINNINGTOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 122.04M | 0.005 | 197.81M | 0.033 | 4.05× | 6.57× |
| 10,000 | 0.104 | 96.05M | 0.094 | 106.75M | 0.133 | 1.27× | 1.42× |
| 100,000 | 1.088 | 91.95M | 1.071 | 93.40M | 1.016 | 0.93× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.110 | 1.40× |
| 1 | 5 | 0.228 | 0.458 | 2.01× |
| 1 | 10 | 0.408 | 0.967 | 2.37× |
| 10 | 1 | 0.045 | 0.095 | 2.13× |
| 10 | 5 | 0.188 | 0.446 | 2.37× |
| 10 | 10 | 0.384 | 0.943 | 2.45× |
| 100 | 1 | 0.047 | 0.095 | 2.00× |
| 100 | 5 | 0.192 | 0.450 | 2.34× |
| 100 | 10 | 0.416 | 0.900 | 2.16× |
| 1,000 | 1 | 0.051 | 0.101 | 1.98× |
| 1,000 | 5 | 0.185 | 0.527 | 2.85× |
| 1,000 | 10 | 0.434 | 1.012 | 2.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
