# UltimateOscillator benchmark (`ULTOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 46.91M | 0.019 | 52.91M | 0.053 | 2.51× | 2.83× |
| 10,000 | 0.152 | 65.77M | 0.151 | 66.28M | 0.198 | 1.30× | 1.31× |
| 100,000 | 2.242 | 44.60M | 1.475 | 67.80M | 1.687 | 0.75× | 1.14× |
| 1,000,000 | 15.769 | 63.42M | 15.167 | 65.93M | 17.057 | 1.08× | 1.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.142 | 1.62× |
| 1 | 5 | 0.328 | 0.549 | 1.68× |
| 1 | 10 | 0.492 | 1.046 | 2.13× |
| 10 | 1 | 0.080 | 0.099 | 1.23× |
| 10 | 5 | 0.274 | 0.529 | 1.93× |
| 10 | 10 | 0.571 | 1.059 | 1.85× |
| 100 | 1 | 0.060 | 0.108 | 1.79× |
| 100 | 5 | 0.285 | 0.518 | 1.82× |
| 100 | 10 | 0.596 | 1.092 | 1.83× |
| 1,000 | 1 | 0.073 | 0.113 | 1.54× |
| 1,000 | 5 | 0.272 | 0.619 | 2.28× |
| 1,000 | 10 | 0.620 | 1.296 | 2.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
