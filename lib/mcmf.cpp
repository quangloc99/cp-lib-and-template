/* https://codeforces.com/contest/1187/submission/219651952 */
#include <bits/stdc++.h>

template <class Cap = long long, class Cost = long long,
          Cost inf = std::numeric_limits<Cost>::max() / 4>
struct MCMF {
    struct Edge {
        int to;
        Cap cap;
        Cost cost;
        using ptr = typename std::list<Edge>::iterator;
        ptr inv;
    };

    int n;
    std::vector<std::list<Edge>> gr;

    // for inc_flow, but should be preallocated
    std::vector<Cost> potential;
    std::vector<typename Edge::ptr> trace;

    MCMF(int n_) : n(n_), gr(n), potential(n), trace(n) {
    }

    void add_edge(int from, int to, Cap cap, Cost cost) {
        gr[from].push_back({ to, cap, cost });
        gr[to].push_back({ from, 0, -cost });
        gr[from].back().inv = --gr[to].end();
        gr[to].back().inv = --gr[from].end();
    }

    template <class T> using min_pq = std::priority_queue<T, std::vector<T>, std::greater<T>>;

    std::pair<Cap, Cost> inc_flow(int src, int dst, Cap flow = inf) {
        min_pq<std::pair<Cost, int>> prio;
        static std::vector<Cost> dist;
        dist.assign(n, inf);
        dist[src] = 0;
        prio.emplace(0, src);

        while (prio.size()) {
            auto [du, u] = prio.top();
            prio.pop();
            if (du > dist[u]) continue;
            for (auto it = gr[u].begin(); it != gr[u].end(); ++it) {
                const auto &[v, cap, cost, _inv] = *it;
                if (cap == 0) continue;
                auto nd = du + (cost + potential[u] - potential[v]);
                if (nd >= dist[v]) continue;
                dist[v] = nd;
                prio.emplace(nd, v);
                trace[v] = it;
            }
        }

        if (dist[dst] >= inf) return { 0, 0 };
        for (int i = n; i--;) dist[i] += potential[i] - potential[src];
        swap(dist, potential);

        for (int u = dst; u != src; u = trace[u]->inv->to) {
            flow = std::min(flow, trace[u]->cap);
        }

        Cost inc_cost = 0;
        for (int u = dst; u != src; u = trace[u]->inv->to) {
            trace[u]->cap -= flow;
            trace[u]->inv->cap += flow;
            inc_cost += trace[u]->cost;
        }
        inc_cost *= flow;

        return { flow, inc_cost };
    }

    std::pair<Cap, Cost> max_flow(int src, int dst) {
        Cost min_cost = 0;
        Cap max_flow = 0;

        while (true) {
            auto [cur_flow, cur_cost] = inc_flow(src, dst);
            if (!cur_flow) break;
            min_cost += cur_cost;
            max_flow += cur_flow;
        }

        return { max_flow, min_cost };
    }
};
