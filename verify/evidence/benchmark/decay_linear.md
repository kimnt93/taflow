# DecayLinear benchmark (`linear decay weighted mean` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.42M | 0.039 | 25.74M | 0.078 | 1.82× | 2.00× |
| 10,000 | 0.325 | 30.76M | 0.315 | 31.73M | 0.271 | 0.83× | 0.86× |
| 100,000 | 3.130 | 31.95M | 3.060 | 32.68M | 2.128 | 0.68× | 0.70× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | 0.121 | 1.36× |
| 1 | 5 | 0.400 | 0.601 | 1.50× |
| 1 | 10 | 0.609 | 1.112 | 1.83× |
| 10 | 1 | 0.075 | 0.107 | 1.43× |
| 10 | 5 | 0.303 | 0.526 | 1.74× |
| 10 | 10 | 0.607 | 1.104 | 1.82× |
| 100 | 1 | 0.070 | 0.143 | 2.04× |
| 100 | 5 | 0.306 | 0.709 | 2.32× |
| 100 | 10 | 0.610 | 1.426 | 2.34× |
| 1,000 | 1 | 0.098 | 0.167 | 1.72× |
| 1,000 | 5 | 0.299 | 0.752 | 2.51× |
| 1,000 | 10 | 0.670 | 1.666 | 2.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
