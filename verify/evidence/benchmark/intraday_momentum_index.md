# IntradayMomentumIndex benchmark (`IMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.058 | 17.23M | 0.053 | 18.73M | 0.080 | 1.38× | 1.50× |
| 10,000 | 0.437 | 22.88M | 0.448 | 22.34M | 0.575 | 1.32× | 1.28× |
| 100,000 | 4.537 | 22.04M | 4.391 | 22.77M | 5.401 | 1.19× | 1.23× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.107 | 0.89× |
| 1 | 5 | 0.511 | 0.550 | 1.08× |
| 1 | 10 | 0.626 | 0.949 | 1.52× |
| 10 | 1 | 0.065 | 0.094 | 1.45× |
| 10 | 5 | 0.318 | 0.473 | 1.48× |
| 10 | 10 | 0.633 | 0.946 | 1.49× |
| 100 | 1 | 0.068 | 0.100 | 1.46× |
| 100 | 5 | 0.315 | 0.469 | 1.49× |
| 100 | 10 | 0.647 | 0.986 | 1.52× |
| 1,000 | 1 | 0.110 | 0.150 | 1.37× |
| 1,000 | 5 | 0.318 | 0.727 | 2.28× |
| 1,000 | 10 | 0.660 | 1.544 | 2.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
