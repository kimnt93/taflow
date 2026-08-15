# CandleThrusting benchmark (`CDLTHRUSTING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 153.35M | 0.003 | 325.16M | 0.033 | 5.03× | 10.66× |
| 10,000 | 0.055 | 182.20M | 0.059 | 168.48M | 0.154 | 2.80× | 2.59× |
| 100,000 | 0.792 | 126.23M | 0.789 | 126.82M | 1.008 | 1.27× | 1.28× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.074 | 0.156 | 2.10× |
| 1 | 5 | 0.268 | 0.475 | 1.77× |
| 1 | 10 | 0.392 | 0.934 | 2.38× |
| 10 | 1 | 0.039 | 0.088 | 2.25× |
| 10 | 5 | 0.187 | 0.429 | 2.29× |
| 10 | 10 | 0.387 | 0.890 | 2.30× |
| 100 | 1 | 0.039 | 0.092 | 2.36× |
| 100 | 5 | 0.169 | 0.437 | 2.58× |
| 100 | 10 | 0.458 | 1.040 | 2.27× |
| 1,000 | 1 | 0.062 | 0.111 | 1.78× |
| 1,000 | 5 | 0.211 | 0.502 | 2.38× |
| 1,000 | 10 | 0.436 | 0.965 | 2.21× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
