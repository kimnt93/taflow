# Amihud benchmark (`AmihudIlliquidity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 106.65M | 0.008 | 129.41M | 0.542 | 57.85× | 70.19× |
| 10,000 | 0.069 | 144.09M | 0.068 | 146.32M | 3.902 | 56.23× | 57.10× |
| 100,000 | 0.644 | 155.39M | 0.639 | 156.52M | 37.643 | 58.49× | 58.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.240 | 2.25× |
| 1 | 5 | 0.228 | 1.088 | 4.77× |
| 1 | 10 | 0.410 | 2.357 | 5.75× |
| 10 | 1 | 0.044 | 0.223 | 5.12× |
| 10 | 5 | 0.187 | 1.058 | 5.67× |
| 10 | 10 | 0.406 | 2.427 | 5.98× |
| 100 | 1 | 0.045 | 0.245 | 5.43× |
| 100 | 5 | 0.206 | 1.243 | 6.03× |
| 100 | 10 | 0.429 | 2.812 | 6.55× |
| 1,000 | 1 | 0.055 | 0.597 | 10.82× |
| 1,000 | 5 | 0.218 | 3.002 | 13.79× |
| 1,000 | 10 | 0.427 | 6.281 | 14.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
