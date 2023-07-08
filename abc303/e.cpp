#include "bits/stdc++.h"
using namespace std;
using ll = int64_t;
using ld = long double;
using ull = unsigned long long;
#define ALL(x) x.begin(), x.end()
#define rep(iter, from, to) for (ll iter = from; iter < to; ++iter)
#define fore(variable, container) for (auto &variable : container)
#define forc(variable, container)    \
    for (auto &variable : container) \
        cout << variable << endl;
const ll MOD = 1e9 + 7;
const ll INF = 1e17;
const vector<ll> dx{1, 0, -1, 0}, dy{0, 1, 0, -1};
// #######################################################################
void op(vector<ll> vec)
{
    ll size = (ll)vec.size();
    for (ll i = 0; i < size - 1; ++i)
        cout << vec[i] << " ";
    cout << vec.back() << endl;
}

void op(vector<vector<ll>> vec)
{
    ll height = (ll)vec.size();
    ll width = (ll)vec[0].size();
    for (ll i = 0; i < height; ++i)
    {
        for (ll j = 0; j < width - 1; ++j)
            cout << vec[i][j] << " ";
        cout << vec[i].back() << endl;
    }
}

void twoText(bool identifier, string outTrue, string outFalse)
{
    if (identifier)
        cout << outTrue << endl;
    else
        cout << outFalse << endl;
}

void twoText(bool identifier)
{
    if (identifier)
        cout << "Yes" << endl;
    else
        cout << "No" << endl;
}

template <class T>
T vecsum(vector<T> &vec)
{
    return accumulate(ALL(vec), (T)0);
}

template <class T, ll>
T vecsum(vector<T> &vec, ll K)
{
    ll ret = 0;
    rep(i, 0, K) ret += vec[i];
    return ret;
}

template <class T>
struct grid
{
    vector<vector<T>> field;
    grid(ll height, ll width) { field = vector<vector<T>>(height, vector<T>(width, (T)0)); }
    void input() { rep(i, 0, field.size()) rep(j, 0, field[i].size()) cin >> field[i][j]; }
};

// #########################################################################
struct UnionFind
{
    vector<ll> parents;
    UnionFind(int size)
    {
        parents.assign(size, -1);
    }
    ll findRoot(ll x)
    {
        if (parents[x] < 0)
            return x;
        return parents[x] = findRoot(parents[x]);
    }
    bool unite(ll x, ll y)
    {
        x = findRoot(x);
        y = findRoot(y);
        if (x == y)
            return false;
        if (parents[x] > parents[y])
            swap(x, y);
        parents[x] += parents[y];
        parents[y] = x;
        return true;
    }
    ll size(ll x) { return -parents[findRoot(x)]; }
    bool isSameGroup(ll x, ll y) { return findRoot(x) == findRoot(y); }
};

struct Lowlink
{
    ll size;
    vector<bool> used;
    vector<ll> ord, low;
    vector<pair<ll, ll>> brides;
    vector<vector<ll>> g;
    Lowlink() {}
    Lowlink(vector<vector<ll>> &g_) : size(g_.size()), g(g_)
    {
        ord.assign(size, 0);
        low.assign(size, 0);
        used.assign(size, 0);
        ll k = 0;
        rep(i, 0, size)
        {
            if (!used[i])
                k = dfs(i, k, -1);
        }
    }
    ll dfs(ll now, ll k, ll par)
    {
        used[now] = true;
        ord[now] = k++;
        low[now] = ord[now];
        ll child = 0;
        for (auto &to : g[now])
        {
            if (!used[to])
            {
                child++;
                k = dfs(to, k, now);
                low[now] = min(low[now], low[to]);
                if (ord[now] < low[to])
                    brides.emplace_back(min(now, to), max(now, to));
            }
            else if (to != par)
            {
                low[now] = min(low[now], ord[to]);
            }
        }
        return k;
    }
};

void solve()
{
    ll N;
    cin >> N;
    ll M = N - 1;
    vector<pair<ll, ll>> edges(M);
    vector<vector<ll>> g(N);
    for (auto [u, v] : edges)
    {
        cin >> u >> v;
        u--;
        v--;
        g[u].emplace_back(v);
        g[v].emplace_back(u);
    }
    Lowlink lw(g);
    auto bridge = lw.brides;
    set<pair<ll, ll>> brdic{};
    fore(x, bridge)
    {
        brdic.emplace(x);
    }
    UnionFind uf(N);
    cout << "bridge:" << bridge.size() << endl;
    fore(x, bridge)
    {
        cout << x.first << " " << x.second << endl;
    }
    for (auto [u, v] : edges)
    {
        ll uu = min(u, v);
        ll vv = max(u, v);
        if (brdic.find(make_pair(uu, vv)) != brdic.end())
        {
            continue;
        }
        uf.unite(uu, vv);
    }
    set<ll> ans;
    rep(i, 0, N)
    {
        ans.insert(uf.size(i));
        cout << i << ":" << uf.size(i) << endl;
    }
    forc(x, ans);
}

int main(void)
{
    std::cin.tie(nullptr);
    std::ios_base::sync_with_stdio(false);
    std::cout << std::fixed << std::setprecision(15);
    solve();
}
