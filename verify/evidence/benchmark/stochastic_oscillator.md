# StochasticOscillator benchmark (`STOCH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 55.22M | 0.015 | 66.97M | 0.053 | 2.95× | 3.57× |
| 10,000 | 0.149 | 66.91M | 0.149 | 66.98M | 0.168 | 1.13× | 1.13× |
| 100,000 | 1.451 | 68.92M | 1.449 | 68.99M | 1.262 | 0.87× | 0.87× |
| 1,000,000 | 16.522 | 60.52M | 16.535 | 60.48M | 14.224 | 0.86× | 0.86× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.132 | 1.20× |
| 1 | 5 | 0.367 | 0.528 | 1.44× |
| 1 | 10 | 0.485 | 1.100 | 2.27× |
| 10 | 1 | 0.053 | 0.113 | 2.13× |
| 10 | 5 | 0.274 | 0.557 | 2.03× |
| 10 | 10 | 0.533 | 1.111 | 2.08× |
| 100 | 1 | 0.056 | 0.105 | 1.87× |
| 100 | 5 | 0.305 | 0.604 | 1.98× |
| 100 | 10 | 0.571 | 1.084 | 1.90× |
| 1,000 | 1 | 0.067 | 0.122 | 1.82× |
| 1,000 | 5 | 0.278 | 0.616 | 2.22× |
| 1,000 | 10 | 0.611 | 1.201 | 1.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
