# HilbertTransformDominantCyclePhase benchmark (`HT_DCPHASE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.115 | 8.68M | 0.103 | 9.73M | 0.471 | 4.09× | 4.58× |
| 10,000 | 1.152 | 8.68M | 1.091 | 9.17M | 4.600 | 3.99× | 4.22× |
| 100,000 | 10.494 | 9.53M | 10.493 | 9.53M | 45.425 | 4.33× | 4.33× |
| 1,000,000 | 107.985 | 9.26M | 106.723 | 9.37M | 488.959 | 4.53× | 4.58× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.118 | 1.50× |
| 1 | 5 | 0.365 | 0.611 | 1.67× |
| 1 | 10 | 0.614 | 1.104 | 1.80× |
| 10 | 1 | 0.053 | 0.091 | 1.70× |
| 10 | 5 | 0.218 | 0.411 | 1.89× |
| 10 | 10 | 0.435 | 0.994 | 2.28× |
| 100 | 1 | 0.065 | 0.112 | 1.71× |
| 100 | 5 | 0.220 | 0.547 | 2.49× |
| 100 | 10 | 0.504 | 1.173 | 2.33× |
| 1,000 | 1 | 0.162 | 0.583 | 3.59× |
| 1,000 | 5 | 0.328 | 2.625 | 8.01× |
| 1,000 | 10 | 0.608 | 5.414 | 8.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
