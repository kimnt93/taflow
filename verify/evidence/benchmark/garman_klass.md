# GarmanKlass benchmark (`GarmanKlassVolatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.107 | 9.37M | 0.104 | 9.62M | 0.284 | 2.66× | 2.73× |
| 10,000 | 0.872 | 11.47M | 0.879 | 11.38M | 1.447 | 1.66× | 1.65× |
| 100,000 | 8.531 | 11.72M | 8.713 | 11.48M | 13.192 | 1.55× | 1.51× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.123 | 0.275 | 2.23× |
| 1 | 5 | 0.445 | 1.390 | 3.12× |
| 1 | 10 | 0.645 | 2.505 | 3.89× |
| 10 | 1 | 0.081 | 0.236 | 2.93× |
| 10 | 5 | 0.324 | 1.440 | 4.44× |
| 10 | 10 | 0.665 | 2.743 | 4.13× |
| 100 | 1 | 0.083 | 0.242 | 2.91× |
| 100 | 5 | 0.323 | 1.496 | 4.63× |
| 100 | 10 | 0.656 | 2.593 | 3.95× |
| 1,000 | 1 | 0.193 | 0.417 | 2.15× |
| 1,000 | 5 | 0.381 | 2.270 | 5.96× |
| 1,000 | 10 | 0.732 | 4.116 | 5.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
