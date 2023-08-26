// https://judge.yosupo.jp/submission/157848

#include <bits/stdc++.h>

template <class T, T root> struct FFT {
    using Poly = std::vector<T>;
    constexpr static T inv_root = 1 / root;
    template <size_t pw, bool inv> static constexpr T root_pw() {
        if constexpr (pw == 0) {
            return inv ? inv_root : root;
        } else {
            constexpr T val = root_pw<pw - 1, inv>();
            return val * val;
        }
    }
    template <size_t x = 0> static constexpr size_t max_pw() {
        if constexpr (root_pw<x, false>() == 1) {
            return x;
        } else {
            return max_pw<x + 1>();
        }
    }
    template <size_t x, bool inv> static constexpr T base_2_primitive_root() {
        return root_pw<max_pw() - x, inv>();
    }

    template <size_t size_width, size_t layer, bool inv> void fft_layer(Poly &a) {
        constexpr T cur_root = base_2_primitive_root<layer + 1, inv>();
        constexpr size_t half = 1 << layer;
        for (size_t i = 0; i < a.size(); i += 2 << layer) {
            T w = 1;
            for (size_t f = 0; f < half; ++f, w *= cur_root) {
                auto &u = a[i + f], &v = a[i + f + half];
                tie(u, v) = std::pair{ u + w * v, u - w * v };
            }
        }
        if constexpr (layer + 1 < size_width) fft_layer<size_width, layer + 1, inv>(a);
    }

    template <size_t size_width, bool inv> void fft(Poly &a) {
        a.resize(1 << size_width);
        for (size_t i = 1, ri = 0; i < (1 << size_width); ++i) {
            for (size_t f = 1 << size_width; f >>= 1; ) {
                ri ^= f;
                if (ri & f) break;
            }
            if (i < ri) swap(a[i], a[ri]);
        }
        fft_layer<size_width, 0, inv>(a);
        if constexpr (!inv) return;
        T inv_sz = T(1) / T(1 << size_width);
        for (auto &i : a) i *= inv_sz;
    }

    template<size_t bit_width = 0> void conv_assign(Poly& a, const Poly& b) {
        constexpr size_t len = 1 << bit_width;
        if constexpr (bit_width >= max_pw()) {
            throw "bit_width exceed max_pw()";
        } else {
            auto max_size = max(a.size(), b.size()) * 2;
            if (max_size > len) return conv_assign<bit_width + 1>(a, b);
            size_t res_size = a.size() + b.size() - 1;
            auto tb = b;
            fft<bit_width, false>(a);
            fft<bit_width, false>(tb);
            for (size_t i = a.size(); i--;) a[i] *= tb[i];
            fft<bit_width, true>(a);
            a.resize(res_size);
        }
    }
    
    Poly conv(Poly a, const Poly& b) {
        conv_assign(a, b);
        return a;
    }
};

// FFT<mint, mint(3).pow(119)> fft;
