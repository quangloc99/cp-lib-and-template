/* #pragma GCC optimize("O3") */
/* #pragma GCC optimize("unroll-loops") */
/* #pragma GCC target("avx2,bmi,bmi2,popcnt,lzcnt") */

#include <bits/stdc++.h>
using namespace std;

// #define constexpr(...) (__VA_ARGS__)
using ll = long long;
using ld = long double;
const ld eps = 1e-8;
// for matching the kactl notes
#define sz(x) ((int)x.size())
#define rep(i, a, b) for (int i = (a); i < (b); ++i)
#define all(a) (a).begin(), (a).end()
#define print_op(...) ostream &operator<<(ostream &out, const __VA_ARGS__ &u)
// DEBUGING TEMPLETE
// ////////////////////////////////////////////////////////////////////////{{{
// What? You wanted to see how this template works?  Check this out:
// https://quangloc99.github.io/2021/07/30/my-CP-debugging-template.html
#define db(val) "[" #val " = " << (val) << "] "
#define CONCAT_(x, y) x##y
#define CONCAT(x, y) CONCAT_(x, y)
#ifdef LOCAL_DEBUG
#define clog cerr << setw(__db_level * 2) << setfill(' ') << "" << setw(0)
#define DB() debug_block CONCAT(dbbl, __LINE__)
int __db_level = 0;
struct debug_block {
    debug_block() {
        clog << "{" << endl;
        ++__db_level;
    }
    ~debug_block() {
        --__db_level;
        clog << "}" << endl;
    }
};
#else
#define clog                                                                   \
    if (0)                                                                     \
    cerr
#define DB(...)
#endif

template <class U, class V> print_op(pair<U, V>) {
    return out << "(" << u.first << ", " << u.second << ")";
}
template <size_t i, class T>
ostream &print_tuple_utils(ostream &out, const T &tup) {
    if constexpr (i == tuple_size<T>::value)
        return out << ")";
    else
        return print_tuple_utils<i + 1, T>(
            out << (i ? ", " : "(") << get<i>(tup), tup);
}
template <class... U> print_op(tuple<U...>) {
    return print_tuple_utils<0, tuple<U...>>(out, u);
}
template <class Con, class = decltype(begin(declval<Con>()))>
typename enable_if<!is_same<Con, string>::value, ostream &>::type
operator<<(ostream &out, const Con &con) {
    out << "{";
    for (auto beg = con.begin(), it = beg; it != con.end(); ++it)
        out << (it == beg ? "" : ", ") << *it;
    return out << "}";
}
// ACTUAL SOLUTION START HERE
// ////////////////////////////////////////////////////////////////}}}

mt19937_64 rng;

template <class T> T rand(T l, T r) {
    return l + (T)(rng() % (r - l + 1));
}

void gen_test(ostream &cout) {
}

using AnsT = int;

AnsT read_ans(istream &cin) {
    AnsT ans;
    cin >> ans;
    return ans;
}

void write_ans(ostream &cout, const AnsT &ans) {
    cout << ans;
}

AnsT run_main() {
    if (system("./build/main")) {
        cout << "RE main" << endl;
        exit(1);
    }
    ifstream ansf("main.out");
    auto ans = read_ans(ansf);
    return ans;
}
bool verify(const AnsT &_u) {
    return true;
}

bool compare(const AnsT &u, const AnsT &v) {
    return u == v;
}

AnsT brute() {
    return AnsT();
}

int main() {
    cin.tie(0)->sync_with_stdio(0);

    rep(testcase, 1, 10001) {
        cout << db(testcase) << endl;
        ofstream inp("main.inp");
        gen_test(inp);
        inp.close();
        
        auto ans = run_main();
        auto exp = brute();
        ofstream expf("stress.out");
        write_ans(expf, exp);
        expf.close();

        if (!verify(ans)) {
            cout << "WA verification main" << endl;
            return 1;
        }
        if (!verify(exp)) {
            cout << "WA verification brute" << endl;
            return 1;
        }
        if (!compare(ans, exp)) {
            cout << "Mismatch ans" << endl;
            return 1;
        }
        cout << "OK" << endl;
    }

    return 0;
}

