# StochasticOscillator benchmark (`STOCH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.165 | 6.04M | 0.146 | 6.85M | 0.051 | 0.31× | 0.35× |
| 10,000 | 1.346 | 7.43M | 1.330 | 7.52M | 0.177 | 0.13× | 0.13× |
| 100,000 | 13.568 | 7.37M | 13.487 | 7.41M | 1.242 | 0.09× | 0.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.132 | 0.122 | 0.92× |
| 1 | 5 | 0.463 | 0.537 | 1.16× |
| 1 | 10 | 0.783 | 1.076 | 1.37× |
| 10 | 1 | 0.088 | 0.115 | 1.30× |
| 10 | 5 | 0.387 | 0.518 | 1.34× |
| 10 | 10 | 0.843 | 1.147 | 1.36× |
| 100 | 1 | 0.100 | 0.112 | 1.12× |
| 100 | 5 | 0.410 | 0.543 | 1.33× |
| 100 | 10 | 0.902 | 1.128 | 1.25× |
| 1,000 | 1 | 0.239 | 0.130 | 0.54× |
| 1,000 | 5 | 0.515 | 0.593 | 1.15× |
| 1,000 | 10 | 0.888 | 1.260 | 1.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
