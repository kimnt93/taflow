# AccumulationDistributionOscillator benchmark (`ADOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.21M | 0.017 | 59.32M | 0.048 | 2.49× | 2.83× |
| 10,000 | 0.092 | 108.13M | 0.149 | 67.17M | 0.092 | 1.00× | 0.62× |
| 100,000 | 1.253 | 79.78M | 1.125 | 88.89M | 0.365 | 0.29× | 0.32× |
| 1,000,000 | 10.310 | 96.99M | 9.775 | 102.30M | 3.923 | 0.38× | 0.40× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.125 | 1.59× |
| 1 | 5 | 0.303 | 0.606 | 2.00× |
| 1 | 10 | 0.676 | 1.303 | 1.93× |
| 10 | 1 | 0.095 | 0.139 | 1.46× |
| 10 | 5 | 0.427 | 0.697 | 1.63× |
| 10 | 10 | 0.671 | 1.385 | 2.06× |
| 100 | 1 | 0.075 | 0.112 | 1.48× |
| 100 | 5 | 0.370 | 0.628 | 1.70× |
| 100 | 10 | 0.723 | 1.325 | 1.83× |
| 1,000 | 1 | 0.074 | 0.122 | 1.64× |
| 1,000 | 5 | 0.384 | 0.657 | 1.71× |
| 1,000 | 10 | 0.947 | 1.417 | 1.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
