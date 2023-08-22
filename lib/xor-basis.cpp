#include <bits/stdc++.h>

template<size_t max_bit, class Num>
struct XorBasis {
    std::array<Num, max_bit> data = {};
    
    bool add_num(Num num) {
        for (size_t i = max_bit; i--; ) {
            if (!((num >> i) & 1)) continue;
            if (data[i]) num ^= data[i];
            else {
                data[i] = num;
                return true;
            }
        }
        return false;
    }
    
    Num find_max() const {
        Num ans = 0;
        for (size_t i = max_bit; i--; ) {
            ans = max(ans, ans ^ data[i]);
        }
        return ans;
    }
    
    friend XorBasis merge(XorBasis u, const XorBasis& v) {
        for (auto i = max_bit; i--; ) if (v.data[i]) u.add_num(v.data[i]);
        return u;
    }
};
