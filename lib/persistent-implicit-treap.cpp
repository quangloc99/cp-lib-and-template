#include <bits/stdc++.h>

struct Treap {
    static std::mt19937 rng;
    static int rand(int n) {
        return (int)(rng() % n);
    }
    
    const Treap *l, *r;
    int val;
    int size;

    friend int get_size(const Treap *t) {
        return t ? t->size : 0;
    }

    friend int upd_size(Treap *t) {
        if (!t) return 0;
        return t->size = get_size(t->l) + get_size(t->r) + 1;
    }

    Treap(int v, const Treap *l_, const Treap *r_)
        : l(l_), r(r_), val(v), size(1 + get_size(l) + get_size(r)) {
    }

    // split [-inf, pos) and [pos, +inf)
    friend std::pair<const Treap *, const Treap *> split(const Treap *t, int pos) {
        if (!t) return { 0, 0 };
        if (get_size(t) <= pos) return { t, nullptr };
        if (pos == 0) return { nullptr, t };
        if (pos > get_size(t->l)) {
            auto [split_l, split_r] = split(t->r, pos - get_size(t->l) - 1);
            return { new Treap(t->val, t->l, split_l), split_r };
        }
        auto [split_l, split_r] = split(t->l, pos);
        return { split_l, new Treap(t->val, split_r, t->r) };
    }

    friend const Treap *join(const Treap *l, const Treap *r) {
        if (!l and !r) return nullptr;
        if (!l) return r;
        if (!r) return l;
        if (rand(get_size(l) + get_size(r)) < get_size(l)) {
            return new Treap(l->val, l->l, join(l->r, r));
        }
        return new Treap(r->val, join(l, r->l), r->r);
    }

    friend int get(const Treap *t, int pos) {
        if (pos == get_size(t->l)) return t->val;
        if (pos < get_size(t->l)) return get(t->l, pos);
        return get(t->r, pos - 1 - get_size(t->l));
    }
};

const Treap *build(std::span<int> nums) {
    if (nums.empty()) return nullptr;
    int mid = (int)(nums.size()) / 2;
    return new Treap(nums[mid], build(nums.subspan(0, mid)), build(nums.subspan(mid + 1)));
}
