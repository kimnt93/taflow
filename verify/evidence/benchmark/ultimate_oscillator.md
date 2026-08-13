# UltimateOscillator benchmark (`ULTOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.152 | 6.57M | 0.143 | 6.99M | 0.049 | 0.32× | 0.34× |
| 10,000 | 1.322 | 7.57M | 1.427 | 7.01M | 0.176 | 0.13× | 0.12× |
| 100,000 | 13.006 | 7.69M | 12.982 | 7.70M | 1.413 | 0.11× | 0.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.115 | 1.02× |
| 1 | 5 | 0.453 | 0.520 | 1.15× |
| 1 | 10 | 0.667 | 1.017 | 1.53× |
| 10 | 1 | 0.070 | 0.102 | 1.45× |
| 10 | 5 | 0.322 | 0.468 | 1.45× |
| 10 | 10 | 0.682 | 0.983 | 1.44× |
| 100 | 1 | 0.090 | 0.093 | 1.04× |
| 100 | 5 | 0.307 | 0.467 | 1.52× |
| 100 | 10 | 0.678 | 0.974 | 1.44× |
| 1,000 | 1 | 0.222 | 0.132 | 0.60× |
| 1,000 | 5 | 0.435 | 0.562 | 1.29× |
| 1,000 | 10 | 0.732 | 1.153 | 1.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
