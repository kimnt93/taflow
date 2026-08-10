# McGinleyDynamic benchmark (`McGinleyDynamic` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 61.57M | 0.015 | 66.38M | 0.186 | 11.45× | 12.35× |
| 10,000 | 0.127 | 78.52M | 0.118 | 84.52M | 0.568 | 4.46× | 4.80× |
| 100,000 | 1.146 | 87.27M | 1.140 | 87.70M | 4.299 | 3.75× | 3.77× |
| 1,000,000 | 11.567 | 86.45M | 11.484 | 87.08M | 43.010 | 3.72× | 3.75× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.267 | 2.81× |
| 1 | 5 | 0.362 | 1.394 | 3.85× |
| 1 | 10 | 0.446 | 2.450 | 5.49× |
| 10 | 1 | 0.051 | 0.221 | 4.36× |
| 10 | 5 | 0.232 | 1.492 | 6.43× |
| 10 | 10 | 0.493 | 2.464 | 4.99× |
| 100 | 1 | 0.062 | 0.220 | 3.52× |
| 100 | 5 | 0.246 | 1.388 | 5.64× |
| 100 | 10 | 0.510 | 2.460 | 4.83× |
| 1,000 | 1 | 0.060 | 0.258 | 4.30× |
| 1,000 | 5 | 0.240 | 1.616 | 6.73× |
| 1,000 | 10 | 0.544 | 2.975 | 5.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
