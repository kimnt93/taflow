# SqueezePro benchmark (`squeeze_pro` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.040 | 24.77M | 0.034 | 29.11M | 8.036 | 199.02× | 233.91× |
| 10,000 | 0.299 | 33.45M | 0.314 | 31.82M | 11.347 | 37.96× | 36.11× |
| 100,000 | 3.042 | 32.87M | 2.985 | 33.50M | 48.609 | 15.98× | 16.28× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.056 | 0.360 | 6.45× |
| 1 | 5 | 0.402 | 1.902 | 4.73× |
| 1 | 10 | 0.433 | 3.659 | 8.45× |
| 10 | 1 | 0.052 | 0.339 | 6.51× |
| 10 | 5 | 0.214 | 1.699 | 7.95× |
| 10 | 10 | 0.436 | 3.353 | 7.68× |
| 100 | 1 | 0.052 | 8.239 | 159.89× |
| 100 | 5 | 0.354 | 42.578 | 120.29× |
| 100 | 10 | 0.521 | 87.910 | 168.75× |
| 1,000 | 1 | 0.086 | 8.837 | 102.68× |
| 1,000 | 5 | 0.414 | 47.814 | 115.44× |
| 1,000 | 10 | 0.455 | 95.789 | 210.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
