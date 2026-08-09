# MathSqrt benchmark (`SQRT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 244.90M | 0.003 | 334.28M | 0.029 | 7.10× | 9.69× |
| 10,000 | 0.013 | 755.02M | 0.010 | 985.87M | 0.044 | 3.32× | 4.34× |
| 100,000 | 0.096 | 1.04G | 0.072 | 1.38G | 0.165 | 1.71× | 2.28× |
| 1,000,000 | 1.284 | 778.68M | 0.767 | 1.30G | 1.444 | 1.12× | 1.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.147 | 1.37× |
| 1 | 5 | 0.313 | 0.462 | 1.48× |
| 1 | 10 | 0.446 | 0.918 | 2.06× |
| 10 | 1 | 0.049 | 0.093 | 1.89× |
| 10 | 5 | 0.229 | 0.404 | 1.76× |
| 10 | 10 | 0.456 | 0.894 | 1.96× |
| 100 | 1 | 0.046 | 0.084 | 1.85× |
| 100 | 5 | 0.249 | 0.447 | 1.79× |
| 100 | 10 | 0.488 | 0.933 | 1.91× |
| 1,000 | 1 | 0.049 | 0.087 | 1.76× |
| 1,000 | 5 | 0.226 | 0.424 | 1.87× |
| 1,000 | 10 | 0.463 | 0.927 | 2.00× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.285 | 0.154 | 6.49M | 169.110 | 1097.39× | 165.09× |
| 100,000 | 10 | 0.987 | 0.476 | 21.01M | 167.209 | 351.28× | 55.46× |
| 100,000 | 1,000 | 3.380 | 2.191 | 456.38M | 171.807 | 78.41× | 12.86× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 487.67M | 656.61M | 1.00× | 2.74M | 3.68M | 1.00× | 276.04M |
| 5 | 757.84M | 1.29G | 1.96× | 2.17M | 2.93M | 0.80× | 400.89M |
| 10 | 648.66M | 1.41G | 2.14× | 2.18M | 2.86M | 0.78× | 384.54M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
