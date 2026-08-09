# AbsolutePriceOscillator benchmark (`APO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 114.97M | 0.008 | 130.47M | 0.040 | 4.64× | 5.26× |
| 10,000 | 0.050 | 200.92M | 0.048 | 207.41M | 0.080 | 1.61× | 1.66× |
| 100,000 | 0.465 | 214.99M | 0.444 | 225.12M | 0.441 | 0.95× | 0.99× |
| 1,000,000 | 5.034 | 198.65M | 4.450 | 224.74M | 4.946 | 0.98× | 1.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.123 | 1.80× |
| 1 | 5 | 0.325 | 0.505 | 1.55× |
| 1 | 10 | 0.586 | 1.121 | 1.91× |
| 10 | 1 | 0.057 | 0.101 | 1.77× |
| 10 | 5 | 0.244 | 0.492 | 2.02× |
| 10 | 10 | 0.515 | 1.005 | 1.95× |
| 100 | 1 | 0.058 | 0.121 | 2.07× |
| 100 | 5 | 0.266 | 0.535 | 2.01× |
| 100 | 10 | 0.552 | 1.021 | 1.85× |
| 1,000 | 1 | 0.055 | 0.102 | 1.86× |
| 1,000 | 5 | 0.288 | 0.666 | 2.31× |
| 1,000 | 10 | 0.588 | 1.104 | 1.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
