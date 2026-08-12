# CandleEngulfing benchmark (`CDLENGULFING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 89.87M | 0.008 | 118.14M | 0.030 | 2.73× | 3.58× |
| 10,000 | 0.081 | 122.94M | 0.073 | 137.72M | 0.079 | 0.97× | 1.09× |
| 100,000 | 0.749 | 133.43M | 0.797 | 125.50M | 0.579 | 0.77× | 0.73× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.129 | 0.145 | 1.12× |
| 1 | 5 | 0.369 | 0.482 | 1.31× |
| 1 | 10 | 0.523 | 0.963 | 1.84× |
| 10 | 1 | 0.065 | 0.098 | 1.50× |
| 10 | 5 | 0.251 | 0.440 | 1.75× |
| 10 | 10 | 0.502 | 0.868 | 1.73× |
| 100 | 1 | 0.063 | 0.094 | 1.48× |
| 100 | 5 | 0.256 | 0.454 | 1.77× |
| 100 | 10 | 0.542 | 0.910 | 1.68× |
| 1,000 | 1 | 0.060 | 0.093 | 1.55× |
| 1,000 | 5 | 0.271 | 0.492 | 1.82× |
| 1,000 | 10 | 0.618 | 1.050 | 1.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
