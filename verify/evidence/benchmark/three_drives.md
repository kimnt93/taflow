# ThreeDrives benchmark (`ThreeDrives` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 93.98M | 0.008 | 131.64M | 0.232 | 21.81× | 30.55× |
| 10,000 | 0.107 | 93.69M | 0.096 | 103.77M | 1.411 | 13.22× | 14.64× |
| 100,000 | 1.060 | 94.35M | 0.998 | 100.18M | 13.060 | 12.32× | 13.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.199 | 2.03× |
| 1 | 5 | 0.280 | 0.816 | 2.92× |
| 1 | 10 | 0.418 | 1.640 | 3.92× |
| 10 | 1 | 0.044 | 0.166 | 3.75× |
| 10 | 5 | 0.198 | 1.137 | 5.75× |
| 10 | 10 | 0.422 | 1.663 | 3.94× |
| 100 | 1 | 0.043 | 0.174 | 4.01× |
| 100 | 5 | 0.201 | 1.179 | 5.86× |
| 100 | 10 | 0.449 | 1.788 | 3.98× |
| 1,000 | 1 | 0.056 | 0.298 | 5.36× |
| 1,000 | 5 | 0.199 | 1.838 | 9.23× |
| 1,000 | 10 | 0.419 | 3.033 | 7.23× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
