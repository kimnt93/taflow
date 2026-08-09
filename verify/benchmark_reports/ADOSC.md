# AccumulationDistributionOscillator benchmark (`ADOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 70.40M | 0.012 | 81.66M | 0.038 | 2.67× | 3.09× |
| 10,000 | 0.086 | 116.54M | 0.081 | 123.91M | 0.060 | 0.70× | 0.74× |
| 100,000 | 0.811 | 123.25M | 0.762 | 131.32M | 0.278 | 0.34× | 0.37× |
| 1,000,000 | 8.579 | 116.57M | 7.940 | 125.95M | 2.929 | 0.34× | 0.37× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.165 | 2.04× |
| 1 | 5 | 0.370 | 0.583 | 1.58× |
| 1 | 10 | 0.500 | 1.019 | 2.04× |
| 10 | 1 | 0.053 | 0.100 | 1.90× |
| 10 | 5 | 0.230 | 0.460 | 2.00× |
| 10 | 10 | 0.503 | 0.975 | 1.94× |
| 100 | 1 | 0.056 | 0.099 | 1.78× |
| 100 | 5 | 0.237 | 0.471 | 1.98× |
| 100 | 10 | 0.507 | 0.958 | 1.89× |
| 1,000 | 1 | 0.061 | 0.103 | 1.68× |
| 1,000 | 5 | 0.244 | 0.466 | 1.91× |
| 1,000 | 10 | 0.517 | 0.982 | 1.90× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.429 | 0.265 | 3.77M | 296.938 | 1120.60× | 119.17× |
| 100,000 | 10 | 2.431 | 1.173 | 8.52M | 270.055 | 230.21× | 26.82× |
| 100,000 | 1,000 | 11.771 | 9.516 | 105.09M | 272.222 | 28.61× | 3.71× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 103.57M | 105.05M | 1.00× | 2.01M | 2.38M | 1.00× | 216.51M |
| 5 | 141.99M | 160.45M | 1.53× | 1.78M | 2.08M | 0.87× | 227.29M |
| 10 | 236.30M | 235.82M | 2.24× | 1.59M | 2.13M | 0.90× | 230.79M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
