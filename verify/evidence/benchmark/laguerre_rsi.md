# LaguerreRelativeStrengthIndex benchmark (`LaguerreRSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.042 | 23.57M | 0.039 | 25.60M | 0.176 | 4.16× | 4.52× |
| 10,000 | 0.329 | 30.43M | 0.309 | 32.36M | 0.537 | 1.64× | 1.74× |
| 100,000 | 3.241 | 30.86M | 3.010 | 33.22M | 4.082 | 1.26× | 1.36× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.213 | 0.250 | 1.18× |
| 1 | 5 | 0.404 | 1.373 | 3.40× |
| 1 | 10 | 0.572 | 2.452 | 4.29× |
| 10 | 1 | 0.073 | 0.212 | 2.91× |
| 10 | 5 | 0.280 | 1.350 | 4.82× |
| 10 | 10 | 0.586 | 2.319 | 3.96× |
| 100 | 1 | 0.080 | 0.218 | 2.73× |
| 100 | 5 | 0.273 | 1.358 | 4.98× |
| 100 | 10 | 0.643 | 2.672 | 4.15× |
| 1,000 | 1 | 0.118 | 0.253 | 2.15× |
| 1,000 | 5 | 0.280 | 1.556 | 5.55× |
| 1,000 | 10 | 0.628 | 2.775 | 4.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
