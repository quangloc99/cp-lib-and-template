#include <bits/stdc++.h>
template <class Num> struct Binom {
    std::vector<Num> fac, ifac;

    Binom(int size) : fac(size), ifac(size) {
        fac[0] = 1;
        for (int i = 1; i < size; ++i)
            fac[i] = fac[i - 1] * i;
        ifac[size - 1] = 1 / fac[size - 1];
        for (int i = size - 2; i >= 0; --i)
            ifac[i] = ifac[i + 1] * (i + 1);
    }
    
    Num operator()(int n, int k) const {
        if (k < 0 or k > n) return Num();
        return fac[n] * ifac[k] * ifac[n - k];
    }
    
    Num partition(int sum, int parts) const {
        return operator()(sum + parts - 1, parts - 1);
    }
};
