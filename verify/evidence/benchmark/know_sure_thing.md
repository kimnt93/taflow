# KnowSureThing benchmark (`KST` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.00M | 0.019 | 52.01M | 0.743 | 36.39× | 38.63× |
| 10,000 | 0.165 | 60.49M | 0.178 | 56.13M | 3.667 | 22.18× | 20.59× |
| 100,000 | 1.617 | 61.84M | 1.717 | 58.24M | 36.872 | 22.80× | 21.47× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | 0.589 | 7.84× |
| 1 | 5 | 0.287 | 2.617 | 9.10× |
| 1 | 10 | 0.426 | 5.144 | 12.07× |
| 10 | 1 | 0.054 | 0.485 | 8.89× |
| 10 | 5 | 0.200 | 2.634 | 13.18× |
| 10 | 10 | 0.435 | 5.207 | 11.97× |
| 100 | 1 | 0.050 | 0.510 | 10.12× |
| 100 | 5 | 0.213 | 2.810 | 13.20× |
| 100 | 10 | 0.440 | 5.477 | 12.44× |
| 1,000 | 1 | 0.065 | 1.072 | 16.53× |
| 1,000 | 5 | 0.221 | 4.540 | 20.57× |
| 1,000 | 10 | 0.484 | 9.298 | 19.22× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
