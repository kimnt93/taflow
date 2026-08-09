# MovingAverage benchmark (`MA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 147.89M | 0.006 | 164.61M | 0.038 | 5.57× | 6.19× |
| 10,000 | 0.047 | 212.61M | 0.043 | 230.96M | 0.053 | 1.12× | 1.22× |
| 100,000 | 0.452 | 221.34M | 0.415 | 241.21M | 0.214 | 0.47× | 0.52× |
| 1,000,000 | 5.101 | 196.05M | 4.631 | 215.92M | 2.144 | 0.42× | 0.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.074 | 0.116 | 1.56× |
| 1 | 5 | 0.263 | 0.517 | 1.96× |
| 1 | 10 | 0.464 | 1.071 | 2.31× |
| 10 | 1 | 0.050 | 0.096 | 1.92× |
| 10 | 5 | 0.238 | 0.499 | 2.10× |
| 10 | 10 | 0.507 | 1.026 | 2.02× |
| 100 | 1 | 0.053 | 0.102 | 1.91× |
| 100 | 5 | 0.266 | 0.620 | 2.33× |
| 100 | 10 | 0.504 | 1.065 | 2.11× |
| 1,000 | 1 | 0.059 | 0.112 | 1.89× |
| 1,000 | 5 | 0.285 | 0.555 | 1.95× |
| 1,000 | 10 | 0.589 | 1.152 | 1.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
