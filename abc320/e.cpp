#include <algorithm>
#include <array>
#include <bitset>
#include <cstdlib>
#include <ctime>
#include <functional>
#include <iomanip>
#include <iostream>
#include <limits>
#include <map>
#include <numeric>
#include <queue>
#include <random>
#include <set>
#include <stack>
#include <string>
#include <type_traits>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>
using namespace std;
using ll = int64_t;
using ld = long double;
using ull = unsigned long long;
#define ALL(x) x.begin(), x.end()
#define rep(iter, from, to) for (ll iter = from; iter < to; ++iter)
#define fore(variable, container) for (auto variable : container)
#define forc(variable, container)   \
    for (auto variable : container) \
        cout << variable << endl;

const ll MOD = 1e9 + 7;
const ll INF = 1e17;
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

void counter(ll &num, ll &increaser, bool checker)
{
    if (checker)
        num += increaser;
}

template <class T>
struct grid
{
    vector<vector<T>> field;
    grid(ll height, ll width)
    {
        field = vector<vector<T>>(height, vector<T>(width, (T)0));
    }
    void input()
    {
        rep(i, 0, field.size()) rep(j, 0, field[i].size()) cin >> field[i][j];
    }
};

template <class T>
T vecsum(vector<T> &vec)
{
    return accumulate(ALL(vec), (T)0);
}
// #########################################################################

void solve()
{
    ll N, M;
    cin >> N >> M;
    set<tuple<ll, ll, ll, ll>> timetable;
    vector<tuple<ll, ll, ll>> somen(M);
    rep(i, 0, M)
    {
        ll time, amount, back;
        cin >> time >> amount >> back;
        somen[i] = make_tuple(time, amount, back);
    }
    for (auto &goods : somen)
    {
        ll time, amount, back;
        tie(time, amount, back) = goods;
        }
}

int main(void)
{
    std::cin.tie(nullptr);
    std::ios_base::sync_with_stdio(false);
    std::cout << std::fixed << std::setprecision(15);
    solve();
}
