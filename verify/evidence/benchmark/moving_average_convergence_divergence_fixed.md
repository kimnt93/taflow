# MovingAverageConvergenceDivergenceFixed benchmark (`MACDFIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 155.92M | 0.005 | 205.94M | 0.049 | 7.67× | 10.13× |
| 10,000 | 0.034 | 289.90M | 0.025 | 397.09M | 0.135 | 3.90× | 5.34× |
| 100,000 | 0.303 | 329.55M | 0.226 | 441.76M | 1.007 | 3.32× | 4.45× |
| 1,000,000 | 13.674 | 73.13M | 2.583 | 387.10M | 12.888 | 0.94× | 4.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.211 | 0.126 | 0.60× |
| 1 | 5 | 0.275 | 0.547 | 1.99× |
| 1 | 10 | 0.480 | 1.028 | 2.14× |
| 10 | 1 | 0.050 | 0.104 | 2.09× |
| 10 | 5 | 0.238 | 0.541 | 2.27× |
| 10 | 10 | 0.492 | 1.013 | 2.06× |
| 100 | 1 | 0.049 | 0.110 | 2.24× |
| 100 | 5 | 0.238 | 0.513 | 2.15× |
| 100 | 10 | 0.474 | 1.008 | 2.13× |
| 1,000 | 1 | 0.063 | 0.108 | 1.70× |
| 1,000 | 5 | 0.233 | 0.522 | 2.25× |
| 1,000 | 10 | 0.505 | 1.170 | 2.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
