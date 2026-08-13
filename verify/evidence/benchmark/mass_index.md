# MassIndex benchmark (`MassIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.075 | 13.26M | 0.086 | 11.68M | 0.226 | 3.00× | 2.64× |
| 10,000 | 0.580 | 17.23M | 0.591 | 16.93M | 0.842 | 1.45× | 1.43× |
| 100,000 | 5.631 | 17.76M | 5.796 | 17.25M | 7.061 | 1.25× | 1.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.138 | 0.272 | 1.97× |
| 1 | 5 | 0.412 | 1.448 | 3.52× |
| 1 | 10 | 0.675 | 2.725 | 4.04× |
| 10 | 1 | 0.079 | 0.251 | 3.17× |
| 10 | 5 | 0.327 | 1.508 | 4.61× |
| 10 | 10 | 0.679 | 2.642 | 3.89× |
| 100 | 1 | 0.090 | 0.256 | 2.85× |
| 100 | 5 | 0.324 | 1.550 | 4.78× |
| 100 | 10 | 0.703 | 2.991 | 4.25× |
| 1,000 | 1 | 0.141 | 0.323 | 2.29× |
| 1,000 | 5 | 0.321 | 1.874 | 5.85× |
| 1,000 | 10 | 0.660 | 3.315 | 5.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
