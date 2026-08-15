# AccelerationBands benchmark (`ACCBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 86.08M | 0.008 | 118.03M | 0.047 | 4.08× | 5.59× |
| 10,000 | 0.076 | 132.30M | 0.068 | 147.89M | 0.116 | 1.53× | 1.71× |
| 100,000 | 1.810 | 55.24M | 1.460 | 68.47M | 1.356 | 0.75× | 0.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.067 | 0.122 | 1.82× |
| 1 | 5 | 0.214 | 0.507 | 2.37× |
| 1 | 10 | 0.406 | 1.012 | 2.49× |
| 10 | 1 | 0.043 | 0.106 | 2.46× |
| 10 | 5 | 0.177 | 0.471 | 2.66× |
| 10 | 10 | 0.390 | 1.022 | 2.62× |
| 100 | 1 | 0.043 | 0.102 | 2.40× |
| 100 | 5 | 0.193 | 0.506 | 2.62× |
| 100 | 10 | 0.403 | 1.025 | 2.54× |
| 1,000 | 1 | 0.058 | 0.117 | 2.02× |
| 1,000 | 5 | 0.191 | 0.530 | 2.77× |
| 1,000 | 10 | 0.413 | 1.083 | 2.62× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
