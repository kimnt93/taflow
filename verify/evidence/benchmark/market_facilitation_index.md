# MarketFacilitationIndex benchmark (`MarketFacilitationIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 108.43M | 0.007 | 144.16M | 0.197 | 21.38× | 28.43× |
| 10,000 | 0.029 | 342.06M | 0.025 | 402.62M | 1.105 | 37.79× | 44.49× |
| 100,000 | 0.223 | 447.90M | 0.198 | 504.89M | 10.296 | 46.12× | 51.99× |
| 1,000,000 | 3.076 | 325.15M | 2.374 | 421.30M | 105.072 | 34.16× | 44.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.201 | 2.52× |
| 1 | 5 | 0.351 | 0.884 | 2.52× |
| 1 | 10 | 0.590 | 1.851 | 3.14× |
| 10 | 1 | 0.054 | 0.158 | 2.92× |
| 10 | 5 | 0.243 | 0.880 | 3.62× |
| 10 | 10 | 0.547 | 1.897 | 3.47× |
| 100 | 1 | 0.053 | 0.174 | 3.28× |
| 100 | 5 | 0.252 | 0.921 | 3.66× |
| 100 | 10 | 0.547 | 1.795 | 3.28× |
| 1,000 | 1 | 0.054 | 0.258 | 4.76× |
| 1,000 | 5 | 0.291 | 1.644 | 5.64× |
| 1,000 | 10 | 0.562 | 2.737 | 4.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
