# RelativeStrengthIndex benchmark (`RSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 95.82M | 0.010 | 104.62M | 0.038 | 3.66× | 4.00× |
| 10,000 | 0.087 | 115.13M | 0.077 | 130.32M | 0.085 | 0.98× | 1.11× |
| 100,000 | 0.774 | 129.19M | 0.734 | 136.29M | 0.577 | 0.74× | 0.79× |
| 1,000,000 | 7.882 | 126.87M | 7.313 | 136.74M | 5.819 | 0.74× | 0.80× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.121 | 1.48× |
| 1 | 5 | 0.471 | 0.499 | 1.06× |
| 1 | 10 | 0.469 | 0.970 | 2.07× |
| 10 | 1 | 0.049 | 0.094 | 1.92× |
| 10 | 5 | 0.215 | 0.450 | 2.09× |
| 10 | 10 | 0.483 | 0.948 | 1.96× |
| 100 | 1 | 0.050 | 0.090 | 1.81× |
| 100 | 5 | 0.211 | 0.440 | 2.09× |
| 100 | 10 | 0.466 | 0.962 | 2.06× |
| 1,000 | 1 | 0.056 | 0.096 | 1.72× |
| 1,000 | 5 | 0.238 | 0.472 | 1.98× |
| 1,000 | 10 | 0.476 | 0.984 | 2.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
