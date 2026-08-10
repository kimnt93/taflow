# AccumulationDistributionOscillator benchmark (`ADOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.83M | 0.013 | 74.44M | 0.039 | 2.54× | 2.87× |
| 10,000 | 0.092 | 108.49M | 0.090 | 111.07M | 0.063 | 0.69× | 0.70× |
| 100,000 | 0.923 | 108.38M | 0.832 | 120.22M | 0.305 | 0.33× | 0.37× |
| 1,000,000 | 8.811 | 113.50M | 8.459 | 118.22M | 3.096 | 0.35× | 0.37× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.164 | 1.42× |
| 1 | 5 | 0.332 | 0.500 | 1.51× |
| 1 | 10 | 0.497 | 1.039 | 2.09× |
| 10 | 1 | 0.054 | 0.105 | 1.95× |
| 10 | 5 | 0.243 | 0.482 | 1.98× |
| 10 | 10 | 0.512 | 1.024 | 2.00× |
| 100 | 1 | 0.062 | 0.112 | 1.79× |
| 100 | 5 | 0.246 | 0.503 | 2.04× |
| 100 | 10 | 0.520 | 0.977 | 1.88× |
| 1,000 | 1 | 0.059 | 0.102 | 1.72× |
| 1,000 | 5 | 0.235 | 0.532 | 2.27× |
| 1,000 | 10 | 0.568 | 1.005 | 1.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
