# Rising benchmark (`period-over-period rising` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 119.98M | 0.007 | 135.88M | 0.033 | 3.99× | 4.52× |
| 10,000 | 0.055 | 182.07M | 0.050 | 198.77M | 0.040 | 0.73× | 0.79× |
| 100,000 | 0.482 | 207.59M | 0.450 | 222.16M | 0.125 | 0.26× | 0.28× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.108 | 1.08× |
| 1 | 5 | 0.352 | 0.450 | 1.28× |
| 1 | 10 | 0.472 | 0.923 | 1.96× |
| 10 | 1 | 0.048 | 0.092 | 1.90× |
| 10 | 5 | 0.271 | 0.467 | 1.72× |
| 10 | 10 | 0.489 | 0.925 | 1.89× |
| 100 | 1 | 0.053 | 0.096 | 1.79× |
| 100 | 5 | 0.239 | 0.460 | 1.93× |
| 100 | 10 | 0.524 | 0.946 | 1.81× |
| 1,000 | 1 | 0.056 | 0.089 | 1.57× |
| 1,000 | 5 | 0.220 | 0.486 | 2.21× |
| 1,000 | 10 | 0.484 | 1.186 | 2.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
