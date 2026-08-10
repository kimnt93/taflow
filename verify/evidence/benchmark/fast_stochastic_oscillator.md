# FastStochasticOscillator benchmark (`STOCHF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 46.64M | 0.019 | 51.34M | 0.052 | 2.41× | 2.66× |
| 10,000 | 0.195 | 51.40M | 0.178 | 56.33M | 0.147 | 0.76× | 0.83× |
| 100,000 | 1.768 | 56.58M | 1.694 | 59.03M | 1.067 | 0.60× | 0.63× |
| 1,000,000 | 19.101 | 52.35M | 17.973 | 55.64M | 10.965 | 0.57× | 0.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.129 | 0.184 | 1.43× |
| 1 | 5 | 0.379 | 0.543 | 1.43× |
| 1 | 10 | 0.519 | 1.026 | 1.98× |
| 10 | 1 | 0.055 | 0.098 | 1.80× |
| 10 | 5 | 0.262 | 0.531 | 2.03× |
| 10 | 10 | 0.544 | 1.045 | 1.92× |
| 100 | 1 | 0.052 | 0.103 | 1.97× |
| 100 | 5 | 0.275 | 0.511 | 1.86× |
| 100 | 10 | 0.545 | 1.084 | 1.99× |
| 1,000 | 1 | 0.069 | 0.125 | 1.82× |
| 1,000 | 5 | 0.248 | 0.551 | 2.22× |
| 1,000 | 10 | 0.531 | 1.209 | 2.28× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
