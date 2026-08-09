# FastStochasticOscillator benchmark (`STOCHF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.68M | 0.017 | 58.94M | 0.048 | 2.71× | 2.82× |
| 10,000 | 0.177 | 56.50M | 0.172 | 58.11M | 0.148 | 0.84× | 0.86× |
| 100,000 | 1.756 | 56.95M | 1.755 | 56.98M | 1.103 | 0.63× | 0.63× |
| 1,000,000 | 20.402 | 49.01M | 18.737 | 53.37M | 15.784 | 0.77× | 0.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.176 | 0.937 | 5.33× |
| 1 | 5 | 1.581 | 1.436 | 0.91× |
| 1 | 10 | 1.589 | 1.492 | 0.94× |
| 10 | 1 | 1.969 | 0.338 | 0.17× |
| 10 | 5 | 0.415 | 1.934 | 4.66× |
| 10 | 10 | 0.834 | 1.460 | 1.75× |
| 100 | 1 | 0.076 | 0.151 | 1.99× |
| 100 | 5 | 0.466 | 0.766 | 1.65× |
| 100 | 10 | 0.835 | 1.664 | 1.99× |
| 1,000 | 1 | 0.094 | 0.168 | 1.79× |
| 1,000 | 5 | 0.414 | 0.893 | 2.16× |
| 1,000 | 10 | 0.761 | 1.586 | 2.08× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
