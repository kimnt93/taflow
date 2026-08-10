# FastStochasticOscillator benchmark (`STOCHF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 51.17M | 0.017 | 57.67M | 0.044 | 2.23× | 2.52× |
| 10,000 | 0.159 | 62.73M | 0.166 | 60.12M | 0.138 | 0.87× | 0.83× |
| 100,000 | 1.592 | 62.80M | 1.611 | 62.09M | 1.051 | 0.66× | 0.65× |
| 1,000,000 | 17.529 | 57.05M | 17.394 | 57.49M | 10.572 | 0.60× | 0.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.199 | 1.86× |
| 1 | 5 | 0.336 | 0.603 | 1.80× |
| 1 | 10 | 0.495 | 1.025 | 2.07× |
| 10 | 1 | 0.056 | 0.103 | 1.84× |
| 10 | 5 | 0.233 | 0.498 | 2.14× |
| 10 | 10 | 0.511 | 1.024 | 2.00× |
| 100 | 1 | 0.054 | 0.108 | 2.01× |
| 100 | 5 | 0.264 | 0.502 | 1.90× |
| 100 | 10 | 0.515 | 1.068 | 2.08× |
| 1,000 | 1 | 0.072 | 0.114 | 1.58× |
| 1,000 | 5 | 0.255 | 0.560 | 2.20× |
| 1,000 | 10 | 0.577 | 1.235 | 2.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
