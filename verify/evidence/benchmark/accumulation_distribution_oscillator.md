# AccumulationDistributionOscillator benchmark (`ADOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 66.49M | 0.020 | 50.90M | 0.041 | 2.71× | 2.07× |
| 10,000 | 0.094 | 106.78M | 0.093 | 107.58M | 0.062 | 0.66× | 0.67× |
| 100,000 | 0.823 | 121.46M | 0.792 | 126.29M | 0.299 | 0.36× | 0.38× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.115 | 1.12× |
| 1 | 5 | 0.309 | 0.475 | 1.54× |
| 1 | 10 | 0.481 | 0.972 | 2.02× |
| 10 | 1 | 0.050 | 0.093 | 1.87× |
| 10 | 5 | 0.266 | 0.506 | 1.91× |
| 10 | 10 | 0.493 | 0.952 | 1.93× |
| 100 | 1 | 0.052 | 0.090 | 1.71× |
| 100 | 5 | 0.240 | 0.489 | 2.04× |
| 100 | 10 | 0.576 | 0.987 | 1.71× |
| 1,000 | 1 | 0.058 | 0.099 | 1.70× |
| 1,000 | 5 | 0.254 | 0.501 | 1.97× |
| 1,000 | 10 | 0.569 | 1.092 | 1.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
