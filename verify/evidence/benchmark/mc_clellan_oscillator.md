# McClellanOscillator benchmark (`McClellanOscillator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 110.91M | 0.008 | 129.65M | 9.203 | 1020.71× | 1193.11× |
| 10,000 | 0.055 | 183.16M | 0.049 | 202.63M | 90.143 | 1651.06× | 1826.60× |
| 100,000 | 0.467 | 214.34M | 0.441 | 226.58M | 871.388 | 1867.76× | 1974.36× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.336 | 2.86× |
| 1 | 5 | 0.300 | 1.413 | 4.72× |
| 1 | 10 | 0.475 | 2.163 | 4.56× |
| 10 | 1 | 0.051 | 0.300 | 5.93× |
| 10 | 5 | 0.227 | 1.743 | 7.67× |
| 10 | 10 | 0.504 | 3.073 | 6.09× |
| 100 | 1 | 0.051 | 1.113 | 21.64× |
| 100 | 5 | 0.235 | 6.001 | 25.56× |
| 100 | 10 | 0.527 | 11.998 | 22.78× |
| 1,000 | 1 | 0.060 | 9.212 | 152.60× |
| 1,000 | 5 | 0.464 | 46.515 | 100.15× |
| 1,000 | 10 | 0.604 | 99.482 | 164.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
