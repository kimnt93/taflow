# Cross benchmark (`causal cross event` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 372.85M | 0.002 | 664.29M | 0.020 | 7.59× | 13.52× |
| 10,000 | 0.012 | 824.83M | 0.008 | 1.18G | 0.044 | 3.64× | 5.20× |
| 100,000 | 0.097 | 1.03G | 0.073 | 1.38G | 0.643 | 6.61× | 8.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.086 | 1.27× |
| 1 | 5 | 0.243 | 0.326 | 1.34× |
| 1 | 10 | 0.359 | 0.687 | 1.91× |
| 10 | 1 | 0.038 | 0.071 | 1.85× |
| 10 | 5 | 0.179 | 0.322 | 1.80× |
| 10 | 10 | 0.396 | 0.687 | 1.74× |
| 100 | 1 | 0.038 | 0.066 | 1.74× |
| 100 | 5 | 0.168 | 0.318 | 1.89× |
| 100 | 10 | 0.380 | 0.717 | 1.88× |
| 1,000 | 1 | 0.044 | 0.073 | 1.67× |
| 1,000 | 5 | 0.191 | 0.539 | 2.82× |
| 1,000 | 10 | 0.400 | 1.147 | 2.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
