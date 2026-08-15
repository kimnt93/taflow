# SessionVolumeLevels benchmark (`anchored volume levels` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.055 | 18.03M | 0.052 | 19.08M | 14.759 | 266.09× | 281.60× |
| 10,000 | 0.527 | 18.97M | 0.496 | 20.15M | 152.813 | 289.95× | 307.88× |
| 100,000 | 5.020 | 19.92M | 5.077 | 19.70M | 1508.363 | 300.47× | 297.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.204 | 0.193 | 0.95× |
| 1 | 5 | 0.341 | 0.717 | 2.10× |
| 1 | 10 | 0.397 | 1.390 | 3.50× |
| 10 | 1 | 0.065 | 0.319 | 4.92× |
| 10 | 5 | 0.192 | 1.604 | 8.36× |
| 10 | 10 | 0.395 | 3.396 | 8.59× |
| 100 | 1 | 0.054 | 1.884 | 34.92× |
| 100 | 5 | 0.267 | 10.144 | 38.03× |
| 100 | 10 | 0.514 | 20.402 | 39.66× |
| 1,000 | 1 | 0.109 | 15.095 | 138.98× |
| 1,000 | 5 | 0.654 | 81.328 | 124.29× |
| 1,000 | 10 | 0.997 | 169.107 | 169.69× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
