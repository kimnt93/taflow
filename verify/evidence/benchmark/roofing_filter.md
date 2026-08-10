# RoofingFilter benchmark (`RoofingFilter` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 133.12M | 0.007 | 140.68M | 0.191 | 25.42× | 26.87× |
| 10,000 | 0.051 | 195.42M | 0.049 | 204.15M | 0.511 | 9.99× | 10.43× |
| 100,000 | 0.474 | 211.19M | 0.455 | 219.91M | 3.725 | 7.87× | 8.19× |
| 1,000,000 | 4.889 | 204.55M | 4.537 | 220.41M | 36.817 | 7.53× | 8.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.125 | 0.328 | 2.63× |
| 1 | 5 | 0.270 | 1.205 | 4.46× |
| 1 | 10 | 0.466 | 2.423 | 5.19× |
| 10 | 1 | 0.053 | 0.240 | 4.52× |
| 10 | 5 | 0.220 | 1.344 | 6.11× |
| 10 | 10 | 0.489 | 2.614 | 5.34× |
| 100 | 1 | 0.056 | 0.244 | 4.38× |
| 100 | 5 | 0.220 | 1.362 | 6.18× |
| 100 | 10 | 0.494 | 2.519 | 5.10× |
| 1,000 | 1 | 0.052 | 0.270 | 5.22× |
| 1,000 | 5 | 0.234 | 1.583 | 6.77× |
| 1,000 | 10 | 0.528 | 3.116 | 5.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
