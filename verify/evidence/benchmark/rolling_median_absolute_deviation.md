# RollingMedianAbsoluteDeviation benchmark (`MedianAbsoluteDeviation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 5.591 | 178.86K | 5.531 | 180.80K | 0.470 | 0.08× | 0.08× |
| 10,000 | 55.153 | 181.32K | 54.674 | 182.90K | 3.460 | 0.06× | 0.06× |
| 100,000 | 537.756 | 185.96K | 540.660 | 184.96K | 33.077 | 0.06× | 0.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.205 | 0.234 | 1.14× |
| 1 | 5 | 0.415 | 1.293 | 3.12× |
| 1 | 10 | 0.613 | 2.282 | 3.72× |
| 10 | 1 | 0.079 | 0.213 | 2.68× |
| 10 | 5 | 0.299 | 1.322 | 4.43× |
| 10 | 10 | 0.636 | 2.244 | 3.53× |
| 100 | 1 | 0.528 | 0.251 | 0.47× |
| 100 | 5 | 0.909 | 1.382 | 1.52× |
| 100 | 10 | 1.232 | 2.781 | 2.26× |
| 1,000 | 1 | 5.607 | 0.596 | 0.11× |
| 1,000 | 5 | 5.941 | 3.201 | 0.54× |
| 1,000 | 10 | 10.599 | 6.008 | 0.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
