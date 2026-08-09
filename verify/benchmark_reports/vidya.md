# VariableIndexDynamicAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 74.67M | 0.012 | 81.72M | 1.679 | 125.36× | 137.18× |
| 10,000 | 0.118 | 85.01M | 0.114 | 88.03M | 7.133 | 60.64× | 62.79× |
| 100,000 | 1.185 | 84.35M | 1.147 | 87.15M | 60.427 | 50.97× | 52.66× |
| 1,000,000 | 16.586 | 60.29M | 11.911 | 83.95M | 605.645 | 36.52× | 50.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.150 | 1.74× |
| 1 | 5 | 0.333 | 0.519 | 1.56× |
| 1 | 10 | 0.506 | 0.927 | 1.83× |
| 10 | 1 | 0.048 | 0.089 | 1.85× |
| 10 | 5 | 0.218 | 0.434 | 1.99× |
| 10 | 10 | 0.504 | 0.945 | 1.87× |
| 100 | 1 | 0.053 | 1.295 | 24.61× |
| 100 | 5 | 0.256 | 6.331 | 24.69× |
| 100 | 10 | 0.513 | 12.551 | 24.46× |
| 1,000 | 1 | 0.067 | 1.752 | 26.33× |
| 1,000 | 5 | 0.246 | 9.605 | 39.09× |
| 1,000 | 10 | 0.544 | 19.732 | 36.27× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | Reference full µs | vs full |
|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.302 | 0.182 | 5.49M | 61688.121 | 338529.60× |
| 100,000 | 10 | 1.059 | 0.584 | 17.12M | 61407.804 | 105153.44× |
| 100,000 | 1,000 | 13.881 | 16.225 | 61.63M | 59208.774 | 3649.30× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | Reference vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 67.15M | 74.03M | 1.00× | 2.53M | 3.29M | 1.00× | 1.61M |
| 5 | 170.49M | 213.87M | 2.89× | 2.35M | 2.46M | 0.75× | 1.69M |
| 10 | 252.89M | 262.81M | 3.55× | 2.13M | 2.32M | 0.71× | 1.63M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
